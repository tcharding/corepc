// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Wallet ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use alloc::collections::BTreeMap;

use bitcoin::address::NetworkUnchecked;
use bitcoin::hashes::hash160;
use bitcoin::{
    bip32, ecdsa, Address, Amount, BlockHash, FeeRate, PrivateKey, Psbt, PublicKey, ScriptBuf,
    Sequence, SignedAmount, Transaction, Txid, WitnessProgram, WitnessVersion,
};
use serde::{Deserialize, Serialize};

/// The purpose of an address.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum AddressPurpose {
    /// A send-to address.
    Send,
    /// A receive-from address.
    Receive,
}

/// The category of a transaction.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum TransactionCategory {
    /// Transaction is a send.
    Send,
    /// Transactions is a receive.
    Receive,
}

/// Whether this transaction can be RBF'ed.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum Bip125Replaceable {
    /// Yes, can be replaced due to BIP-125 (RBF).
    Yes,
    /// No, cannot be replaced due to BIP-125 (RBF).
    No,
    /// RBF unknown.
    Unknown,
}

/// Models the result of JSON-RPC method `addmultisigaddress`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddMultisigAddress {
    /// The new multisig address.
    pub address: Address<NetworkUnchecked>,
    /// The redemption script.
    pub redeem_script: ScriptBuf,
}

/// Models the result of JSON-RPC method `bumpfee`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BumpFee {
    /// The id of the new transaction.
    pub txid: Txid,
    /// Fee of the replaced transaction.
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub original_fee: Amount,
    /// Fee of the new transaction.
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub fee: Amount,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
}

/// Models the result of JSON-RPC method `createwallet`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateWallet {
    /// The wallet name if created successfully.
    ///
    /// If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warnings: Vec<String>,
}

/// Models the result of JSON-RPC method `dumpprivkey`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DumpPrivKey(pub PrivateKey);

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DumpWallet {
    /// The filename with full absolute path.
    pub file_name: String, // FIXME: Should this be `PathBuf`?
}

/// Models the result of JSON-RPC method `getaddressesbylabel`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAddressesByLabel(pub BTreeMap<Address<NetworkUnchecked>, AddressInformation>);

/// Information about address.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddressInformation {
    /// Purpose of address.
    pub purpose: AddressPurpose,
}

/// Models the result of JSON-RPC method `getaddressinfo`.
// TODO: Support serde (currently not supported by `WitnessProgram` or `WitnessVersion`)
// https://github.com/rust-bitcoin/rust-bitcoin/issues/3513
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetAddressInfo {
    /// The bitcoin address validated.
    pub address: Address<NetworkUnchecked>,
    /// The hex encoded scriptPubKey generated by the address.
    pub script_pubkey: ScriptBuf,
    /// If the address is yours or not.
    pub is_mine: bool,
    /// If the address is watchonly.
    pub is_watch_only: bool,
    /// If the key is a script.
    pub is_script: bool,
    /// If the address is a witness address.
    pub is_witness: bool,
    /// The version number of the witness program.
    pub witness_version: Option<WitnessVersion>,
    /// The hex value of the witness program.
    pub witness_program: Option<WitnessProgram>,
    /// The output script type.
    ///
    /// Only if "is_script" is true and the redeemscript is known.
    pub script: Option<ScriptType>,
    /// The redeemscript for the p2sh address.
    // TODO: Should we rename this to redeem_script?
    pub hex: Option<ScriptBuf>,
    /// Array of pubkeys associated with the known redeemscript (only if "script" is "multisig").
    pub pubkeys: Option<Vec<PublicKey>>,
    /// Number of signatures required to spend multisig output (only if "script" is "multisig").
    pub sigs_required: Option<u32>,
    /// The hex value of the raw public key, for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Option<PublicKey>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    pub embedded: Option<GetAddressInfoEmbedded>,
    /// If the address is compressed.
    pub is_compressed: bool,
    /// The label associated with the address, "" is the default account.
    pub label: String,
    /// The creation time of the key if available in seconds since epoch (Jan 1 1970 GMT).
    pub timestamp: Option<u32>,
    /// The HD keypath if the key is HD and available.
    pub hd_key_path: Option<bip32::DerivationPath>,
    /// The Hash160 of the HD seed.
    pub hd_seed_id: Option<hash160::Hash>,
    /// Labels associated with the address.
    pub labels: Vec<AddressLabel>,
}

/// An address script type.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ScriptType {
    /// Non-standard output script type.
    NonStandard,
    /// Pubkey output script.
    Pubkey,
    /// Pubkey hash output script.
    PubkeyHash,
    /// Script hash output script.
    ScriptHash,
    /// Multisig output script.
    Multisig,
    /// Null data for output script.
    NullData,
    /// Witness version 0 key hash output script.
    WitnessV0KeyHash,
    /// Witness version 0 script hash output script.
    WitnessV0ScriptHash,
    /// Witness unknown for output script.
    WitnessUnknown,
}

/// An address label.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddressLabel {
    /// The address label.
    pub name: String,
    /// Purpose of address (send or receive).
    pub purpose: AddressPurpose,
}

/// The `embedded` field of `GetAddressInfo`.
///
/// It includes all getaddressinfo output fields for the embedded address, excluding metadata
/// ("timestamp", "hdkeypath", "hdseedid") and relation to the wallet ("ismine", "iswatchonly",
/// "account").
// TODO: Support serde (currently not supported by `WitnessProgram` or `WitnessVersion`)
// https://github.com/rust-bitcoin/rust-bitcoin/issues/3513
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetAddressInfoEmbedded {
    /// The bitcoin address validated.
    pub address: Address<NetworkUnchecked>,
    /// The hex encoded scriptPubKey generated by the address.
    pub script_pubkey: ScriptBuf,
    /// If the key is a script.
    pub is_script: bool,
    /// If the address is a witness address.
    pub is_witness: bool,
    /// The version number of the witness program.
    pub witness_version: Option<WitnessVersion>,
    /// The hex value of the witness program.
    pub witness_program: Option<WitnessProgram>,
    /// The output script type.
    ///
    /// Only if "is_script" is true and the redeemscript is known.
    pub script: Option<ScriptType>,
    /// The redeemscript for the p2sh address.
    pub hex: Option<ScriptBuf>,
    /// Array of pubkeys associated with the known redeemscript (only if "script" is "multisig").
    pub pubkeys: Vec<PublicKey>,
    /// Number of signatures required to spend multisig output (only if "script" is "multisig").
    pub sigs_required: Option<u32>,
    /// The hex value of the raw public key, for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Option<PublicKey>,
    /// If the address is compressed.
    pub is_compressed: bool,
    /// The label associated with the address, "" is the default account.
    pub label: String,
    /// Labels associated with the address.
    pub labels: Vec<AddressLabel>,
}

/// Models the result of JSON-RPC method `getbalance`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalance(
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub Amount
);

/// Models the result of JSON-RPC method `getbalances`.
///
/// Core version 0.19 onwards.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalances {
    /// Balances from outputs that the wallet can sign.
    pub mine: GetBalancesMine,
    pub watch_only: Option<GetBalancesWatchOnly>,
}

/// Balances from outputs that the wallet can sign.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesMine {
    /// Trusted balance (outputs created by the wallet or confirmed outputs).
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub trusted: Amount,
    /// Untrusted pending balance (outputs created by others that are in the mempool).
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub untrusted_pending: Amount,
    /// Balance from immature coinbase outputs.
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub immature: Amount,
    /// Balance from coins sent to addresses that were previously spent from (potentially privacy violating).
    ///
    /// Only present if `avoid_reuse` is set.
    #[serde(with = "bitcoin::amount::serde::as_sat::opt")]
    pub used: Option<Amount>,
}

/// Hash and height of the block this information was generated on.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesWatchOnly {
    /// Trusted balance (outputs created by the wallet or confirmed outputs).
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub trusted: Amount,
    /// Untrusted pending balance (outputs created by others that are in the mempool).
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub untrusted_pending: Amount,
    /// Balance from immature coinbase outputs.
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub immature: Amount,
}

/// Models the result of JSON-RPC method `getnewaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNewAddress(pub Address<NetworkUnchecked>);

/// Models the result of JSON-RPC method `getrawchangeaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetRawChangeAddress(pub Address<NetworkUnchecked>);

/// Models the result of JSON-RPC method `getreceivedbyaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetReceivedByAddress(
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub Amount
);

/// Models the result of JSON-RPC method `gettransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransaction {
    /// The transaction amount.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub amount: SignedAmount,   // Core returns -1 at times for some reason.
    /// The amount of the fee.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    #[serde(default, with = "bitcoin::amount::serde::as_sat::opt")]
    pub fee: Option<SignedAmount>,
    /// The number of confirmations.
    pub confirmations: i64, // Docs do not indicate what negative value means?
    /// The block hash.
    pub block_hash: Option<BlockHash>,
    /// The index of the transaction in the block that includes it.
    pub block_index: Option<u32>,
    /// The time in seconds since epoch (1 Jan 1970 GMT).
    pub block_time: Option<u32>,
    /// The transaction id.
    pub txid: Txid,
    /// The transaction time in seconds since epoch (1 Jan 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (1 Jan 1970 GMT).
    pub time_received: u32,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    pub bip125_replaceable: Bip125Replaceable,
    /// Transaction details.
    pub details: Vec<GetTransactionDetail>,
    /// The transaction, parsed from hex string.
    pub tx: Transaction,
}

/// Part of the `GetTransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionDetail {
    /// The bitcoin address involved in the transaction.
    pub address: Address<NetworkUnchecked>,
    /// The category, either 'send' or 'receive'.
    pub category: TransactionCategory,
    ///  The amount.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub amount: SignedAmount,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// the vout value.
    pub vout: u32,
    /// The amount of the fee.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    #[serde(default, with = "bitcoin::amount::serde::as_sat::opt")]
    pub fee: Option<SignedAmount>,
    /// If the transaction has been abandoned (inputs are respendable).
    ///
    /// Only available for the 'send' category of transactions.
    pub abandoned: Option<bool>,
}

/// Models the result of JSON-RPC method `getunconfirmedbalance`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetUnconfirmedBalance(
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub Amount
);

/// Models the result of JSON-RPC method `getwalletinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetWalletInfo {
    /// The wallet name.
    pub wallet_name: String,
    /// The wallet version.
    pub wallet_version: u32,
    /// The total confirmed balance of the wallet in BTC.
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub balance: Amount,
    /// The total unconfirmed balance of the wallet in BTC.
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub unconfirmed_balance: Amount,
    /// The total immature balance of the wallet in BTC.
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    pub immature_balance: Amount,
    /// The total number of transactions in the wallet
    pub tx_count: u32,
    /// The timestamp (seconds since Unix epoch) of the oldest pre-generated key in the key pool.
    pub keypool_oldest: u32,
    /// How many new keys are pre-generated (only counts external keys).
    pub keypool_size: u32,
    /// How many new keys are pre-generated for internal use (used for change outputs, only appears
    /// if the wallet is using this feature, otherwise external keys are used).
    pub keypool_size_hd_internal: u32,
    /// The timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked
    /// for transfers, or 0 if the wallet is locked.
    pub unlocked_until: u32,
    /// The transaction fee configuration.
    #[serde(with = "bitcoin::fee_rate::serde::as_sat_per_kwu::opt")]
    pub pay_tx_fee: Option<FeeRate>,
    /// The Hash160 of the HD seed (only present when HD is enabled).
    pub hd_seed_id: Option<hash160::Hash>,
    /// If privatekeys are disabled for this wallet (enforced watch-only wallet).
    pub private_keys_enabled: bool,
}

/// Models the result of JSON-RPC method `listaddressgroupings`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListAddressGroupings(pub Vec<Vec<ListAddressGroupingsItem>>);

/// List item type returned as part of `listaddressgroupings`.
// FIXME: The Core docs seem wrong, not sure what shape this should be?
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListAddressGroupingsItem {
    /// The bitcoin address.
    pub address: Address<NetworkUnchecked>,
    /// The amount.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub amount: Amount,
    /// The label.
    pub label: Option<String>,
}

/// Models the result of JSON-RPC method `listlabels`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListLabels(pub Vec<String>);

/// Models the result of JSON-RPC method `listlockunspent`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListLockUnspent(pub Vec<ListLockUnspentItem>);

/// List item returned as part of of `listlockunspent`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListLockUnspentItem {
    /// The transaction id locked.
    pub txid: Txid,
    /// The vout value.
    pub vout: u32,
}

/// Models the result of JSON-RPC method `listreceivedbyaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedByAddress(pub Vec<ListReceivedByAddressItem>);

/// List item returned as part of of `listreceivedbyaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedByAddressItem {
    /// Only returned if imported addresses were involved in transaction.
    pub involves_watch_only: bool,
    /// The receiving address.
    pub address: Address<NetworkUnchecked>,
    /// The total amount received by the address.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub amount: Amount,
    /// The number of confirmations of the most recent transaction included.
    pub confirmations: i64, // Docs do not indicate what negative value means?
    /// The label of the receiving address. The default label is "".
    pub label: String,
    /// The ids of transactions received with the address.
    pub txids: Vec<Txid>,
}

/// Models the result of JSON-RPC method `listsinceblock`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListSinceBlock {
    /// All the transactions.
    pub transactions: Vec<ListSinceBlockTransaction>,
    /// Only present if `include_removed=true`.
    ///
    /// Note: transactions that were re-added in the active chain will appear as-is in this array,
    /// and may thus have a positive confirmation count.
    pub removed: Vec<ListSinceBlockTransaction>,
    /// The hash of the block (target_confirmations-1) from the best block on the main chain.
    ///
    /// This is typically used to feed back into listsinceblock the next time you call it. So you
    /// would generally use a target_confirmations of say 6, so you will be continually
    /// re-notified of transactions until they've reached 6 confirmations plus any new ones.
    pub last_block: BlockHash,
}

/// Transaction list item, part of `ListSinceBlock`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]

// https://github.com/rust-bitcoin/rust-bitcoin/issues/3516
pub struct ListSinceBlockTransaction {
    /// The bitcoin address of the transaction.
    pub address: Option<Address<NetworkUnchecked>>,
    /// The transaction category.
    pub category: TransactionCategory,
    /// The amount in BTC.
    ///
    /// This is negative for the 'send' category, and for the 'move' category for moves outbound. It
    /// is positive for the 'receive' category, and for the 'move' category for inbound funds.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub amount: SignedAmount,
    /// The vout value.
    pub vout: u32,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub fee: SignedAmount,
    /// The number of confirmations for the transaction.
    ///
    /// Available for 'send' and 'receive' category of transactions. When it's < 0, it means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The block hash containing the transaction.
    ///
    /// Available for 'send' and 'receive' category of transactions.
    pub block_hash: BlockHash,
    /// The index of the transaction in the block that includes it.
    ///
    /// Available for 'send' and 'receive' category of transactions.
    pub block_index: u32,
    /// The block time in seconds since epoch (1 Jan 1970 GMT).
    pub block_time: u32,
    /// The transaction id.
    ///
    /// Available for 'send' and 'receive' category of transactions.
    pub txid: Option<Txid>,
    /// The transaction time in seconds since epoch (Jan 1 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (Jan 1 1970 GMT).
    ///
    /// Available for 'send' and 'receive' category of transactions.
    pub time_received: u32,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    pub bip125_replaceable: Bip125Replaceable,
    /// If the transaction has been abandoned (inputs are respendable).
    ///
    /// Only available for the 'send' category of transactions.
    pub abandoned: Option<bool>,
    /// If a comment is associated with the transaction.
    pub comment: Option<String>,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
}

/// Models the result of JSON-RPC method `listtransactions`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListTransactions(pub Vec<ListTransactionsItem>);

/// Transaction list item, part of `ListTransactions`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListTransactionsItem {
    /// The bitcoin address of the transaction.
    pub address: Address<NetworkUnchecked>,
    /// The transaction category.
    pub category: TransactionCategory,
    /// The amount.
    ///
    /// This is negative for the 'send' category, and is positive for the 'receive' category.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub amount: SignedAmount,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// The vout value.
    pub vout: u32,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub fee: SignedAmount,
    /// The number of confirmations for the transaction.
    ///
    /// Negative confirmations indicate the transaction conflicts with the block chain.
    pub confirmations: i64,
    /// Whether we consider the outputs of this unconfirmed transaction safe to spend.
    pub trusted: bool,
    /// The block hash containing the transaction.
    pub block_hash: BlockHash,
    /// The index of the transaction in the block that includes it.
    pub block_index: u32,
    /// The block time in seconds since epoch (1 Jan 1970 GMT).
    pub block_time: u32,
    /// The transaction id.
    pub txid: Txid,
    /// The transaction time in seconds since epoch (Jan 1 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (Jan 1 1970 GMT).
    pub time_received: u32,
    /// If a comment is associated with the transaction.
    pub comment: Option<String>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    pub bip125_replaceable: Bip125Replaceable,
    /// If the transaction has been abandoned (inputs are respendable).
    ///
    /// Only available for the 'send' category of transactions.
    pub abandoned: Option<bool>,
}

/// Models the result of JSON-RPC method `listunspent`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListUnspent(pub Vec<ListUnspentItem>);

/// Unspent transaction output, returned as part of `listunspent`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListUnspentItem {
    /// The transaction id.
    pub txid: Txid,
    /// The vout value.
    pub vout: u32,
    /// The bitcoin address of the transaction.
    pub address: Address<NetworkUnchecked>,
    /// The associated label, or "" for the default label.
    pub label: String,
    /// The script key.
    pub script_pubkey: ScriptBuf,
    /// The transaction amount.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub amount: SignedAmount,
    /// The number of confirmations.
    pub confirmations: u32, // Docs do not indicate what negative value means?
    /// The redeemScript if scriptPubKey is P2SH.
    pub redeem_script: Option<ScriptBuf>,
    /// Whether we have the private keys to spend this output.
    pub spendable: bool,
    /// Whether we know how to spend this output, ignoring the lack of keys.
    pub solvable: bool,
    /// Whether this output is considered safe to spend. Unconfirmed transactions from outside keys
    /// and unconfirmed replacement transactions are considered unsafe and are not eligible for
    /// spending by fundrawtransaction and sendtoaddress.
    pub safe: bool,
}

/// Models the result of JSON-RPC method `listwallets`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListWallets(pub Vec<String>);

/// Models the result of JSON-RPC method `loadwallet`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Vec<String>,
}

/// Models the result of JSON-RPC method `rescanblockchain`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RescanBlockchain {
    /// The block height where the rescan has started.
    pub start_height: u32,
    /// The height of the last rescanned block.
    pub stop_height: u32,
}

/// Models the result of JSON-RPC method `sendmany`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SendMany(pub Txid);

/// Models the result of JSON-RPC method `sendtoaddress`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SendToAddress {
    pub txid: Txid,
}

/// Models the result of JSON-RPC method `signmessage`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SignMessage(pub ecdsa::Signature);

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SignRawTransactionWithWallet {
    /// The raw transaction with signature(s).
    pub raw_transaction: Transaction,
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
    /// Script verification errors (if there are any).
    pub errors: Vec<SignErrorData>, // 'Data' suffix to differentiate this from a normal error type.
}

/// Returned as part of `signrawtransactionwithwallet`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SignErrorData {
    /// The txid of the previous transaction.
    pub txid: Txid,
    /// The index of the output to spent and used as input.
    pub vout: u32,
    /// The signature script.
    pub script_sig: ScriptBuf,
    /// Script sequence number.
    pub sequence: Sequence,
    /// Verification or signing error related to the input.
    pub error: String,
}

/// Models the result of JSON-RPC method `unloadwallet`.
///
/// Core version v0.21 onwards.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UnloadWallet {
    /// Warning messages, if any, related to unloading the wallet.
    // Changes from single string to vector in Core v25
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbt {
    /// The resulting PSBT.
    pub psbt: Psbt,
    /// Fee the resulting transaction pays.
    #[serde(default, with = "bitcoin::amount::serde::as_sat")]
    pub fee: SignedAmount,
    /// The position of the added change output, or -1.
    pub change_pos: u32,
}

/// Models the result of JSON-RPC method `walletprocesspsbt`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WalletProcessPsbt {
    /// The partially signed transaction.
    pub psbt: Psbt,
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
}
