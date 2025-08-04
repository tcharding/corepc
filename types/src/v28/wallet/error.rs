// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::bip32;

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetHdKeys` type into the model type.
#[derive(Debug)]
pub enum GetHdKeysError {
    /// Conversion of the `xpub` field failed.
    Xpub(bip32::Error),
    /// Conversion of the `xpriv` field failed.
    Xpriv(bip32::Error),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for GetHdKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetHdKeysError::*;
        match *self {
            Xpub(ref e) => write_err!(f, "conversion of the `xpub` field failed"; e),
            Xpriv(ref e) => write_err!(f, "conversion of the `xpriv` field failed"; e),
            Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetHdKeysError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetHdKeysError::*;
        match *self {
            Xpub(ref e) => Some(e),
            Xpriv(ref e) => Some(e),
            Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetHdKeysError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
