// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Util ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use bitcoin::{PublicKey, PrivateKey};
use integration_test::{Node, NodeExt as _, Wallet};
use node::vtype::*;
use node::mtype;

#[test]
fn util__create_multisig__modelled() {
    let nrequired = 2;

    // Use two valid, deterministic public keys from the pubkey_sort test vectors.
    let pubkey1 = "02ff12471208c14bd580709cb2358d98975247d8765f92bc25eab3b2763ed605f8"
        .parse::<PublicKey>()
        .unwrap();
    let pubkey2 = "02fe6f0a5a297eb38c391581c4413e084773ea23954d93f7753db7dc0adc188b2f"
        .parse::<PublicKey>()
        .unwrap();

    let node = Node::with_wallet(Wallet::Default, &[]);
    let json: CreateMultisig = node
        .client
        .create_multisig(nrequired, vec![pubkey1, pubkey2])
        .expect("createmultisig");
    let res: Result<mtype::CreateMultisig, CreateMultisigError> = json.into_model();
    let _ = res.expect("CreateMultisig into model");
}

#[cfg(not(feature = "v17"))]
#[test]
fn util__derive_addresses__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    // Use a valid, deterministic public key from the pubkey_sort test vectors and the checksum for it.
    let descriptor = "pkh(02ff12471208c14bd580709cb2358d98975247d8765f92bc25eab3b2763ed605f8)#sf4k0g3u";

    let json: DeriveAddresses = node.client.derive_addresses(descriptor).expect("deriveaddresses");
    let res: Result<mtype::DeriveAddresses, _> = json.into_model();
    let _ = res.expect("DeriveAddresses into model");

    // For v29 and above test a multipath descriptor.
    #[cfg(not(feature = "v28_and_below"))]
    {
        // Create a multipath descriptor taken from running `listdescriptors` on the node. With 2 derivation paths.
        let multipath_descriptor = "wpkh([26b4ed16/84h/1h/0h]tpubDDe7JUw2CGU1rYZxupmNrhDXuE1fv25gs4je3BBuWCFwTW9QHGgyh5cjAEugd14ysJXTVshPvnUVABfD66HZKCS9gp5AYFd5K2WN2oVFp8t/<0;1>/*)#grvmsm8m";

        let range = (0, 3);
        let json: DeriveAddressesMultipath = node.client.derive_addresses_multipath(multipath_descriptor, range)
            .expect("deriveaddresses");
        let res: Result<mtype::DeriveAddressesMultipath, _> = json.into_model();
        let derived = res.expect("DeriveAddressesMultipath into model");

        // Should return 2 `DeriveAddresses`, one for each derivation path (0 and 1).
        assert_eq!(derived.addresses.len(), 2);
        // Each `DeriveAddresses` should contain 4 addresses for range [0, 3].
        assert_eq!(derived.addresses[0].addresses.len(), 4);
    }
}

#[test]
fn util__estimate_smart_fee__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: EstimateSmartFee = node.client.estimate_smart_fee(6).expect("estimatesmartfee");
    let res: Result<mtype::EstimateSmartFee, _> = json.into_model();
    let _ = res.expect("EstimateSmartFee into model");
}

#[cfg(not(feature = "v17"))]
#[test]
fn util__get_descriptor_info() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    // Use a valid, deterministic public key from the pubkey_sort test vectors
    let descriptor = "pkh(02ff12471208c14bd580709cb2358d98975247d8765f92bc25eab3b2763ed605f8)";
    let _: GetDescriptorInfo = node.client.get_descriptor_info(descriptor).expect("getdescriptorinfo");
}

#[cfg(not(feature = "v20_and_below"))]
#[test]
fn util__get_index_info() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    let index_info: GetIndexInfo = node.client.get_index_info().expect("getindexinfo");

    let txindex_info = index_info.0.get("txindex").unwrap();
    assert!(txindex_info.best_block_height < u32::MAX, "best_block_height should be a valid block height");
}

#[test]
fn util__sign_message_with_priv_key__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let privkey =
        PrivateKey::from_wif("cVt4o7BGAig1UXywgGSmARhxMdzP5qvQsxKkSsc1XEkw3tDTQFpy").unwrap();
    let message = "integration test message";

    // Derive the address from the private key
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let pubkey = privkey.public_key(&secp);
    let addr = bitcoin::Address::p2pkh(pubkey, privkey.network);

    // Sign the message with the private key
    let json: SignMessageWithPrivKey = node
        .client
        .sign_message_with_privkey(&privkey, message)
        .expect("signmessagewithprivkey");
    let res: Result<mtype::SignMessageWithPrivKey, _> = json.into_model();
    let sig = res.expect("SignMessageWithPrivKey into model");

    // Verify the message using the returned signature
    let verified: VerifyMessage = node
        .client
        .verify_message(&addr, &sig.0, message)
        .expect("verifymessage");
    assert!(verified.0, "Signature should verify for the correct address and message");
}

#[test]
fn util__validate_address__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let addr = node.client.new_address().expect("new_address");
    let json: ValidateAddress = node.client.validate_address(&addr).expect("validateaddress");
    let res: Result<mtype::ValidateAddress, ValidateAddressError> = json.into_model();
    let _ = res.expect("ValidateAddress into model");
}

// This is tested in util__sign_message_with_priv_key__modelled()
#[test]
fn util__verify_message() {}
