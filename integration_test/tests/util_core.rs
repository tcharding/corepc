// SPDX-License-Identifier: CC0-1.0

//! Tests derived from Bitcoin Core's rpc_validateaddress.py and rpc_getdescriptorinfo.py

use bitcoind::vtype::*;
use bitcoind::AddressType;
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

#[test]
fn validate_address_witness_fields_present_for_bech32() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);

    let addr = node.client.new_address_with_type(AddressType::Bech32).unwrap();
    let json: ValidateAddress = node.client.validate_address(&addr).unwrap();

    assert!(json.is_witness);
    assert!(json.witness_version.is_some());
    assert!(json.witness_program.is_some());
}

#[test]
#[cfg(not(feature = "v28_and_below"))]
fn get_descriptor_info_multipath_returns_expansion() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);

    let multipath = "wpkh([26b4ed16/84h/1h/0h]tpubDDe7JUw2CGU1rYZxupmNrhDXuE1fv25gs4je3BBuWCFwTW9QHGgyh5cjAEugd14ysJXTVshPvnUVABfD66HZKCS9gp5AYFd5K2WN2oVFp8t/<0;1>/*)#grvmsm8m";

    let json: GetDescriptorInfo = node.client.get_descriptor_info(multipath).unwrap();

    assert!(json.multipath_expansion.is_some());
}

#[test]
fn sign_and_verify_message_round_trip() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);

    let addr = node.client.new_address_with_type(bitcoind::AddressType::Legacy).unwrap();
    let message = "corepc round-trip test";

    let json: SignMessage = node.client.sign_message(&addr, message).unwrap();
    let sig = json.into_model().unwrap().0;

    let verified: VerifyMessage = node.client.verify_message(&addr, &sig, message).unwrap();
    assert!(verified.0);
}
