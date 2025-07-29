// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v26` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::GetTxOutSetInfoError;

/// Result of JSON-RPC method `gettxoutsetinfo`.
///
/// > gettxoutsetinfo
/// >
/// > Returns statistics about the unspent transaction output set.
/// > Note this call may take some time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
