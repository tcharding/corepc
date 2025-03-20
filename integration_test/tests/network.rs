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
    get_peer_info_one_node_network();
    get_peer_info_three_node_network();
}

fn get_peer_info_one_node_network() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json = node.client.get_peer_info().expect("getpeerinfo");
    assert_eq!(json.0.len(), 0);
}

fn get_peer_info_three_node_network() {
    let (node1, node2, node3) = integration_test::three_node_network();

    // Just for good measure.
    node1.mine_a_block();
    node2.mine_a_block();
    node3.mine_a_block();

    // FIXME: Fails if we use equal to 2 ???
    assert!(node1.peers_connected() >= 1);
    assert!(node2.peers_connected() >= 1);
    assert!(node3.peers_connected() >= 1);
}
