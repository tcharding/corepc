// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v0.21`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, Txid};

use crate::client_sync::into_json;
use crate::types::v21::*;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use crate::client_sync::v17::AddressType;

crate::define_jsonrpc_minreq_client!("v21");

// == Blockchain ==
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getblockcount!();
crate::impl_client_v17__getblockhash!();
crate::impl_client_v17__getblockheader!();
crate::impl_client_v17__getblockstats!();
crate::impl_client_v17__getchaintips!();
crate::impl_client_v17__getchaintxstats!();
crate::impl_client_v17__getdifficulty!();
crate::impl_client_v19__getmempoolancestors!();
crate::impl_client_v19__getmempooldescendants!();
crate::impl_client_v19__getmempoolentry!();
crate::impl_client_v17__getmempoolinfo!();
crate::impl_client_v17__getrawmempool!();
crate::impl_client_v17__gettxout!();
crate::impl_client_v17__gettxoutproof!();
crate::impl_client_v17__gettxoutsetinfo!();
crate::impl_client_v17__verifytxoutproof!();

// == Control ==
crate::impl_client_v17__stop!();

// == Generating ==
crate::impl_client_v17__generatetoaddress!();
crate::impl_client_v17__invalidateblock!();

// == Network ==
crate::impl_client_v17__getnetworkinfo!();
crate::impl_client_check_expected_server_version!({ [210200] });

// == Rawtransactions ==
crate::impl_client_v17__sendrawtransaction!();

// == Wallet ==
crate::impl_client_v17__createwallet!();
crate::impl_client_v17__loadwallet!();
crate::impl_client_v17__getnewaddress!();
crate::impl_client_v17__getbalance!();
crate::impl_client_v19__getbalances!();
crate::impl_client_v17__sendtoaddress!();
crate::impl_client_v17__gettransaction!();
