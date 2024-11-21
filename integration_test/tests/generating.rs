// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Generating ==` section of the API docs.

use integration_test::{Node, NodeExt as _};

#[test]
fn generate_to_address() {
    const NBLOCKS: usize = 1;

    let node = Node::new_with_default_wallet();
    let address = node.client.new_address().expect("failed to get new address");
    let json = node.client.generate_to_address(NBLOCKS, &address).expect("generatetoaddress");
    json.into_model().unwrap();
}

// #[test]
// #[cfg(not(feature = "v19"))]
// fn generate() {
//     const NBLOCKS: usize = 100;

//     let node = Node::new_with_default_wallet();
//     let json = node.client.generate(NBLOCKS).expect("generate");
//     let model = json.into_model().unwrap();
//     assert_eq!(model.len(), NBLOCKS);
// }
