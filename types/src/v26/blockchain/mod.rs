// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v26` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::{DumpTxOutSetError, GetTxOutSetInfoError};

/// Result of JSON-RPC method `dumptxoutset`.
///
/// > dumptxoutset "path"
/// >
/// > Write the serialized UTXO set to disk.
/// >
/// > Arguments:
/// > 1. path    (string, required) Path to the output file. If relative, will be prefixed by datadir.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DumpTxOutSet {
    /// The number of coins written in the snapshot.
    pub coins_written: f64,
    /// The hash of the base of the snapshot.
    pub base_hash: String,
    /// The height of the base of the snapshot.
    pub base_height: i64,
    /// The absolute path that the snapshot was written to.
    pub path: String,
    /// The hash of the UTXO set contents.
    #[serde(rename = "txoutset_hash")]
    pub tx_out_set_hash: String,
    /// The number of transactions in the chain up to and including the base block.
    #[serde(rename = "nchaintx")]
    pub n_chain_tx: i64,
}

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
