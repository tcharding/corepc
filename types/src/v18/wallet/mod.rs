// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::ListReceivedByLabelError;

/// Result of the JSON-RPC method `getreceivedbylabel`.
///
/// > getreceivedbylabel "label" ( minconf )
/// >
/// > Returns the total amount received by addresses with `<label>` in transactions with at least `[minconf]` confirmations.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetReceivedByLabel(pub f64);

/// Result of the JSON-RPC method `listreceivedbylabel`.
///
/// > listreceivedbylabel ( minconf include_empty include_watchonly )
/// >
/// > List received transactions by label.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ListReceivedByLabel(pub Vec<ListReceivedByLabelItem>);

/// Item returned as part of `listreceivedbylabel`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ListReceivedByLabelItem {
    /// Only returned if imported addresses were involved in transaction.
    #[serde(rename = "involvesWatchonly")]
    pub involves_watch_only: Option<bool>,
    /// The total amount received by addresses with this label.
    pub amount: f64,
    /// The number of confirmations of the most recent transaction included.
    pub confirmations: i64,
    /// The label of the receiving address. The default label is "".
    pub label: String,
}

/// Result of the JSON-RPC method `listunspent`.
///
/// > listunspent ( minconf maxconf  ["addresses",...] `[include_unsafe]` `[query_options]`)
/// >
/// > Returns array of unspent transaction outputs
/// > with between minconf and maxconf (inclusive) confirmations.
/// > Optionally filter to only include txouts paid to specified addresses.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ListUnspent(pub Vec<ListUnspentItem>);

/// Unspent transaction output, returned as part of `listunspent`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ListUnspentItem {
    /// The transaction id.
    pub txid: String,
    /// The vout value.
    pub vout: i64,
    /// The bitcoin address of the transaction.
    pub address: String,
    /// The associated label, or "" for the default label.
    pub label: String,
    /// The script key.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    /// The transaction amount in BTC.
    pub amount: f64,
    /// The number of confirmations.
    pub confirmations: i64,
    /// The redeemScript if scriptPubKey is P2SH.
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<String>,
    /// Whether we have the private keys to spend this output.
    pub spendable: bool,
    /// Whether we know how to spend this output, ignoring the lack of keys.
    pub solvable: bool,
    /// A descriptor for spending this output (only when solvable)
    #[serde(rename = "desc")]
    pub descriptor: Option<String>,
    /// Whether this output is considered safe to spend. Unconfirmed transactions from outside keys
    /// and unconfirmed replacement transactions are considered unsafe and are not eligible for
    /// spending by fundrawtransaction and sendtoaddress.
    pub safe: bool,
}

/// Result of the JSON-RPC method `listwalletdir`.
///
/// > listwalletdir
/// >
/// > Returns a list of wallets in the wallet directory.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ListWalletDir {
    /// The list of wallets in the wallet directory.
    pub wallets: Vec<ListWalletDirWallet>,
}

/// Wallet entry returned as part of `listwalletdir`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ListWalletDirWallet {
    /// The wallet name.
    pub name: String,
}
