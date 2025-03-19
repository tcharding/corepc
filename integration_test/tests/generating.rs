// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Generating ==` section of the API docs.

use integration_test::{Node, NodeExt as _, Wallet};

#[test]
// The `generate` method deprecated in Core v18 and was removed in v19.
#[cfg(feature = "v17")]
fn generate() {
    const NBLOCKS: usize = 10;

    let node = Node::with_wallet(Wallet::Default, &[]);
    let _ = node.client.generate(NBLOCKS).expect("generate");
}

#[test]
fn generate_to_address() {
    const NBLOCKS: usize = 1;

    let node = Node::with_wallet(Wallet::Default, &[]);

    let address = node.client.new_address().expect("failed to get new address");
    let json = node.client.generate_to_address(NBLOCKS, &address).expect("generatetoaddress");
    json.into_model().unwrap();
}

#[test]
fn invalidate_block() {
    const NBLOCKS: usize = 1;

    let node = Node::with_wallet(Wallet::Default, &[]);

    let address = node.client.new_address().expect("failed to get new address");
    let old_best_block = node.client.get_best_block_hash().expect("getbestblockhash").into_model().unwrap().0;
    node.client.generate_to_address(NBLOCKS, &address).expect("generatetoaddress").into_model().unwrap();

    let new_best_block = node.client.get_best_block_hash().expect("getbestblockhash").into_model().unwrap().0;
    assert_ne!(old_best_block, new_best_block);

    node.client.invalidate_block(new_best_block).expect("invalidateblock");
    let best_block = node.client.get_best_block_hash().expect("getbestblockhash").into_model().unwrap().0;
    assert_eq!(old_best_block, best_block);
}

// #[test]
// #[cfg(not(feature = "v19"))]
// fn generate() {
//     const NBLOCKS: usize = 100;

//     let node = Node::with_wallet_with_default_wallet();
//     let json = node.client.generate(NBLOCKS).expect("generate");
//     let model = json.into_model().unwrap();
//     assert_eq!(model.len(), NBLOCKS);
// }
