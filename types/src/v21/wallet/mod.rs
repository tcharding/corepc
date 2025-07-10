// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.21` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `importdescriptors`.
///
/// > Import descriptors. This will trigger a rescan of the blockchain based on the earliest
/// > timestamp of all descriptors being imported. Requires a new wallet backup.
/// >
/// > Note: This call can take over an hour to complete if using an early timestamp; during that
/// > time, other rpc calls may report that the imported keys, addresses or scripts exist but
/// > related transactions are still missing.
/// >
/// > Arguments:
/// > 1. requests (json array, required) Data to be imported
/// >    [
/// >      { (json object)
/// >        "desc": "str", (string, required) Descriptor to import.
/// >        "active": bool, (boolean, optional, default=false) Set this descriptor to be the
/// >            active descriptor for the corresponding output type/externality.
/// >        "range": n or \[n,n\], (numeric or array) If a ranged descriptor is used, this
/// >            specifies the end or the range (in the form \[begin,end\]) to import.
/// >        "next_index": n, (numeric) If a ranged descriptor is set to active, this specifies
/// >            the next index to generate addresses from.
/// >        "timestamp": timestamp | "now", (integer / string, required) Time from which to
/// >            start rescanning the blockchain for this descriptor, in UNIX epoch time.
/// >            Use the string "now" to substitute the current synced blockchain time.
/// >            "now" can be specified to bypass scanning, for outputs which are known to never
/// >            have been used, and 0 can be specified to scan the entire blockchain. Blocks up
/// >            to 2 hours before the earliest timestamp of all descriptors being imported will
/// >            be scanned.
/// >        "internal": bool, (boolean, optional, default=false) Whether matching outputs should
/// >            be treated as not incoming payments (e.g. change).
/// >        "label": "str", (string, optional, default='') Label to assign to the address, only
/// >            allowed with internal=false.
/// >      },
/// >      ...
/// >    ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImportDescriptors(
    /// Response is an array with the same size as the input that has the execution result.
    pub Vec<ImportDescriptorsResult>,
);

/// Result object for each descriptor import in `importdescriptors`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImportDescriptorsResult {
    /// Whether the import was successful.
    pub success: bool,
    /// Warnings, if any.
    pub warnings: Option<Vec<String>>,
    /// Error object, if any.
    pub error: Option<serde_json::Value>,
}

/// Result of the JSON-RPC method `unloadwallet`.
///
/// > unloadwallet ( "wallet_name" load_on_startup )
/// >
/// > Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// > Specifying the wallet name on a wallet endpoint is invalid.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnloadWallet {
    /// Warning messages, if any, related to unloading the wallet.
    pub warning: String,
}
