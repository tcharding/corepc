// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `createwallet`.
#[macro_export]
macro_rules! impl_test_v17__createwallet {
    () => {
        #[test]
        pub fn create_wallet() {
            // Implicitly tests createwalled because we create the default wallet.
            let _ = $crate::bitcoind_with_default_wallet();
        }
    };
}

/// Requires `Client` to be in scope and to implement `loadwallet`.
#[macro_export]
macro_rules! impl_test_v17__loadwallet {
    () => {
        #[test]
        fn load_wallet() {
            // Implicitly test loadwalled because we load the default wallet.
            let _ = $crate::bitcoind_with_default_wallet();
        }
    };
}

/// Requires `Client` to be in scope and to implement `unloadwallet`.
#[macro_export]
macro_rules! impl_test_v17__unloadwallet {
    () => {
        #[test]
        fn unload_wallet() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let wallet = format!("wallet-{}", rand::random::<u32>()).to_string();
            bitcoind.client.create_wallet(&wallet).expect("failed to create wallet");
            let json = bitcoind.client.unload_wallet(&wallet).expect("unloadwallet");
            assert!(json.into_model().is_ok())
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_new_address`.
#[macro_export]
macro_rules! impl_test_v17__getnewaddress {
    () => {
        #[test]
        fn get_new_address() {
            use bitcoind::AddressType;

            let bitcoind = $crate::bitcoind_with_default_wallet();

            let json = bitcoind.client.get_new_address().expect("getnewaddress");
            assert!(json.into_model().is_ok());

            // Test the helper as well just for good measure.
            let _ = bitcoind.client.new_address().unwrap();

            // Exhaustively test address types with helper.
            let _ = bitcoind
                .client
                .new_address_with_type(AddressType::Legacy)
                .unwrap();
            let _ = bitcoind
                .client
                .new_address_with_type(AddressType::P2shSegwit)
                .unwrap();
            let _ = bitcoind
                .client
                .new_address_with_type(AddressType::Bech32)
                .unwrap();
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_balance`.
#[macro_export]
macro_rules! impl_test_v17__getbalance {
    () => {
        #[test]
        fn get_balance() {
            use client::types::model;

            let bitcoind = $crate::bitcoind_with_default_wallet();
            let json = bitcoind.client.get_balance().expect("getbalance");
            assert!(json.into_model().is_ok())
        }
    };
}

/// Requires `Client` to be in scope and to implement:
/// - `generate_to_address`
/// - `send_to_address`
#[macro_export]
macro_rules! impl_test_v17__sendtoaddress {
    () => {
        #[test]
        fn send_to_address() {
            use bitcoin::Amount;

            let bitcoind = $crate::bitcoind_with_default_wallet();
            let address = bitcoind.client.new_address().expect("failed to create new address");
            let _ = bitcoind.client.generate_to_address(101, &address).expect("generatetoaddress");

            let json = bitcoind
                .client
                .send_to_address(&address, Amount::from_sat(10_000))
                .expect("sendtddress");
            json.into_model().unwrap();
        }
    };
}

/// Requires `Client` to be in scope and to implement:
/// - `generate_to_address`
/// - `send_to_address`
/// - `get_transaction`
#[macro_export]
macro_rules! impl_test_v17__gettransaction {
    () => {
        #[test]
        fn get_transaction() {
            use bitcoin::Amount;
            use client::types::model;

            let bitcoind = $crate::bitcoind_with_default_wallet();
            let address = bitcoind.client.new_address().expect("failed to create new address");
            let _ = bitcoind.client.generate_to_address(101, &address).expect("generatetoaddress");

            let txid = bitcoind
                .client
                .send_to_address(&address, Amount::from_sat(10_000))
                .expect("sendtoaddress")
                .txid()
                .unwrap();

            let json = bitcoind.client.get_transaction(txid).expect("gettransaction");
            json.into_model().unwrap();
        }
    };
}
