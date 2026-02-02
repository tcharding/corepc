// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

use bitreq::Url as BitreqUrl;

#[inline]
pub fn do_test(data: &[u8]) {
    // Convert the byte slice to a string, ignoring invalid UTF-8
    let input = String::from_utf8_lossy(data);

    // Try to parse with both implementations
    let bitreq_result = BitreqUrl::parse(&input);
    let url_result = url::Url::parse(&input);

    match (bitreq_result, url_result) {
        (Ok(bitreq_url), Ok(ref_url)) => {
            // Both parsed successfully - compare all accessors

            assert_eq!(
                bitreq_url.scheme(),
                ref_url.scheme(),
                "Scheme mismatch for input: {input:?}",
            );

            assert_eq!(
                bitreq_url.username(),
                ref_url.username(),
                "Username mismatch for input: {input:?}",
            );

            assert_eq!(
                bitreq_url.password(),
                ref_url.password(),
                "Password mismatch for input: {input:?}",
            );

            if let Some(ref_host) = ref_url.host_str() {
                assert_eq!(bitreq_url.base_url(), ref_host, "Host mismatch for input: {input:?}",);
            }

            // Port handling: url crate returns Option<u16> for explicit port,
            // while ours returns the actual port (explicit or default).
            // If url crate has an explicit port, it should match ours.
            if let Some(ref_port) = ref_url.port() {
                assert_eq!(bitreq_url.port(), ref_port, "Port mismatch for input: {input:?}",);
            }

            assert_eq!(bitreq_url.path(), ref_url.path(), "Path mismatch for input: {input:?}",);

            assert_eq!(
                bitreq_url.query(),
                ref_url.query(),
                "Query mismatch for input: {:?}",
                input
            );

            assert_eq!(
                bitreq_url.fragment(),
                ref_url.fragment(),
                "Fragment mismatch for input: {:?}",
                input
            );

            let _ = format!("{}", bitreq_url);
            let _ = bitreq_url.as_str();

            let bitreq_segments: Vec<_> = bitreq_url.path_segments().collect();
            let ref_segments: Vec<_> =
                ref_url.path_segments().map(|s| s.collect::<Vec<_>>()).unwrap_or_default();
            assert_eq!(
                bitreq_segments, ref_segments,
                "Path segments mismatch for input: {:?}",
                input
            );

            let bitreq_pairs: Vec<(&str, &str)> = bitreq_url.query_pairs().collect();
            let ref_pairs: Vec<(String, String)> =
                ref_url.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();
            let ref_pairs_str: Vec<(&str, &str)> =
                ref_pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            assert_eq!(bitreq_pairs, ref_pairs_str, "Query pairs mismatch for input: {:?}", input);
        }
        (Ok(v), Err(e)) => {
            panic!("bitreq parsed, URL did not. Input {input:?}. Err {e:?}");
        }
        (Err(e), Ok(v)) => match e {
            bitreq::UrlParseError::InvalidCharacter(_) => { 
                // InvalidCharacter errors are currently expected as bitreq::Url only handles ASCII
                // characters.
            }
            bitreq::UrlParseError::MissingScheme | bitreq::UrlParseError::InvalidScheme => {
                // MissingScheme or InvalidScheme errors are expected as bitreq::Url parses the scheme more
                // strictly.
            }
            _ => {
                panic!("URL parsed, bitreq did not. Input {input:?}. Err {e:?}");
            }
        },
        (Err(_), Err(_)) => {
            // Both failed to parse - this is fine
        }
    }
}

#[no_mangle]
pub extern "C" fn url_parse_run(data: *const u8, datalen: usize) {
    do_test(unsafe { std::slice::from_raw_parts(data, datalen) });
}
