// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v28`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `gethdkeys`.
#[macro_export]
macro_rules! impl_client_v28__get_hd_keys {
    () => {
        impl Client {
            pub fn get_hd_keys(&self) -> Result<GetHdKeys> { self.call("gethdkeys", &[]) }
        }
    };
}
