// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v29` - mining.
//!
//! Types for methods found under the `== Mining ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::{
    BlockTemplateTransactionError, GetMiningInfoError, NextBlockInfoError,
};

/// Contents of non-coinbase transactions that should be included in the next block.
///
/// Returned as part of `getblocktemplate`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BlockTemplateTransaction {
    /// Transaction data encoded in hexadecimal (byte-for-byte).
    pub data: String,
    /// Transaction id encoded in little-endian hexadecimal.
    pub txid: String,
    /// Hash encoded in little-endian hexadecimal (including witness data).
    pub hash: String,
    /// Array of numbers.
    ///
    /// Transactions before this one (by 1-based index in 'transactions' list) that must be present in the final block if this one is.
    pub depends: Vec<i64>,
    /// Difference in value between transaction inputs and outputs (in satoshis); for coinbase
    /// transactions, this is a negative Number of the total collected block fees (ie, not including
    /// the block subsidy); if key is not present, fee is unknown and clients MUST NOT assume there
    /// isn't one.
    pub fee: i64,
    /// Total SigOps cost, as counted for purposes of block limits; if key is not present, sigop
    /// cost is unknown and clients MUST NOT assume it is zero.
    pub sigops: i64,
    /// Total transaction weight, as counted for purposes of block limits.
    pub weight: u64,
}

/// Result of the JSON-RPC method `getmininginfo`.
///
/// > getmininginfo
/// >
/// > Returns a json object containing mining-related information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMiningInfo {
    /// The current block.
    pub blocks: u64,
    /// The block weight (including reserved weight for block header, txs count and coinbase tx) of
    /// the last assembled block (only present if a block was ever assembled).
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: Option<u64>,
    /// The number of block transactions (excluding coinbase) of the last assembled block (only
    /// present if a block was ever assembled).
    #[serde(rename = "currentblocktx")]
    pub current_block_tx: Option<i64>,
    /// The current nBits, compact representation of the block difficulty target.
    pub bits: String,
    /// The current difficulty.
    pub difficulty: f64,
    /// The current target.
    pub target: String,
    /// The network hashes per second.
    #[serde(rename = "networkhashps")]
    pub network_hash_ps: i64,
    /// The size of the mempool.
    #[serde(rename = "pooledtx")]
    pub pooled_tx: i64,
    /// Current network name as defined in BIP70 (main, test, regtest).
    pub chain: String,
    /// The block challenge (aka. block script), in hexadecimal (only present if the current network
    /// is a signet).
    pub signet_challenge: Option<String>,
    /// The next block.
    pub next: NextBlockInfo,
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}

/// Represents the `next` block information within the GetMiningInfo result.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NextBlockInfo {
    /// The next height.
    pub height: u64,
    /// The next nBits.
    pub bits: String,
    /// The next difficulty.
    pub difficulty: f64,
    /// The next target.
    pub target: String,
}
