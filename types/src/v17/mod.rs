// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `v0.17`.
//!
//! These structs are shaped for the JSON data returned by the JSON-RPC API. They use stdlib types
//! (or custom types) and where necessary implement an `into_model` function to convert the type to
//! a [`crate::model`] type of the same name. The types in this module are version specific. The
//! types in the `model` module are version nonspecific and are strongly typed using `rust-bitcoin`.
//!
//! ### Method name and implementation status
//!
//! Every JSON-RPC method supported by this version of Bitcoin Core is listed below along with its
//! current implementation status.
//!
//! <details>
//! <summary> Methods from the == Blockchain == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getbestblockhash                   | done            |
//! | getblock                           | done            |
//! | getblockchaininfo                  | done            |
//! | getblockcount                      | done            |
//! | getblockhash                       | done            |
//! | getblockheader                     | done            |
//! | getblockstats                      | done            |
//! | getchaintips                       | done            |
//! | getchaintxstats                    | done            |
//! | getdifficulty                      | done            |
//! | getmempoolancestors                | done (untested) |
//! | getmempooldescendants              | done (untested) |
//! | getmempoolentry                    | done (untested) |
//! | getmempoolinfo                     | done (untested) |
//! | getrawmempool                      | done (untested) |
//! | gettxout                           | done (untested) |
//! | gettxoutproof                      | done (untested) |
//! | gettxoutsetinfo                    | done (untested) |
//! | preciousblock                      | omitted         |
//! | pruneblockchain                    | omitted         |
//! | savemempool                        | omitted         |
//! | scantxoutset                       | omitted         |
//! | verifychain                        | omitted         |
//! | verifytxoutproof                   | done (untested) |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Control == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getmemoryinfo                      | done            |
//! | help                               | omitted         |
//! | logging                            | done            |
//! | stop                               | omitted         |
//! | uptime                             | omitted         |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Generating == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | generate                           | done (untested) |
//! | generatetoaddress                  | done            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Mining == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getblocktemplate                   | todo            |
//! | getmininginfo                      | todo            |
//! | getnetworkhashps                   | todo            |
//! | prioritisetransaction              | todo            |
//! | submitblock                        | todo            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Network == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | addnode                            | omitted         |
//! | clearbanned                        | omitted         |
//! | disconnectnode                     | omitted         |
//! | getaddednodeinfo                   | done            |
//! | getconnectioncount                 | omitted         |
//! | getnettotals                       | done            |
//! | getnetworkinfo                     | done            |
//! | getpeerinfo                        | done            |
//! | listbanned                         | omitted         |
//! | ping                               | omitted         |
//! | setban                             | omitted         |
//! | setnetworkactive                   | omitted         |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Rawtransactions == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | combinepsbt                        | todo            |
//! | combinerawtransaction              | todo            |
//! | converttopsbt                      | todo            |
//! | createpsbt                         | todo            |
//! | createrawtransaction               | todo            |
//! | decodepsbt                         | todo            |
//! | decoderawtransaction               | todo            |
//! | decodescript                       | todo            |
//! | finalizepsbt                       | todo            |
//! | fundrawtransaction                 | todo            |
//! | getrawtransaction                  | todo            |
//! | sendrawtransaction                 | done (untested) |
//! | signrawtransaction                 | todo            |
//! | signrawtransactionwithkey          | todo            |
//! | testmempoolaccept                  | todo            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | createmultisig                     | omitted         |
//! | estimatesmartfee                   | omitted         |
//! | signmessagewithprivkey             | omitted         |
//! | validateaddress                    | omitted         |
//! | verifymessage                      | omitted         |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Wallet == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | abandontransaction                 | omitted         |
//! | abortrescan                        | omitted         |
//! | addmultisigaddress                 | done (untested) |
//! | backupwallet                       | omitted         |
//! | bumpfee                            | done            |
//! | createwallet                       | done            |
//! | dumpprivkey                        | done            |
//! | dumpwallet                         | done            |
//! | encryptwallet                      | omitted         |
//! | getaccount                         | omitted         |
//! | getaccountaddress                  | omitted         |
//! | getaddressbyaccount                | omitted         |
//! | getaddressesbylabel                | done            |
//! | getaddressinfo                     | done (untested) |
//! | getbalance                         | done            |
//! | getnewaddress                      | done            |
//! | getrawchangeaddress                | done            |
//! | getreceivedbyaccount               | omitted         |
//! | getreceivedbyaddress               | done            |
//! | gettransaction                     | done            |
//! | getunconfirmedbalance              | done (untested) |
//! | getwalletinfo                      | done (untested) |
//! | importaddress                      | omitted         |
//! | importmulti                        | omitted         |
//! | importprivkey                      | omitted         |
//! | importprunedfunds                  | omitted         |
//! | importpubkey                       | omitted         |
//! | importwallet                       | omitted         |
//! | keypoolrefill                      | omitted         |
//! | listaccounts                       | omitted         |
//! | listaddressgroupings               | done (untested) |
//! | listlabels                         | done (untested) |
//! | listlockunspent                    | done (untested) |
//! | listreceivedbyaccount              | omitted         |
//! | listreceivedbyaddress              | done (untested) |
//! | listsinceblock                     | done (untested) |
//! | listtransactions                   | done (untested) |
//! | listunspent                        | done (untested) |
//! | listwallets                        | done (untested) |
//! | loadwallet                         | done            |
//! | lockunspent                        | omitted         |
//! | move                               | omitted         |
//! | removeprunedfunds                  | omitted         |
//! | rescanblockchain                   | done (untested) |
//! | sendfrom                           | omitted         |
//! | sendmany                           | done (untested) |
//! | sendtoaddress                      | done            |
//! | setaccount                         | omitted         |
//! | sethdseed                          | omitted         |
//! | settxfee                           | omitted         |
//! | signmessage                        | done (untested) |
//! | signrawtransactionwithwallet       | done (untested) |
//! | unloadwallet                       | omitted         |
//! | walletcreatefundedpsbt             | done (untested) |
//! | walletlock                         | omitted         |
//! | walletpassphrase                   | omitted         |
//! | walletpassphrasechange             | omitted         |
//! | walletprocesspsbt                  | done (untested) |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Zmq == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getzmqnotifications                | done (untested) |
//!
//! </details>
//!
//!
//! **Items marked omitted were omitted because:**
//!
//! - Method does not return anything.
//! - Method returns a simple type (e.g. bool or integer).
//! - Method is deprecated.

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
    zmq::GetZmqNotifications,
};
