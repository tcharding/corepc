use core::time::Duration;
use std::env;
#[cfg(feature = "async")]
use std::future::Future;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
#[cfg(feature = "async")]
use std::pin::Pin;
#[cfg(feature = "async")]
use std::task::{Context, Poll};
use std::time::Instant;

#[cfg(all(feature = "async", feature = "proxy"))]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async")]
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
#[cfg(feature = "async")]
use tokio::net::TcpStream as AsyncTcpStream;

use crate::request::{ConnectionParams, ParsedRequest};
#[cfg(feature = "async")]
use crate::Response;
use crate::{Error, Method, ResponseLazy};

type UnsecuredStream = TcpStream;

#[cfg(feature = "rustls")]
mod rustls_stream;
#[cfg(feature = "rustls")]
type SecuredStream = rustls_stream::SecuredStream;

pub(crate) enum HttpStream {
    Unsecured(UnsecuredStream, Option<Instant>),
    #[cfg(feature = "rustls")]
    Secured(Box<SecuredStream>, Option<Instant>),
    #[cfg(feature = "async")]
    Buffer(std::io::Cursor<Vec<u8>>),
}

impl HttpStream {
    fn create_unsecured(reader: UnsecuredStream, timeout_at: Option<Instant>) -> HttpStream {
        HttpStream::Unsecured(reader, timeout_at)
    }

    #[cfg(feature = "rustls")]
    fn create_secured(reader: SecuredStream, timeout_at: Option<Instant>) -> HttpStream {
        HttpStream::Secured(Box::new(reader), timeout_at)
    }

    #[cfg(feature = "async")]
    pub(crate) fn create_buffer(buffer: Vec<u8>) -> HttpStream {
        HttpStream::Buffer(std::io::Cursor::new(buffer))
    }
}

fn timeout_err() -> io::Error {
    io::Error::new(io::ErrorKind::TimedOut, "the timeout of the request was reached")
}

fn timeout_at_to_duration(timeout_at: Option<Instant>) -> Result<Option<Duration>, io::Error> {
    if let Some(timeout_at) = timeout_at {
        if let Some(duration) = timeout_at.checked_duration_since(Instant::now()) {
            Ok(Some(duration))
        } else {
            Err(timeout_err())
        }
    } else {
        Ok(None)
    }
}

impl Read for HttpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let timeout = |tcp: &TcpStream, timeout_at: Option<Instant>| -> io::Result<()> {
            let _ = tcp.set_read_timeout(timeout_at_to_duration(timeout_at)?);
            Ok(())
        };

        let result = match self {
            HttpStream::Unsecured(inner, timeout_at) => {
                timeout(inner, *timeout_at)?;
                inner.read(buf)
            }
            #[cfg(feature = "rustls")]
            HttpStream::Secured(inner, timeout_at) => {
                timeout(inner.get_ref(), *timeout_at)?;
                inner.read(buf)
            }
            #[cfg(feature = "async")]
            HttpStream::Buffer(cursor) => std::io::Read::read(cursor, buf),
        };
        match result {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // We're a blocking socket, so EWOULDBLOCK indicates a timeout
                Err(timeout_err())
            }
            r => r,
        }
    }
}

#[cfg(feature = "async-https")]
type AsyncSecuredStream = rustls_stream::AsyncSecuredStream;

#[cfg(feature = "async")]
pub(crate) enum AsyncHttpStream {
    Unsecured(AsyncTcpStream),
    #[cfg(feature = "async-https")]
    Secured(Box<AsyncSecuredStream>),
}

#[cfg(feature = "async")]
impl AsyncRead for AsyncHttpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match &mut *self {
            AsyncHttpStream::Unsecured(inner) => Pin::new(inner).poll_read(cx, buf),
            #[cfg(feature = "async-https")]
            AsyncHttpStream::Secured(inner) => Pin::new(inner).poll_read(cx, buf),
        }
    }
}

#[cfg(feature = "async")]
impl AsyncWrite for AsyncHttpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            AsyncHttpStream::Unsecured(inner) => Pin::new(inner).poll_write(cx, buf),
            #[cfg(feature = "async-https")]
            AsyncHttpStream::Secured(inner) => Pin::new(inner).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            AsyncHttpStream::Unsecured(inner) => Pin::new(inner).poll_flush(cx),
            #[cfg(feature = "async-https")]
            AsyncHttpStream::Secured(inner) => Pin::new(inner).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            AsyncHttpStream::Unsecured(inner) => Pin::new(inner).poll_shutdown(cx),
            #[cfg(feature = "async-https")]
            AsyncHttpStream::Secured(inner) => Pin::new(inner).poll_shutdown(cx),
        }
    }
}

/// An async connection to the server for sending
/// [`Request`](struct.Request.html)s.
#[cfg(feature = "async")]
pub struct AsyncConnection {
    stream: AsyncHttpStream,
}

#[cfg(feature = "async")]
impl AsyncConnection {
    /// Creates a new `AsyncConnection`.
    pub(crate) async fn new(
        params: ConnectionParams<'_>,
        timeout_at: Option<Instant>,
    ) -> Result<AsyncConnection, Error> {
        let future = async move {
            let socket = Self::connect(params).await?;

            if params.https {
                #[cfg(not(feature = "async-https"))]
                return Err(Error::HttpsFeatureNotEnabled);
                #[cfg(feature = "async-https")]
                rustls_stream::wrap_async_stream(socket, params.host).await
            } else {
                Ok(AsyncHttpStream::Unsecured(socket))
            }
        };
        let stream = if let Some(timeout_at) = timeout_at {
            tokio::time::timeout_at(timeout_at.into(), future)
                .await
                .unwrap_or(Err(Error::IoError(timeout_err())))?
        } else {
            future.await?
        };

        Ok(AsyncConnection { stream })
    }

    async fn tcp_connect(host: &str, port: u16) -> Result<AsyncTcpStream, Error> {
        #[cfg(feature = "log")]
        log::trace!("Looking up host {host}");

        let addrs = tokio::net::lookup_host((host, port)).await.map_err(Error::IoError)?;
        let addrs: Vec<_> = addrs.collect();
        let addrs_count = addrs.len();

        if addrs.is_empty() {
            return Err(Error::AddressNotFound);
        }

        // Try all resolved addresses. Return the first one to which we could connect. If all
        // failed return the last error encountered.
        for (i, addr) in addrs.iter().enumerate() {
            #[cfg(feature = "log")]
            log::trace!("Attempting to connect to {addr} for {host}");

            match AsyncTcpStream::connect(addr).await {
                Ok(s) => {
                    #[cfg(feature = "log")]
                    log::trace!("Connected to {addr} for {host}");
                    return Ok(s);
                }
                Err(e) =>
                    if i == addrs_count - 1 {
                        return Err(Error::IoError(e));
                    },
            }
        }

        Err(Error::AddressNotFound)
    }

    /// Asynchronously connect to the server.
    async fn connect(params: ConnectionParams<'_>) -> Result<AsyncTcpStream, Error> {
        #[cfg(feature = "proxy")]
        match &params.proxy {
            Some(proxy) => {
                // do proxy things
                let mut tcp = Self::tcp_connect(&proxy.server, proxy.port).await?;

                let proxy_request = proxy.connect(params.host, params.port.port());
                tcp.write_all(proxy_request.as_bytes()).await?;
                tcp.flush().await?;

                let mut proxy_response = Vec::new();
                let mut buf = vec![0; 256];
                loop {
                    let n = tcp.read(&mut buf).await?;
                    proxy_response.extend_from_slice(&buf[..n]);
                    if n < 256 {
                        break;
                    }
                }

                crate::Proxy::verify_response(&proxy_response)?;

                Ok(tcp)
            }
            None => Self::tcp_connect(params.host, params.port.port()).await,
        }

        #[cfg(not(feature = "proxy"))]
        Self::tcp_connect(&params.host, params.port.port()).await
    }

    /// Sends the [`Request`](struct.Request.html) asynchronously using HTTP.
    pub(crate) async fn send(mut self, request: ParsedRequest) -> Result<Response, Error> {
        let timeout = request.timeout_at;
        let future = async move {
            // Send request
            #[cfg(feature = "log")]
            log::trace!("Writing HTTP request.");
            self.stream.write_all(&request.as_bytes()).await?;

            // Receive response
            #[cfg(feature = "log")]
            log::trace!("Reading HTTP response.");
            let response = Response::create_async(
                self.stream,
                request.config.method == Method::Head,
                request.config.max_headers_size,
                request.config.max_status_line_len,
            )
            .await?;
            async_handle_redirects(request, response).await
        };
        if let Some(timeout_at) = timeout {
            tokio::time::timeout_at(timeout_at.into(), future)
                .await
                .unwrap_or(Err(Error::IoError(timeout_err())))
        } else {
            future.await
        }
    }
}

/// A connection to the server for sending
/// [`Request`](struct.Request.html)s.
pub struct Connection {
    request: ParsedRequest,
    timeout_at: Option<Instant>,
}

impl Connection {
    /// Creates a new `Connection`. See [Request] and [ParsedRequest]
    /// for specifics about *what* is being sent.
    pub(crate) fn new(request: ParsedRequest) -> Connection {
        let timeout = request.config.timeout.or_else(|| match env::var("BITREQ_TIMEOUT") {
            Ok(t) => t.parse::<u64>().ok(),
            Err(_) => None,
        });
        let timeout_at = timeout.map(|t| Instant::now() + Duration::from_secs(t));
        Connection { request, timeout_at }
    }

    /// Returns the timeout duration for operations that should end at
    /// timeout and are starting "now".
    ///
    /// The Result will be Err if the timeout has already passed.
    fn timeout(&self) -> Result<Option<Duration>, io::Error> {
        let timeout = timeout_at_to_duration(self.timeout_at);
        #[cfg(feature = "log")]
        log::trace!("Timeout requested, it is currently: {:?}", timeout);
        timeout
    }

    /// Sends the [`Request`](struct.Request.html), consumes this
    /// connection, and returns a [`Response`](struct.Response.html).
    #[cfg(feature = "rustls")]
    pub(crate) fn send_https(self) -> Result<ResponseLazy, Error> {
        enforce_timeout(self.timeout_at, move || {
            let secured_stream = rustls_stream::create_secured_stream(&self)?;

            #[cfg(feature = "log")]
            log::trace!("Reading HTTPS response from {}.", self.request.url.host);
            let response = ResponseLazy::from_stream(
                secured_stream,
                self.request.config.max_headers_size,
                self.request.config.max_status_line_len,
            )?;

            handle_redirects(self, response)
        })
    }

    /// Sends the [`Request`](struct.Request.html), consumes this
    /// connection, and returns a [`Response`](struct.Response.html).
    pub(crate) fn send(self) -> Result<ResponseLazy, Error> {
        enforce_timeout(self.timeout_at, move || {
            let bytes = self.request.as_bytes();

            #[cfg(feature = "log")]
            log::trace!("Establishing TCP connection to {}.", self.request.url.host);
            let mut tcp = self.connect()?;

            // Send request
            #[cfg(feature = "log")]
            log::trace!("Writing HTTP request.");
            let _ = tcp.set_write_timeout(self.timeout()?);
            tcp.write_all(&bytes)?;

            // Receive response
            #[cfg(feature = "log")]
            log::trace!("Reading HTTP response.");
            let stream = HttpStream::create_unsecured(tcp, self.timeout_at);
            let response = ResponseLazy::from_stream(
                stream,
                self.request.config.max_headers_size,
                self.request.config.max_status_line_len,
            )?;
            handle_redirects(self, response)
        })
    }

    fn connect(&self) -> Result<TcpStream, Error> {
        let tcp_connect = |host: &str, port: u16| -> Result<TcpStream, Error> {
            let addrs = (host, port).to_socket_addrs().map_err(Error::IoError)?;
            let addrs_count = addrs.len();

            // Try all resolved addresses. Return the first one to which we could connect. If all
            // failed return the last error encountered.
            for (i, addr) in addrs.enumerate() {
                let stream = if let Some(timeout) = self.timeout()? {
                    TcpStream::connect_timeout(&addr, timeout)
                } else {
                    TcpStream::connect(addr)
                };
                if stream.is_ok() || i == addrs_count - 1 {
                    return stream.map_err(Error::from);
                }
            }

            Err(Error::AddressNotFound)
        };

        #[cfg(feature = "proxy")]
        match self.request.config.proxy {
            Some(ref proxy) => {
                // do proxy things
                let mut tcp = tcp_connect(&proxy.server, proxy.port)?;

                write!(tcp, "{}", proxy.connect(&self.request.url.host, self.request.url.port.port())).unwrap();
                tcp.flush()?;

                let mut proxy_response = Vec::new();

                loop {
                    let mut buf = vec![0; 256];
                    let total = tcp.read(&mut buf)?;
                    proxy_response.append(&mut buf);
                    if total < 256 {
                        break;
                    }
                }

                crate::Proxy::verify_response(&proxy_response)?;

                Ok(tcp)
            }
            None => tcp_connect(&self.request.url.host, self.request.url.port.port()),
        }

        #[cfg(not(feature = "proxy"))]
        tcp_connect(&self.request.url.host, self.request.url.port.port())
    }
}

fn handle_redirects(
    mut connection: Connection,
    mut response: ResponseLazy,
) -> Result<ResponseLazy, Error> {
    let status_code = response.status_code;
    let url = response.headers.get("location");
    match get_redirect(connection.request, status_code, url) {
        NextHop::Redirect(request) => {
            let request = request?;
            connection.request = request;
            if connection.request.url.https {
                #[cfg(not(feature = "rustls"))]
                return Err(Error::HttpsFeatureNotEnabled);
                #[cfg(feature = "rustls")]
                return connection.send_https();
            } else {
                connection.send()
            }
        }
        NextHop::Destination(request) => {
            connection.request = request;
            let dst_url = connection.request.url;
            dst_url.write_base_url_to(&mut response.url).unwrap();
            dst_url.write_resource_to(&mut response.url).unwrap();
            Ok(response)
        }
    }
}

#[cfg(feature = "async")]
fn async_handle_redirects(
    request: ParsedRequest,
    mut response: Response,
) -> Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>> {
    Box::pin(async move {
        let status_code = response.status_code;
        let url = response.headers.get("location");
        match async_get_redirect(request, status_code, url) {
            NextHopAsync::Redirect(request) => {
                let request = request?;
                let connection =
                    AsyncConnection::new(request.connection_params(), request.timeout_at).await?;
                connection.send(request).await
            }
            NextHopAsync::Destination(request) => {
                let dst_url = request.url;
                dst_url.write_base_url_to(&mut response.url).unwrap();
                dst_url.write_resource_to(&mut response.url).unwrap();
                Ok(response)
            }
        }
    })
}

macro_rules! redirect_utils {
    ($get_redirect: ident, $NextHop: ident, $Response: ident) => {
        enum $NextHop {
            Redirect(Result<ParsedRequest, Error>),
            Destination(ParsedRequest),
        }

        fn $get_redirect(
            mut request: ParsedRequest,
            status_code: i32,
            url: Option<&String>,
        ) -> $NextHop {
            match status_code {
                301 | 302 | 303 | 307 => {
                    let url = match url {
                        Some(url) => url,
                        None => return $NextHop::Redirect(Err(Error::RedirectLocationMissing)),
                    };
                    #[cfg(feature = "log")]
                    log::debug!("Redirecting ({}) to: {}", status_code, url);

                    match request.redirect_to(url.as_str()) {
                        Ok(()) => {
                            if status_code == 303 {
                                match request.config.method {
                                    Method::Post | Method::Put | Method::Delete => {
                                        request.config.method = Method::Get;
                                    }
                                    _ => {}
                                }
                            }

                            $NextHop::Redirect(Ok(request))
                        }
                        Err(err) => $NextHop::Redirect(Err(err)),
                    }
                }
                _ => $NextHop::Destination(request),
            }
        }
    };
}

redirect_utils!(get_redirect, NextHop, ResponseLazy);
#[cfg(feature = "async")]
redirect_utils!(async_get_redirect, NextHopAsync, Response);

/// Enforce the timeout by running the function in a new thread and
/// parking the current one with a timeout.
///
/// While bitreq does use timeouts (somewhat) properly, some
/// interfaces such as [ToSocketAddrs] don't allow for specifying the
/// timeout. Hence this.
fn enforce_timeout<F, R>(timeout_at: Option<Instant>, f: F) -> Result<R, Error>
where
    F: 'static + Send + FnOnce() -> Result<R, Error>,
    R: 'static + Send,
{
    use std::sync::mpsc::{channel, RecvTimeoutError};

    match timeout_at {
        Some(deadline) => {
            let (sender, receiver) = channel();
            let thread = std::thread::spawn(move || {
                let result = f();
                let _ = sender.send(());
                result
            });
            if let Some(timeout_duration) = deadline.checked_duration_since(Instant::now()) {
                match receiver.recv_timeout(timeout_duration) {
                    Ok(()) => thread.join().unwrap(),
                    Err(err) => match err {
                        RecvTimeoutError::Timeout => Err(Error::IoError(timeout_err())),
                        RecvTimeoutError::Disconnected =>
                            Err(Error::Other("request connection paniced")),
                    },
                }
            } else {
                Err(Error::IoError(timeout_err()))
            }
        }
        None => f(),
    }
}
