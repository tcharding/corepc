// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::{amount, hex};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetTxOut` type into the model type.
#[derive(Debug)]
pub enum GetTxOutSetInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the transaction `total_amount` field failed.
    TotalAmount(amount::ParseAmountError),
}

impl fmt::Display for GetTxOutSetInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTxOutSetInfoError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            BestBlock(ref e) => write_err!(f, "conversion of the `beast_block` field failed"; e),
            TotalAmount(ref e) => write_err!(f, "conversion of the `total_amount` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTxOutSetInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTxOutSetInfoError::*;

        match *self {
            Numeric(ref e) => Some(e),
            BestBlock(ref e) => Some(e),
            TotalAmount(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTxOutSetInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
