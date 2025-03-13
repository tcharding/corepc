// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Network ==` section of the API docs.

#![cfg(any(feature = "0_17_1", feature = "0_18_1"))]

use integration_test::{Node, NodeExt as _, Wallet};

#[test]
fn get_added_node_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _ = node.client.get_added_node_info().expect("getaddednodeinfo");
}

#[test]
fn get_net_totals() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _ = node.client.get_net_totals().expect("getnettotals");
}

#[test]
fn get_network_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json = node.client.get_network_info().expect("getnetworkinfo");
    assert!(json.into_model().is_ok());

    // Server version is returned as part of the getnetworkinfo method.
    node.client.check_expected_server_version().expect("unexpected version");
}

#[test]
fn get_peer_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _ = node.client.get_peer_info().expect("getpeerinfo");
}
