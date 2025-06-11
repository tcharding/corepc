// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `ListReceivedByLabel` type into the model type.
#[derive(Debug)]
pub enum ListReceivedByLabelError {
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for ListReceivedByLabelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListReceivedByLabelError::*;

        match *self {
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListReceivedByLabelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListReceivedByLabelError::*;

        match *self {
            Amount(ref e) => Some(e),
            Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ListReceivedByLabelError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
