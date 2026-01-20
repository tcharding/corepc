//! Percent encoding utilities for URL query parameters.

/// Returns the `%HH` triplet representing `byte` for percent encoding.
fn percent_encoded_triplet(byte: u8) -> [char; 3] {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    ['%', HEX[(byte >> 4) as usize] as char, HEX[(byte & 0x0F) as usize] as char]
}

/// Percent-encodes a char and appends it to `result`.
/// Unreserved characters (0-9, A-Z, a-z, -, ., _, ~) are not encoded.
pub(crate) fn percent_encode_char(c: char, result: &mut String) {
    match c {
        // All URL-'safe' characters are not encoded
        '0'..='9' | 'A'..='Z' | 'a'..='z' | '-' | '.' | '_' | '~' => {
            result.push(c);
        }
        _ => {
            // Any UTF-8 character can fit in 4 bytes
            let mut utf8_buf = [0u8; 4];
            c.encode_utf8(&mut utf8_buf).as_bytes().iter().for_each(|byte| {
                for ch in percent_encoded_triplet(*byte) {
                    result.push(ch);
                }
            });
        }
    }
}

/// Percent-encodes the entire input string and returns the encoded version.
pub(crate) fn percent_encode_string(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());
    for ch in input.chars() {
        percent_encode_char(ch, &mut encoded);
    }
    encoded
}
