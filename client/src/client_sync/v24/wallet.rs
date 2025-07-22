// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v24`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `sendall`.
#[macro_export]
macro_rules! impl_client_v24__send_all {
    () => {
        impl Client {
            pub fn send_all(&self, recipients: &[Address]) -> Result<SendAll> {
                self.call("sendall", &[into_json(recipients)?])
            }
        }
    };
}
