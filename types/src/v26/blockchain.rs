// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v26` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

use core::fmt;

use bitcoin::{amount, hex, Amount, BlockHash};
use serde::{Deserialize, Serialize};

use crate::error::write_err;
use crate::{model, NumericError};

/// Result of JSON-RPC method `gettxoutsetinfo`.
///
/// > gettxoutsetinfo
/// >
/// > Returns statistics about the unspent transaction output set.
/// > Note this call may take some time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTxOutSetInfo {
    /// The current block height (index).
    pub height: i64,
    /// The hash of the block at the tip of the chain.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of transactions with unspent outputs.
    pub transactions: i64,
    /// The number of unspent transaction outputs.
    #[serde(rename = "txouts")]
    pub tx_outs: i64,
    /// A meaningless metric for UTXO set size.
    #[serde(rename = "bogosize")]
    pub bogo_size: i64,
    /// The estimated size of the chainstate on disk.
    pub disk_size: i64,
    /// The total amount.
    pub total_amount: f64,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen).
    /// v26 and later only.
    pub hash_serialized_3: Option<String>,
}

impl GetTxOutSetInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTxOutSetInfo, GetTxOutSetInfoError> {
        use GetTxOutSetInfoError as E;

        let height = crate::to_u32(self.height, "height")?;
        let best_block = self.best_block.parse::<BlockHash>().map_err(E::BestBlock)?;
        let transactions = crate::to_u32(self.transactions, "transactions")?;
        let tx_outs = crate::to_u32(self.tx_outs, "tx_outs")?;
        let bogo_size = crate::to_u32(self.bogo_size, "bogo_size")?;
        let disk_size = crate::to_u32(self.disk_size, "disk_size")?;
        let total_amount = Amount::from_btc(self.total_amount).map_err(E::TotalAmount)?;

        Ok(model::GetTxOutSetInfo {
            height,
            best_block,
            transactions,
            tx_outs,
            bogo_size,
            hash_serialized_2: None, // v17 to v25 only.
            hash_serialized_3: self.hash_serialized_3,
            disk_size,
            total_amount,
        })
    }
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
