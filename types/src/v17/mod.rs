// SPDX-License-Identifier: CC0-1.0

//! JSON-RPC types for `bitcoind v0.17.1`.
//!
//! These structs are shaped for the JSON data returned by the JSON-RPC API. They use stdlib types
//! (or custom types) and where necessary implement an `into_model` function to convert the type to
//! a [`crate::model`] type of the same name. The types in this module are version specific, the
//! types in the `model` module are version nonspecific and are strongly typed using `rust-bitcoin`.
//!
//! ## Key:
//!
//! - `[ ]` Not yet done.
//! - `[i]` Implemented but not yet tested (includes `into_model`).
//! - `[x]` Implemented _and_  tested.
//! - `[-]` Intentionally not done, typically for one of the following reasons:
//!           - Method does not return anything.
//!           - Method returns a simple type (e.g. bool or integer).
//!           - Method is deprecated.
// TODO: After all the `[i]` is gone (ie testing done) remove the backticks.
//!
//! **== Blockchain ==**
//! - `[x]` `getbestblockhash`
//! - `[x]` `getblock "blockhash" ( verbosity ) `
//! - `[x]` `getblockchaininfo`
//! - `[x]` `getblockcount`
//! - `[x]` `getblockhash height`
//! - `[x]` `getblockheader "hash" ( verbose )`
//! - `[x]` `getblockstats hash_or_height ( stats )`
//! - `[x]` `getchaintips`
//! - `[x]` `getchaintxstats ( nblocks blockhash )`
//! - `[x]` `getdifficulty`
//! - `[i]` `getmempoolancestors txid (verbose)`
//! - `[i]` `getmempooldescendants txid (verbose)`
//! - `[i]` `getmempoolentry txid`
//! - `[i]` `getmempoolinfo`
//! - `[i]` `getrawmempool ( verbose )`
//! - `[i]` `gettxout "txid" n ( include_mempool )`
//! - `[i]` `gettxoutproof ["txid",...] ( blockhash )`
//! - `[i]` `gettxoutsetinfo`
//! - `[-]` `preciousblock "blockhash"`
//! - `[-]` `pruneblockchain`
//! - `[-]` `savemempool`
//! - `[-]` `scantxoutset <action> ( <scanobjects> )`
//! - `[-]` `verifychain ( checklevel nblocks )`
//! - `[i]` `verifytxoutproof "proof"`
//!
//! **== Control ==**
//! - `[x]` `getmemoryinfo ("mode")`
//! - `[-]` `help ( "command" )`
//! - `[x]` `logging ( <include> <exclude> )`
//! - `[x]` `stop`
//! - `[x]` `uptime`
//!
//! **== Generating ==**
//! - `[x]` `generate nblocks ( maxtries )`
//! - `[x]` `generatetoaddress nblocks address (maxtries)`
//!
//! **== Mining ==**
//! - `[ ]` `getblocktemplate ( TemplateRequest )`
//! - `[ ]` `getmininginfo`
//! - `[ ]` `getnetworkhashps ( nblocks height )`
//! - `[ ]` `prioritisetransaction <txid> <dummy value> <fee delta>`
//! - `[ ]` `submitblock "hexdata"  ( "dummy" )`
//!
//! **== Network ==**
//! - `[-]` `addnode "node" "add|remove|onetry"`
//! - `[-]` `clearbanned`
//! - `[-]` `disconnectnode "[address]" [nodeid]`
//! - `[x]` `getaddednodeinfo ( "node" )`
//! - `[-]` `getconnectioncount`
//! - `[x]` `getnettotals`
//! - `[x]` `getnetworkinfo`
//! - `[x]` `getpeerinfo`
//! - `[-]` `listbanned`
//! - `[-]` `ping`
//! - `[-]` `setban "subnet" "add|remove" (bantime) (absolute)`
//! - `[-]` `setnetworkactive true|false`
//!
//! **== Rawtransactions ==**
//! - `[ ]` `combinepsbt ["psbt",...]`
//! - `[ ]` `combinerawtransaction ["hexstring",...]`
//! - `[ ]` `converttopsbt "hexstring" ( permitsigdata iswitness )`
//! - `[ ]` `createpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )`
//! - `[ ]` `createrawtransaction [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )`
//! - `[ ]` `decodepsbt "psbt"`
//! - `[ ]` `decoderawtransaction "hexstring" ( iswitness )`
//! - `[ ]` `decodescript "hexstring"`
//! - `[ ]` `finalizepsbt "psbt" ( extract )`
//! - `[ ]` `fundrawtransaction "hexstring" ( options iswitness )`
//! - `[ ]` `getrawtransaction "txid" ( verbose "blockhash" )`
//! - `[i]` `sendrawtransaction "hexstring" ( allowhighfees )`
//! - `[ ]` `signrawtransaction "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] ["privatekey1",...] sighashtype )`
//! - `[ ]` `signrawtransactionwithkey "hexstring" ["privatekey1",...] ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )`
//! - `[ ]` `testmempoolaccept ["rawtxs"] ( allowhighfees )`
//!
//! **== Util ==**
//! - `[ ]` `createmultisig nrequired ["key",...] ( "address_type" )`
//! - `[ ]` `estimatesmartfee conf_target ("estimate_mode")`
//! - `[ ]` `signmessagewithprivkey "privkey" "message"`
//! - `[ ]` `validateaddress "address"`
//! - `[ ]` `verifymessage "address" "signature" "message"`
//!
//! **== Wallet ==**
//! - `[-]` `abandontransaction "txid"`
//! - `[-]` `abortrescan`
//! - `[x]` `addmultisigaddress nrequired ["key",...] ( "label" "address_type" )`
//! - `[-]` `backupwallet "destination"`
//! - `[x]` `bumpfee "txid" ( options ) `
//! - `[x]` `createwallet "wallet_name" ( disable_private_keys )`
//! - `[x]` `dumpprivkey "address"`
//! - `[x]` `dumpwallet "filename"`
//! - `[-]` `encryptwallet "passphrase"`
//! - `[-]` `getaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - `[-]` `getaccountaddress (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - `[-]` `getaddressbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - `[x]` `getaddressesbylabel "label"`
//! - `[x]` `getaddressinfo "address"`
//! - `[x]` `getbalance ( "(dummy)" minconf include_watchonly )`
//! - `[x]` `getnewaddress ( "label" "address_type" )`
//! - `[x]` `getrawchangeaddress ( "address_type" )`
//! - `[-]` `getreceivedbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - `[i]` `getreceivedbyaddress "address" ( minconf )`
//! - `[x]` `gettransaction "txid" ( include_watchonly )`
//! - `[i]` `getunconfirmedbalance`
//! - `[i]` `getwalletinfo`
//! - `[-]` `importaddress "address" ( "label" rescan p2sh )`
//! - `[-]` `importmulti "requests" ( "options" )`
//! - `[-]` `importprivkey "privkey" ( "label" ) ( rescan )`
//! - `[-]` `importprunedfunds`
//! - `[-]` `importpubkey "pubkey" ( "label" rescan )`
//! - `[-]` `importwallet "filename"`
//! - `[-]` `keypoolrefill ( newsize )`
//! - `[-]` `listaccounts (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - `[i]` `listaddressgroupings`
//! - `[i]` `listlabels ( "purpose" )`
//! - `[i]` `listlockunspent`
//! - `[-]` `listreceivedbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - `[i]` `listreceivedbyaddress ( minconf include_empty include_watchonly address_filter )`
//! - `[i]` `listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )`
//! - `[i]` `listtransactions (label count skip include_watchonly)`
//! - `[i]` `listunspent ( minconf maxconf  ["addresses",...] [include_unsafe] [query_options])`
//! - `[i]` `listwallets`
//! - `[x]` `loadwallet "filename"`
//! - `[-]` `lockunspent unlock ([{"txid":"txid","vout":n},...])`
//! - `[-]` `move (Deprecated, will be removed in V0.18. To use this command, start bitcboind with -deprecatedrpc=accounts)`
//! - `[-]` `removeprunedfunds "txid"`
//! - `[x]` `rescanblockchain ("start_height") ("stop_height")`
//! - `[-]` `sendfrom (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - `[i]` `sendmany "" {"address":amount,...} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode")`
//! - `[x]` `sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode")`
//! - `[-]` `setaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)`
//! - `[-]` `sethdseed ( "newkeypool" "seed" )`
//! - `[-]` `settxfee amount`
//! - `[i]` `signmessage "address" "message"`
//! - `[i]` `signrawtransactionwithwallet "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )`
//! - `[-]` `unloadwallet ( "wallet_name" )`
//! - `[i]` `walletcreatefundedpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable ) ( options bip32derivs )`
//! - `[-]` `walletlock`
//! - `[-]` `walletpassphrase "passphrase" timeout`
//! - `[-]` `walletpassphrasechange "oldpassphrase" "newpassphrase"`
//! - `[i]` `walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )`
//!
//! **== Zmq ==**
//! - `[i]` `getzmqnotifications`

// JSON-RPC types by API section.
mod blockchain;
mod control;
mod generating;
mod mining;
mod network;
mod raw_transactions;
mod util;
mod wallet;
mod zmq;

#[doc(inline)]
pub use self::{
    blockchain::{
        Bip9Softfork, Bip9SoftforkStatus, ChainTips, ChainTipsStatus, GetBestBlockHash,
        GetBlockCount, GetBlockHash, GetBlockHeader, GetBlockHeaderVerbose, GetBlockStats,
        GetBlockVerbosityOne, GetBlockVerbosityZero, GetBlockchainInfo, GetChainTips,
        GetChainTxStats, GetDifficulty, GetMempoolAncestors, GetMempoolAncestorsVerbose,
        GetMempoolDescendants, GetMempoolDescendantsVerbose, GetMempoolEntry, GetMempoolInfo,
        GetRawMempool, GetRawMempoolVerbose, GetTxOut, GetTxOutProof, GetTxOutSetInfo,
        MempoolEntry, MempoolEntryFees, ScriptPubkey, Softfork, SoftforkReject, VerifyTxOutProof,
    },
    control::{GetMemoryInfoStats, Locked, Logging, Uptime},
    generating::{Generate, GenerateToAddress},
    network::{
        AddedNode, AddedNodeAddress, Banned, GetAddedNodeInfo, GetNetTotals, GetNetworkInfo,
        GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork, GetPeerInfo, ListBanned,
        PeerInfo, UploadTarget,
    },
    raw_transactions::SendRawTransaction,
    wallet::{
        AddMultisigAddress, AddressInformation, BumpFee, CreateWallet, DumpPrivKey, DumpWallet,
        GetAddressInfo, GetAddressInfoEmbedded, GetAddressInfoLabel, GetAddressesByLabel,
        GetBalance, GetNewAddress, GetRawChangeAddress, GetReceivedByAddress, GetTransaction,
        GetTransactionDetail, GetUnconfirmedBalance, GetWalletInfo, ListAddressGroupings,
        ListAddressGroupingsItem, ListLabels, ListLockUnspent, ListLockUnspentItem,
        ListReceivedByAddress, ListReceivedByAddressItem, ListSinceBlock,
        ListSinceBlockTransaction, ListTransactions, ListTransactionsItem, ListUnspent,
        ListUnspentItem, ListWallets, LoadWallet, RescanBlockchain, SendMany, SendToAddress,
        SignErrorData, SignMessage, SignRawTransactionWithWallet, TransactionCategory,
        WalletCreateFundedPsbt, WalletProcessPsbt,
    },
};
