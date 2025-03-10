// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Control ==` section of the API docs.

use integration_test::{Node, NodeExt as _};

#[test]
fn get_memory_info() {
    let node = Node::new_no_wallet();
    let _ = node.client.get_memory_info().expect("getmemoryinfo");
}

#[cfg(not(feature = "v17"))]
#[test]
fn get_rpc_info() {
    let node = Node::new_no_wallet();
    let _ = node.client.get_rpc_info().expect("getrpcinfo");
}

#[test]
fn help() {
    let node = Node::new_no_wallet();
    // There is no json object for `stop`, we just return a string.
    let _ = node.client.help().expect("help");
}

#[test]
fn logging() {
    let node = Node::new_no_wallet();
    let _ = node.client.logging().expect("logging");
}

#[test]
fn stop() {
    let node = Node::new_no_wallet();
    // There is no json object for `stop`, we just return a string.
    let _ = node.client.stop().expect("stop");
}

#[test]
fn uptime() {
    let node = Node::new_no_wallet();
    let _ = node.client.uptime().expect("uptime");
}
