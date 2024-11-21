// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Control ==` section of the API docs.

#![cfg(any(feature = "0_17_1", feature = "0_18_1"))]

use integration_test::{Node, NodeExt as _};

#[test]
fn get_memory_info() {
    let node = Node::new_no_wallet();
    // There is no model for `getmemoryinfo`, just check we can make the call.
    let _ = node.client.get_memory_info().expect("getmemoryinfo");
}

#[test]
fn logging() {
    let node = Node::new_no_wallet();
    // There is no model for `logging`, just check we can make the call.
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
    // There is no json object for `stop`, we just return a int.
    let _ = node.client.uptime().expect("uptime");
}
