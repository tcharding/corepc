// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::{amount, hex};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `ChainState` type into the model type.
#[derive(Debug)]
pub enum GetChainStatesError {
    /// Conversion of the `best_block_hash` field failed.
    BestBlockHash(hex::HexToArrayError),
    /// Conversion of the `snapshot_block_hash` field failed.
    SnapshotBlockHash(hex::HexToArrayError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for GetChainStatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetChainStatesError::*;

        match *self {
            BestBlockHash(ref e) =>
                write_err!(f, "conversion of the `best_block_hash` field failed"; e),
            SnapshotBlockHash(ref e) =>
                write_err!(f, "conversion of the `snapshot_block_hash` field failed"; e),
            Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetChainStatesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetChainStatesError::*;

        match *self {
            BestBlockHash(ref e) => Some(e),
            SnapshotBlockHash(ref e) => Some(e),
            Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetChainStatesError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `DumpTxOutSet` type into the model type.
#[derive(Debug)]
pub enum DumpTxOutSetError {
    /// Conversion of the `coins_written` field to Amount failed.
    CoinsWritten(amount::ParseAmountError),
    /// Conversion of the `base_hash` field failed.
    BaseHash(hex::HexToArrayError),
    /// Conversion of the `txoutset_hash` field failed.
    TxOutSetHash(hex::HexToArrayError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for DumpTxOutSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DumpTxOutSetError::*;

        match *self {
            CoinsWritten(ref e) =>
                write_err!(f, "conversion of the `coins_written` field failed"; e),
            BaseHash(ref e) => write_err!(f, "conversion of the `base_hash` field failed"; e),
            TxOutSetHash(ref e) =>
                write_err!(f, "conversion of the `txoutset_hash` field failed"; e),
            Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DumpTxOutSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use DumpTxOutSetError::*;

        match *self {
            CoinsWritten(ref e) => Some(e),
            BaseHash(ref e) => Some(e),
            TxOutSetHash(ref e) => Some(e),
            Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for DumpTxOutSetError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

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

/// Error when converting a `LoadTxOutSet` type into the model type.
#[derive(Debug)]
pub enum LoadTxOutSetError {
    /// Conversion of the `coins_loaded` field to Amount failed.
    CoinsLoaded(amount::ParseAmountError),
    /// Conversion of the `tip_hash` field failed.
    TipHash(hex::HexToArrayError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for LoadTxOutSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LoadTxOutSetError::*;

        match *self {
            CoinsLoaded(ref e) => write_err!(f, "conversion of the `coins_loaded` field failed"; e),
            TipHash(ref e) => write_err!(f, "conversion of the `tip_hash` field failed"; e),
            Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LoadTxOutSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use LoadTxOutSetError::*;

        match *self {
            CoinsLoaded(ref e) => Some(e),
            TipHash(ref e) => Some(e),
            Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for LoadTxOutSetError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
