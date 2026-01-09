use core::time::Duration;
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

fn set_socket_write_timeout(tcp: &TcpStream, timeout_at: Option<Instant>) -> io::Result<()> {
    tcp.set_write_timeout(timeout_at_to_duration(timeout_at)?)?;
    Ok(())
}

impl Write for HttpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = match self {
            HttpStream::Unsecured(inner, timeout_at) => {
                set_socket_write_timeout(inner, *timeout_at)?;
                inner.write(buf)
            }
            #[cfg(feature = "rustls")]
            HttpStream::Secured(inner, timeout_at) => {
                set_socket_write_timeout(inner.get_ref(), *timeout_at)?;
                inner.write(buf)
            }
            #[cfg(feature = "async")]
            HttpStream::Buffer(_) => {
                debug_assert!(false, "We shouldn't write to a pre-loaded stream");
                Ok(buf.len())
            }
        };
        match result {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // We're a blocking socket, so EWOULDBLOCK indicates a timeout
                Err(timeout_err())
            }
            r => r,
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let result = match self {
            HttpStream::Unsecured(inner, timeout_at) => {
                set_socket_write_timeout(inner, *timeout_at)?;
                inner.flush()
            }
            #[cfg(feature = "rustls")]
            HttpStream::Secured(inner, timeout_at) => {
                set_socket_write_timeout(inner.get_ref(), *timeout_at)?;
                inner.flush()
            }
            #[cfg(feature = "async")]
            HttpStream::Buffer(_) => {
                debug_assert!(false, "We shouldn't write to a pre-loaded stream");
                Ok(())
            }
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
    stream: HttpStream,
}

impl Connection {
    /// Creates a new `Connection`. See [Request] and [ParsedRequest]
    /// for specifics about *what* is being sent.
    pub(crate) fn new(
        params: ConnectionParams<'_>,
        timeout_at: Option<Instant>,
    ) -> Result<Connection, Error> {
        let socket = Self::connect(params, timeout_at)?;

        let stream = if params.https {
            #[cfg(not(feature = "rustls"))]
            return Err(Error::HttpsFeatureNotEnabled);
            #[cfg(feature = "rustls")]
            {
                let tls = rustls_stream::wrap_stream(socket, params.host)?;
                HttpStream::Secured(Box::new(tls), timeout_at)
            }
        } else {
            HttpStream::create_unsecured(socket, timeout_at)
        };

        Ok(Connection { stream })
    }

    fn tcp_connect(host: &str, port: u16, timeout_at: Option<Instant>) -> Result<TcpStream, Error> {
        #[cfg(feature = "log")]
        log::trace!("Looking up host {host}");

        let addrs = (host, port).to_socket_addrs().map_err(Error::IoError)?;
        let addrs_count = addrs.len();

        // Try all resolved addresses. Return the first one to which we could connect. If all
        // failed return the last error encountered.
        for (i, addr) in addrs.enumerate() {
            #[cfg(feature = "log")]
            log::trace!("Attempting to connect to {addr} for {host}");

            let stream = if let Some(timeout) = timeout_at_to_duration(timeout_at)? {
                TcpStream::connect_timeout(&addr, timeout)
            } else {
                TcpStream::connect(addr)
            };

            match stream {
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

    /// Connect to the server.
    fn connect(
        params: ConnectionParams<'_>,
        timeout_at: Option<Instant>,
    ) -> Result<TcpStream, Error> {
        #[cfg(feature = "proxy")]
        match &params.proxy {
            Some(proxy) => {
                // do proxy things
                let mut tcp = Self::tcp_connect(&proxy.server, proxy.port, timeout_at)?;

                write!(tcp, "{}", proxy.connect(params.host, params.port.port()))?;
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
            None => Self::tcp_connect(params.host, params.port.port(), timeout_at),
        }

        #[cfg(not(feature = "proxy"))]
        Self::tcp_connect(params.host, params.port.port(), timeout_at)
    }

    /// Sends the [`Request`](struct.Request.html), consumes this
    /// connection, and returns a [`Response`](struct.Response.html).
    pub(crate) fn send(mut self, request: ParsedRequest) -> Result<ResponseLazy, Error> {
        enforce_timeout(request.timeout_at, move || {
            // Send request
            #[cfg(feature = "log")]
            log::trace!("Writing HTTP request.");
            self.stream.write_all(&request.as_bytes())?;

            // Receive response
            #[cfg(feature = "log")]
            log::trace!("Reading HTTP response.");
            let response = ResponseLazy::from_stream(
                self.stream,
                request.config.max_headers_size,
                request.config.max_status_line_len,
            )?;
            handle_redirects(request, response)
        })
    }
}

fn handle_redirects(
    request: ParsedRequest,
    mut response: ResponseLazy,
) -> Result<ResponseLazy, Error> {
    let status_code = response.status_code;
    let url = response.headers.get("location");
    match get_redirect(request, status_code, url) {
        NextHop::Redirect(request) => {
            let request = request?;
            Connection::new(request.connection_params(), request.timeout_at)?.send(request)
        }
        NextHop::Destination(request) => {
            let dst_url = request.url;
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
