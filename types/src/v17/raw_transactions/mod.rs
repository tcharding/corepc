// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

// TODO: Remove wildcard, use explicit types.
pub use self::error::*;

/// Result of JSON-RPC method `createrawtransaction`.
///
/// > createrawtransaction [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )
/// >
/// > Create a transaction spending the given inputs and creating new outputs.
/// > Outputs can be addresses or data.
/// > Returns hex-encoded raw transaction.
/// > Note that the transaction's inputs are not signed, and
/// > it is not stored in the wallet or transmitted to the network.
/// >
/// > Arguments:
/// > 1. "inputs"                (array, required) A json array of json objects
/// >      [
/// >        {
/// >          "txid":"id",      (string, required) The transaction id
/// >          "vout":n,         (numeric, required) The output number
/// >          "sequence":n      (numeric, optional) The sequence number
/// >        }
/// >        ,...
/// >      ]
/// > 2. "outputs"               (array, required) a json array with outputs (key-value pairs)
/// >    [
/// >     {
/// >       "address": x.xxx,    (obj, optional) A key-value pair. The key (string) is the bitcoin address, the value (float or string) is the amount in BTC
/// >     },
/// >     {
/// >       "data": "hex"        (obj, optional) A key-value pair. The key must be "data", the value is hex encoded data
/// >     }
/// >     ,...                     More key-value pairs of the above form. For compatibility reasons, a dictionary, which holds the key-value pairs directly, is also
/// >                              accepted as second parameter.
/// >    ]
/// > 3. locktime                  (numeric, optional, default=0) Raw locktime. Non-0 value also locktime-activates inputs
/// > 4. replaceable               (boolean, optional, default=false) Marks this transaction as BIP125 replaceable.
/// >                              Allows this transaction to be replaced by a transaction with higher fees. If provided, it is an error if explicit sequence numbers are incompatible.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateRawTransaction(
    /// The hex encoded transaction.
    pub String,
);

/// Result of JSON-RPC method `fundrawtransaction`.
///
/// > fundrawtransaction "hexstring" ( options iswitness )
/// >
/// > Add inputs to a transaction until it has enough in value to meet its out value.
/// > This will not modify existing inputs, and will add at most one change output to the outputs.
/// > No existing outputs will be modified unless "subtractFeeFromOutputs" is specified.
/// > Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
/// > The inputs added will not be signed, use signrawtransaction for that.
/// > Note that all existing inputs must have their previous output transaction be in the wallet.
/// > Note that all inputs selected must be of standard form and P2SH scripts must be
/// > in the wallet using importaddress or addmultisigaddress (to calculate fees).
/// > You can see whether this is the case by checking the "solvable" field in the listunspent output.
/// > Only pay-to-pubkey, multisig, and P2SH versions thereof are currently supported for watch-only
/// >
/// > Arguments:
/// > 1. "hexstring"           (string, required) The hex string of the raw transaction
/// > 2. options                 (object, optional)
/// >    {
/// >      "changeAddress"          (string, optional, default pool address) The bitcoin address to receive the change
/// >      "changePosition"         (numeric, optional, default random) The index of the change output
/// >      "change_type"            (string, optional) The output type to use. Only valid if changeAddress is not specified. Options are "legacy", "p2sh-segwit", and "bech32". Default is set by -changetype.
/// >      "includeWatching"        (boolean, optional, default false) Also select inputs which are watch only
/// >      "lockUnspents"           (boolean, optional, default false) Lock selected unspent outputs
/// >      "feeRate"                (numeric, optional, default not set: makes wallet determine the fee) Set a specific fee rate in BTC/kB
/// >      "subtractFeeFromOutputs" (array, optional) A json array of integers.
/// >                               The fee will be equally deducted from the amount of each specified output.
/// >                               The outputs are specified by their zero-based index, before any change output is added.
/// >                               Those recipients will receive less bitcoins than you enter in their corresponding amount field.
/// >                               If no outputs are specified here, the sender pays the fee.
/// >                                   [vout_index,...]
/// >      "replaceable"            (boolean, optional) Marks this transaction as BIP125 replaceable.
/// >                               Allows this transaction to be replaced by a transaction with higher fees
/// >      "conf_target"            (numeric, optional) Confirmation target (in blocks)
/// >      "estimate_mode"          (string, optional, default=UNSET) The fee estimate mode, must be one of:
/// >          "UNSET"
/// >          "ECONOMICAL"
/// >          "CONSERVATIVE"
/// >    }
/// >                          for backward compatibility: passing in a true instead of an object will result in {"includeWatching":true}
/// > 3. iswitness               (boolean, optional) Whether the transaction hex is a serialized witness transaction
/// >                               If iswitness is not present, heuristic tests will be used in decoding
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FundRawTransaction {
    /// The resulting raw transaction (hex-encoded string).
    pub hex: String,
    /// Fee in BTC the resulting transaction pays.
    pub fee: f64,
    /// The position of the added change output, or -1.
    #[serde(rename = "changepos")]
    pub change_position: i64,
}

/// Result of JSON-RPC method `sendrawtransaction`.
///
/// > sendrawtransaction "hexstring" ( allowhighfees )
/// >
/// > Submits raw transaction (serialized, hex-encoded) to local node and network.
/// >
/// > Also see createrawtransaction and signrawtransactionwithkey calls.
/// >
/// > Arguments:
/// > 1. hexstring        (string, required) The hex string of the raw transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendRawTransaction(
    /// The hex encoded txid.
    pub String,
);
