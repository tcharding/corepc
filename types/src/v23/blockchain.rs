// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v23` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.
use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `savemempool`.
///
/// > savemempool
///
/// > Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SaveMempool {
    /// The directory and file where the mempool was saved.
    pub filename: String,
}
