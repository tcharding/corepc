// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v28` - mining.
//!
//! Types for methods found under the `== Mining ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `getmininginfo`.
///
/// > getmininginfo
/// >
/// > Returns a json object containing mining-related information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMiningInfo {
    /// The current block.
    pub blocks: i64,
    /// The last block weight.
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: Option<i64>,
    /// The last block transaction.
    #[serde(rename = "currentblocktx")]
    pub current_block_transaction: Option<i64>,
    /// The current difficulty.
    pub difficulty: f64,
    /// The network hashes per second.
    #[serde(rename = "networkhashps")]
    pub network_hash_ps: i64,
    /// The size of the mempool.
    #[serde(rename = "pooledtx")]
    pub pooled_transactions: i64,
    /// Current network name as defined in BIP70 (main, test, regtest).
    pub chain: String,
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}
