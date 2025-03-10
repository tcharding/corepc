// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.20` - control.
//!
//! Types for methods found under the `== Control ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `logging`.
///
/// > logging ( `<include>` `<exclude>` )
///
/// > Gets and sets the logging configuration.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Logging {
    pub net: bool,
    pub tor: bool,
    pub mempool: bool,
    pub http: bool,
    pub bench: bool,
    pub zmq: bool,
    pub walletdb: bool,
    pub rpc: bool,
    pub estimatefee: bool,
    pub addrman: bool,
    pub selectcoins: bool,
    pub reindex: bool,
    pub cmpctblock: bool,
    pub rand: bool,
    pub prune: bool,
    pub proxy: bool,
    pub mempoolrej: bool,
    pub libevent: bool,
    pub coindb: bool,
    pub qt: bool,
    pub leveldb: bool,
    pub validation: bool,
}
