// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Blockchain ==` section of the API docs.

use integration_test::{Node, NodeExt as _};

// FIXME: Do we need this?
fn best_block_hash() -> bitcoin::BlockHash {
    let node = Node::new_no_wallet();
    node.client.best_block_hash().expect("best_block_hash failed")
}

#[test]
fn get_blockchain_info() {
    let node = Node::new_no_wallet();
    let json = node.client.get_blockchain_info().expect("getblockchaininfo");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_best_block_hash() {
    let node = Node::new_no_wallet();
    let json = node.client.get_best_block_hash().expect("getbestblockhash");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_block() {
    let node = Node::new_no_wallet();
    let block_hash = best_block_hash();

    let json = node.client.get_block_verbose_zero(block_hash).expect("getblock verbose=0");
    assert!(json.into_model().is_ok());

    let json = node.client.get_block_verbose_one(block_hash).expect("getblock verbose=1");
    assert!(json.into_model().is_ok());

    // TODO: Test getblock 2
    // let json = node.client.get_block_with_verbosity(block_hash, 2).expect("getblock verbosity 2");
    // assert!(json.into_model().is_ok());
}

#[test]
fn get_block_count() {
    let node = Node::new_no_wallet();
    let json = node.client.get_block_count().expect("getblockcount");
    let _ = json.into_model();
}

#[test]
fn get_block_hash() {
    let node = Node::new_no_wallet();
    let json = node.client.get_block_hash(0).expect("getblockhash");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_block_header() { // verbose = false
    let node = Node::new_no_wallet();
    let block_hash = best_block_hash();
    let json = node.client.get_block_header(&block_hash).expect("getblockheader");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_block_header_verbose() { // verbose = true
    let node = Node::new_no_wallet();
    let block_hash = best_block_hash();
    let json = node.client.get_block_header_verbose(&block_hash).expect("getblockheader");
    assert!(json.into_model().is_ok());
}

// FIXME: I don't know why this passes for v17 and not v18. I tried making stats
// optional as suggested in the docs but to no avail.
#[test]
#[cfg(feature = "0_17_1")]
fn get_block_stats() {
    get_block_stats_by_height();
    get_block_stats_by_hash();
}

#[cfg(feature = "0_17_1")]
fn get_block_stats_by_height() {
    let node = Node::new_no_wallet();
    let json = node.client.get_block_stats_by_height(0).expect("getblockstats");
    assert!(json.into_model().is_ok());
}

#[cfg(feature = "0_17_1")]
fn get_block_stats_by_hash() { // verbose = true
    let node = Node::new_no_wallet();
    let block_hash = best_block_hash();
    let json = node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_block_stats_by_height_txindex() {
    let node = Node::new_no_wallet_txindex();
    let json = node.client.get_block_stats_by_height(0).expect("getblockstats");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_block_stats_by_hash_txindex() { // verbose = true
    let node = Node::new_no_wallet_txindex();
    let block_hash = best_block_hash();
    let json = node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_chain_tips() {
    let node = Node::new_no_wallet();
    let json = node.client.get_chain_tips().expect("getchaintips");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_chain_tx_stats() {
    let node = Node::new_no_wallet();
    let json = node.client.get_chain_tx_stats().expect("getchaintxstats");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_difficulty() {
    let node = Node::new_no_wallet();
    let json = node.client.get_difficulty().expect("getdifficulty");
    let _ = json.into_model();
}

#[test]
#[cfg(feature = "TODO")]
fn get_mempool_ancestors() {
    // We can probably get away with not testing this because it returns the
    // same type as `getmempoolentry` which is tested below.
}

#[test]
#[cfg(feature = "TODO")]
fn get_mempool_descendants() {
    // We can probably get away with not testing this because it returns the
    // same type as `getmempoolentry` which is tested below.
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_mempool_entry() {
    let node = Node::new_with_default_wallet();
    node.fund_wallet();
    let (_address, txid) = node.create_mempool_transaction();

    let json = node.client.get_mempool_entry(txid).expect("getmempoolentry");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_mempool_info() {
    let node = Node::new_with_default_wallet();
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    // Test the type and into model conversion code.
    let json = node.client.get_mempool_info().expect("getmempoolinfo");
    let info = json.into_model().expect("into_model");
    // Sanity check.
    assert_eq!(info.size, 1);
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_raw_mempool() {
    let node = Node::new_with_default_wallet();
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    // Test the type and into model conversion code.
    let json = node.client.get_raw_mempool().expect("getrawmempool");
    let mempool = json.into_model().expect("into_model");
    // Sanity check.
    assert_eq!(mempool.0.len(), 1);
}

#[test]
// FIXME: Fails with getrawmempool verbose: JsonRpc(Json(Error("invalid type: map, expected a sequence", line: 1, column: 0)))
#[cfg(feature = "TODO")]
fn get_raw_mempool_verbose() {
    let node = Node::new_with_default_wallet();
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    // Test the type and into model conversion code.
    let json = node.client.get_raw_mempool_verbose().expect("getrawmempool verbose");
    let mempool = json.into_model().expect("into_model");
    // Sanity check.
    assert_eq!(mempool.0.len(), 1);
}

#[test]
fn get_tx_out() {
    let node = Node::new_with_default_wallet();
    node.fund_wallet();
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    // Test the type and into model conversion code.
    let json = node.client.get_tx_out(txid, 1).expect("gettxout");
    let _ = json.into_model().expect("into_model");
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_tx_out_set_info() {
    let node = Node::new_with_default_wallet();
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();

    // Test the type and into model conversion code.
    let json = node.client.get_tx_out_set_info().expect("gettxoutsetinfo");
    let _ = json.into_model().expect("into_model");

}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
// Implicitly tests the omitted method `gettxoutproof` as well.
#[test]
fn verify_tx_out_proof() {
    let node = Node::new_with_default_wallet();
    node.fund_wallet();
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let proof = node.client.get_tx_out_proof(&[txid]).expect("gettxoutproof");

    let txids = node.client.verify_tx_out_proof(&proof).expect("verifytxoutproof");
    assert_eq!(txids.0.len(), 1);
}
