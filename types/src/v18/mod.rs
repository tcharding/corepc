// SPDX-License-Identifier: CC0-1.0

// This module is currently just copy of v17. As such it introduces the following TODOs:
//
// - Check all items marked with `-` still should be omitted (currently just copied from v17).
// - Work out how to solve the problem that the docs on the re-exported types are for v17. We
// probably have to write a script to pull the v18 docs and check them against the v17 docs for
// differences.
// - Double check that nothing marked `x` has a feature gated test (i.e. ensure tested).

//! Structs with standard types.
//!
//! These structs model the types returned by the JSON-RPC API and use stdlib types (or custom
//! types) and are specific to a specific to Bitcoin Core `v0.18.1`.
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
//! ** == Blockchain ==**
//! - `[x]` `getbestblockhash`
//! - `[x]` `getblock "blockhash" ( verbosity )`
//! - `[x]` `getblockchaininfo`
//! - `[x]` `getblockcount`
//! - `[x]` `getblockhash height`
//! - `[x]` `getblockheader "blockhash" ( verbose )`
//! - `[x]` `getblockstats hash_or_height ( stats )`
//! - `[x]` `getchaintips`
//! - `[x]` `getchaintxstats ( nblocks "blockhash" )`
//! - `[x]` `getdifficulty`
//! - `[i]` `getmempoolancestors "txid" ( verbose )`
//! - `[i]` `getmempooldescendants "txid" ( verbose )`
//! - `[i]` `getmempoolentry "txid"`
//! - `[i]` `getmempoolinfo`
//! - `[i]` `getrawmempool ( verbose )`
//! - `[i]` `gettxout "txid" n ( include_mempool )`
//! - `[i]` `gettxoutproof ["txid",...] ( "blockhash" )`
//! - `[i]` `gettxoutsetinfo`
//! - `[-]` `preciousblock "blockhash"`
//! - `[-]` `pruneblockchain height`
//! - `[-]` `savemempool`
//! - `[-]` `scantxoutset "action" [scanobjects,...]`
//! - `[-]` `verifychain ( checklevel nblocks )`
//! - `[i]` `verifytxoutproof "proof"`
//!
//! ** == Control ==**
//! - `[i]` `getmemoryinfo ( "mode" )`
//! - `[ ]` `getrpcinfo`
//! - `[-]` `help ( "command" )`
//! - `[x]` `logging ( ["include_category",...] ["exclude_category",...] )`
//! - `[x]` `stop`
//! - `[x]` `uptime`
//!
//! ** == Generating ==**
//! - `[x]` `generate nblocks ( maxtries )`
//! - `[x]` `generatetoaddress nblocks "address" ( maxtries )`
//!
//! ** == Mining ==**
//! - `[ ]` `getblocktemplate "template_request"`
//! - `[ ]` `getmininginfo`
//! - `[ ]` `getnetworkhashps ( nblocks height )`
//! - `[ ]` `prioritisetransaction "txid" ( dummy ) fee_delta`
//! - `[ ]` `submitblock "hexdata" ( "dummy" )`
//! - `[ ]` `submitheader "hexdata"`
//!
//! ** == Network ==**
//! - `[-]` `addnode "node" "command"`
//! - `[-]` `clearbanned`
//! - `[-]` `disconnectnode ( "address" nodeid )`
//! - `[x]` `getaddednodeinfo ( "node" )`
//! - `[-]` `getconnectioncount`
//! - `[x]` `getnettotals`
//! - `[x]` `getnetworkinfo`
//! - `[ ]` `getnodeaddresses ( count )`
//! - `[x]` `getpeerinfo`
//! - `[-]` `listbanned`
//! - `[-]` `ping`
//! - `[-]` `setban "subnet" "command" ( bantime absolute )`
//! - `[-]` `setnetworkactive state`
//!
//! ** == Rawtransactions ==**
//! - `[ ]` `analyzepsbt "psbt"`
//! - `[ ]` `combinepsbt ["psbt",...]`
//! - `[ ]` `combinerawtransaction ["hexstring",...]`
//! - `[ ]` `converttopsbt "hexstring" ( permitsigdata iswitness )`
//! - `[ ]` `createpsbt [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount},{"data":"hex"},...] ( locktime replaceable )`
//! - `[ ]` `createrawtransaction [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount},{"data":"hex"},...] ( locktime replaceable )`
//! - `[ ]` `decodepsbt "psbt"`
//! - `[ ]` `decoderawtransaction "hexstring" ( iswitness )`
//! - `[ ]` `decodescript "hexstring"`
//! - `[ ]` `finalizepsbt "psbt" ( extract )`
//! - `[ ]` `fundrawtransaction "hexstring" ( options iswitness )`
//! - `[ ]` `getrawtransaction "txid" ( verbose "blockhash" )`
//! - `[ ]` `joinpsbts ["psbt",...]`
//! - `[i]` `sendrawtransaction "hexstring" ( allowhighfees )`
//! - `[ ]` `signrawtransactionwithkey "hexstring" ["privatekey",...] ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )`
//! - `[ ]` `testmempoolaccept ["rawtx",...] ( allowhighfees )`
//! - `[ ]` `utxoupdatepsbt "psbt"`
//!
//! ** == Util ==**
//! - `[ ]` `createmultisig nrequired ["key",...] ( "address_type" )`
//! - `[ ]` `deriveaddresses "descriptor" ( range )`
//! - `[ ]` `estimatesmartfee conf_target ( "estimate_mode" )`
//! - `[ ]` `getdescriptorinfo "descriptor"`
//! - `[ ]` `signmessagewithprivkey "privkey" "message"`
//! - `[ ]` `validateaddress "address"`
//! - `[ ]` `verifymessage "address" "signature" "message"`
//!
//! ** == Wallet ==**
//! - `[-]` `abandontransaction "txid"`
//! - `[-]` `abortrescan`
//! - `[x]` `addmultisigaddress nrequired ["key",...] ( "label" "address_type" )`
//! - `[-]` `backupwallet "destination"`
//! - `[x]` `bumpfee "txid" ( options )`
//! - `[x]` `createwallet "wallet_name" ( disable_private_keys blank )`
//! - `[x]` `dumpprivkey "address"`
//! - `[x]` `dumpwallet "filename"`
//! - `[-]` `encryptwallet "passphrase"`
//! - `[x]` `getaddressesbylabel "label"`
//! - `[x]` `getaddressinfo "address"`
//! - `[x]` `getbalance ( "dummy" minconf include_watchonly )`
//! - `[x]` `getnewaddress ( "label" "address_type" )`
//! - `[x]` `getrawchangeaddress ( "address_type" )`
//! - `[i]` `getreceivedbyaddress "address" ( minconf )`
//! - `[ ]` `getreceivedbylabel "label" ( minconf )`
//! - `[x]` `gettransaction "txid" ( include_watchonly )`
//! - `[i]` `getunconfirmedbalance`
//! - `[i]` `getwalletinfo`
//! - `[-]` `importaddress "address" ( "label" rescan p2sh )`
//! - `[-]` `importmulti "requests" ( "options" )`
//! - `[-]` `importprivkey "privkey" ( "label" rescan )`
//! - `[-]` `importprunedfunds "rawtransaction" "txoutproof"`
//! - `[-]` `importpubkey "pubkey" ( "label" rescan )`
//! - `[-]` `importwallet "filename"`
//! - `[-]` `keypoolrefill ( newsize )`
//! - `[i]` `listaddressgroupings`
//! - `[i]` `listlabels ( "purpose" )`
//! - `[i]` `listlockunspent`
//! - `[i]` `listreceivedbyaddress ( minconf include_empty include_watchonly "address_filter" )`
//! - `[ ]` `listreceivedbylabel ( minconf include_empty include_watchonly )`
//! - `[i]` `listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )`
//! - `[i]` `listtransactions ( "label" count skip include_watchonly )`
//! - `[i]` `listunspent ( minconf maxconf ["address",...] include_unsafe query_options )`
//! - `[ ]` `listwalletdir`
//! - `[i]` `listwallets`
//! - `[x]` `loadwallet "filename"`
//! - `[-]` `lockunspent unlock ( [{"txid":"hex","vout":n},...] )`
//! - `[i]` `removeprunedfunds "txid"`
//! - `[x]` `rescanblockchain ( start_height stop_height )`
//! - `[i]` `sendmany "" {"address":amount} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode" )`
//! - `[x]` `sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode" )`
//! - `[i]` `sethdseed ( newkeypool "seed" )`
//! - `[ ]` `setlabel "address" "label"`
//! - `[ ]` `settxfee amount`
//! - `[i]` `signmessage "address" "message"`
//! - `[i]` `signrawtransactionwithwallet "hexstring" ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )`
//! - `[-]` `unloadwallet ( "wallet_name" )`
//! - `[i]` `walletcreatefundedpsbt [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount},{"data":"hex"},...] ( locktime options bip32derivs )`
//! - `[-]` `walletlock`
//! - `[-]` `walletpassphrase "passphrase" timeout`
//! - `[-]` `walletpassphrasechange "oldpassphrase" "newpassphrase"`
//! - `[-]` `walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )`
//!
//! ** == Zmq ==**`
//! - `[i]` `getzmqnotifications`

#[doc(inline)]
pub use crate::v17::{
    AddMultisigAddress, AddedNode, AddedNodeAddress, AddressInformation, Banned, Bip9Softfork,
    Bip9SoftforkStatus, BumpFee, ChainTips, ChainTipsStatus, CreateWallet, DumpPrivKey, DumpWallet,
    Generate, GenerateToAddress, GetAddedNodeInfo, GetAddressInfo, GetAddressInfoEmbedded,
    GetAddressInfoLabel, GetAddressesByLabel, GetBalance, GetBestBlockHash, GetBlockCount,
    GetBlockHash, GetBlockHeader, GetBlockHeaderVerbose, GetBlockStats, GetBlockVerbosityOne,
    GetBlockVerbosityZero, GetBlockchainInfo, GetChainTips, GetChainTxStats, GetDifficulty,
    GetMemoryInfoStats, GetMempoolAncestors, GetMempoolAncestorsVerbose, GetMempoolDescendants,
    GetMempoolDescendantsVerbose, GetMempoolEntry, GetMempoolInfo, GetNetTotals, GetNetworkInfo,
    GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork, GetNewAddress, GetPeerInfo,
    GetRawChangeAddress, GetRawMempool, GetRawMempoolVerbose, GetReceivedByAddress, GetTransaction,
    GetTransactionDetail, GetTxOut, GetTxOutProof, GetTxOutSetInfo, GetUnconfirmedBalance,
    GetWalletInfo, ListAddressGroupings, ListAddressGroupingsItem, ListBanned, ListLabels,
    ListLockUnspent, ListLockUnspentItem, ListReceivedByAddress, ListReceivedByAddressItem,
    ListSinceBlock, ListSinceBlockTransaction, ListTransactions, ListTransactionsItem, ListUnspent,
    ListUnspentItem, ListWallets, LoadWallet, Locked, Logging, MempoolEntry, MempoolEntryFees,
    PeerInfo, RescanBlockchain, ScriptPubkey, SendMany, SendRawTransaction, SendToAddress,
    SignErrorData, SignMessage, SignRawTransactionWithWallet, Softfork, SoftforkReject,
    TransactionCategory, UploadTarget, Uptime, VerifyTxOutProof, WalletCreateFundedPsbt,
    WalletProcessPsbt,
};
