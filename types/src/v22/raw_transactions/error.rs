// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::{address, hex};

use crate::error::write_err;

/// Error when converting a `DecodeScript` type into the model type.
#[derive(Debug)]
pub enum DecodeScriptError {
    /// Conversion of the transaction `hex` field failed.
    Hex(hex::HexToBytesError),
    /// Conversion of the transaction `address` field failed.
    Address(address::ParseError),
    /// Conversion of the transaction `addresses` field failed.
    Addresses(address::ParseError),
    /// Conversion of the transaction `p2sh` field failed.
    P2sh(address::ParseError),
}

impl fmt::Display for DecodeScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecodeScriptError as E;

        match *self {
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::Addresses(ref e) => write_err!(f, "conversion of the `addresses` field failed"; e),
            E::P2sh(ref e) => write_err!(f, "conversion of the `p2sh` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeScriptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use DecodeScriptError as E;

        match *self {
            E::Hex(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::Addresses(ref e) => Some(e),
            E::P2sh(ref e) => Some(e),
        }
    }
}
