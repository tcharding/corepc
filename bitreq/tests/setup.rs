#![cfg(feature = "std")]

extern crate bitreq;
extern crate tiny_http;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Once};
use std::thread;
use std::time::Duration;

use self::tiny_http::{Header, Method, Response, Server, StatusCode};

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        let server = Arc::new(Server::http("localhost:35562").unwrap());
        for _ in 0..8 {
            let server = server.clone();

            thread::spawn(move || loop {
                let mut request = {
                    if let Ok(request) = server.recv() {
                        request
                    } else {
                        continue; // If .recv() fails, just try again.
                    }
                };
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content).ok();
                let headers = Vec::from(request.headers());

                let url = String::from(request.url().split('#').next().unwrap());
                match request.method() {
                    Method::Get if url == "/header_pong" => {
                        for header in headers {
                            if header.field.as_str() == "Ping" {
                                let response = Response::from_string(format!("{}", header.value));
                                request.respond(response).ok();
                                return;
                            }
                        }
                        request.respond(Response::from_string("No header!")).ok();
                    }

                    Method::Get if url == "/slow_a" => {
                        thread::sleep(Duration::from_secs(2));
                        let response = Response::from_string(format!("j: {}", content));
                        request.respond(response).ok();
                    }

                    Method::Get if url == "/a" => {
                        let response = Response::from_string(format!("j: {}", content));
                        request.respond(response).ok();
                    }
                    Method::Post if url == "/a" => {
                        let response = Response::from_string("POST to /a is not valid.");
                        request.respond(response).ok();
                    }

                    Method::Get if url == "/long_header" => {
                        let mut long_header = String::with_capacity(1000);
                        long_header += "Very-Long-Header: ";
                        for _ in 0..1000 - long_header.len() {
                            long_header += ".";
                        }
                        let long_header = Header::from_str(&long_header).unwrap();
                        let response = Response::empty(200).with_header(long_header);
                        request.respond(response).ok();
                    }
                    Method::Get if url == "/massive_content_length" => {
                        let status = StatusCode(200);
                        let body = std::io::empty();
                        let length = 1_000_000_000_000_000;
                        let response = Response::new(status, vec![], body, Some(length), None)
                            .with_chunked_threshold(2 * length);
                        request.respond(response).ok();
                    }
                    Method::Get if url == "/long_status_line" => {
                        request.respond(Response::empty(203)).ok();
                    }

                    Method::Get if url == "/redirect-baz" => {
                        let response = Response::empty(301).with_header(
                            Header::from_str("Location: http://localhost:35562/a#baz").unwrap(),
                        );
                        request.respond(response).ok();
                    }

                    Method::Get if url == "/redirect" => {
                        let response = Response::empty(301).with_header(
                            Header::from_bytes(&b"Location"[..], &b"http://localhost:35562/a"[..])
                                .unwrap(),
                        );
                        request.respond(response).ok();
                    }
                    Method::Post if url == "/redirect" => {
                        let response = Response::empty(303).with_header(
                            Header::from_bytes(&b"Location"[..], &b"http://localhost:35562/a"[..])
                                .unwrap(),
                        );
                        request.respond(response).ok();
                    }

                    Method::Get if url == "/infiniteredirect" => {
                        let response = Response::empty(301).with_header(
                            Header::from_bytes(
                                &b"Location"[..],
                                &b"http://localhost:35562/redirectpong"[..],
                            )
                            .unwrap(),
                        );
                        request.respond(response).ok();
                    }
                    Method::Get if url == "/redirectpong" => {
                        let response = Response::empty(301).with_header(
                            Header::from_bytes(
                                &b"Location"[..],
                                &b"http://localhost:35562/infiniteredirect"[..],
                            )
                            .unwrap(),
                        );
                        request.respond(response).ok();
                    }
                    Method::Get if url == "/relativeredirect" => {
                        let response = Response::empty(303)
                            .with_header(Header::from_bytes(&b"Location"[..], &b"/a"[..]).unwrap());
                        request.respond(response).ok();
                    }

                    Method::Post if url == "/echo" => {
                        request.respond(Response::from_string(content)).ok();
                    }

                    Method::Head if url == "/b" => {
                        request.respond(Response::empty(418)).ok();
                    }
                    Method::Post if url == "/c" => {
                        let response = Response::from_string(format!("l: {}", content));
                        request.respond(response).ok();
                    }
                    Method::Put if url == "/d" => {
                        let response = Response::from_string(format!("m: {}", content));
                        request.respond(response).ok();
                    }
                    Method::Delete if url == "/e" => {
                        let response = Response::from_string(format!("n: {}", content));
                        request.respond(response).ok();
                    }
                    Method::Trace if url == "/f" => {
                        let response = Response::from_string(format!("o: {}", content));
                        request.respond(response).ok();
                    }
                    Method::Options if url == "/g" => {
                        let response = Response::from_string(format!("p: {}", content));
                        request.respond(response).ok();
                    }
                    Method::Connect if url == "/h" => {
                        let response = Response::from_string(format!("q: {}", content));
                        request.respond(response).ok();
                    }
                    Method::Patch if url == "/i" => {
                        let response = Response::from_string(format!("r: {}", content));
                        request.respond(response).ok();
                    }

                    _ => {
                        request
                            .respond(Response::from_string("Not Found").with_status_code(404))
                            .ok();
                    }
                }
            });
        }
    });
}

pub fn url(req: &str) -> String { format!("http://localhost:35562{}", req) }

pub async fn maybe_make_request(
    request: bitreq::Request,
) -> Result<bitreq::Response, bitreq::Error> {
    let response = request.clone().send();
    let lazy_response = request.clone().send_lazy();
    match (&response, lazy_response) {
        (Ok(resp), Ok(mut lazy_resp)) => {
            assert_eq!(lazy_resp.status_code, resp.status_code);
            assert_eq!(lazy_resp.reason_phrase, resp.reason_phrase);
            let mut lazy_bytes = Vec::new();
            lazy_resp.read_to_end(&mut lazy_bytes).unwrap();
            assert_eq!(lazy_bytes, resp.as_bytes());
        }
        (Err(e), Err(lazy_e)) => assert_eq!(format!("{e:?}"), format!("{lazy_e:?}")),
        (res, lazy_res) => panic!("{res:?} != {}", lazy_res.is_err()),
    }

    #[cfg(feature = "async")]
    {
        if let Ok(resp) = &response {
            if resp.url.starts_with("https") && !cfg!(feature = "async-https") {
                return response;
            }
        } else {
            // Assume its not HTTPS or async-https is set
        }
        let async_response = request.clone().send_async().await;
        let lazy_async_response = request.send_lazy_async().await;
        match (&response, &async_response) {
            (Ok(resp), Ok(async_resp)) => {
                assert_eq!(async_resp.status_code, resp.status_code);
                assert_eq!(async_resp.reason_phrase, resp.reason_phrase);
                assert_eq!(async_resp.as_bytes(), resp.as_bytes());
            }
            (Err(e), Err(async_e)) => assert_eq!(format!("{e:?}"), format!("{async_e:?}")),
            (res, async_res) => panic!("{res:?} != {async_res:?}"),
        }
        match (&response, lazy_async_response) {
            (Ok(resp), Ok(mut lazy_resp)) => {
                assert_eq!(lazy_resp.status_code, resp.status_code);
                assert_eq!(lazy_resp.reason_phrase, resp.reason_phrase);
                let mut lazy_bytes = Vec::new();
                lazy_resp.read_to_end(&mut lazy_bytes).unwrap();
                assert_eq!(lazy_bytes, resp.as_bytes());
            }
            (Err(e), Err(lazy_e)) => assert_eq!(format!("{e:?}"), format!("{lazy_e:?}")),
            (res, lazy_res) => panic!("{res:?} != {}", lazy_res.is_err()),
        }
    }
    response
}

pub async fn make_request(request: bitreq::Request) -> bitreq::Response {
    maybe_make_request(request).await.unwrap()
}

pub async fn get_body(request: bitreq::Request) -> String {
    let response = make_request(request).await;
    String::from(response.as_str().unwrap())
}

pub async fn get_status_code(request: bitreq::Request) -> i32 {
    let response = make_request(request).await;
    response.status_code
}
