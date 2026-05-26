// SPDX-License-Identifier: CC0-1.0

//! Tests derived from Bitcoin Core rpc_misc.py and rpc_uptime.py

use bitcoind::vtype::*;
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

#[test]
fn get_memory_info_has_locked_key() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let json: GetMemoryInfoStats = node.client.get_memory_info().unwrap();

    assert!(json.0.contains_key("locked"));

    let locked = &json.0["locked"];
    assert_eq!(locked.used + locked.free, locked.total);
}

#[test]
#[cfg(not(feature = "v18_and_below"))]
fn get_rpc_info_active_commands_contains_self() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);

    let json: GetRpcInfo = node.client.get_rpc_info().unwrap();

    assert!(!json.active_commands.is_empty());
    assert!(json.active_commands.iter().any(|c| c.method == "getrpcinfo"));
}

#[test]
#[cfg(not(feature = "v20_and_below"))]
fn get_index_info_has_txindex() {
    let node = BitcoinD::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let height = node.client.get_block_count().unwrap().0;
    let json: GetIndexInfo = node.client.get_index_info().unwrap();

    let txindex = json.0.get("txindex").unwrap();
    assert!(txindex.best_block_height <= height as u32);
}
