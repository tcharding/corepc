// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v0.21`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `unloadwallet`
#[macro_export]
macro_rules! impl_client_v21__unload_wallet {
    () => {
        impl Client {
            pub fn unload_wallet(&self, wallet: &str) -> Result<UnloadWallet> {
                self.call("unloadwallet", &[wallet.into()])
            }
        }
    };
}
