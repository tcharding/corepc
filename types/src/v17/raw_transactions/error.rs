// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;

use crate::error::write_err;

/// Error when converting a `FundRawTransaction` type into the model type.
#[derive(Debug)]
pub enum FundRawTransactionError {
    /// Conversion of the transaction `hex` field failed.
    Hex(encode::FromHexError),
    /// Conversion of the transaction `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for FundRawTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FundRawTransactionError as E;

        match *self {
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FundRawTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use FundRawTransactionError as E;

        match *self {
            E::Hex(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
}
