// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v0.19`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getmempoolancestors`
#[macro_export]
macro_rules! impl_client_v19__getmempoolancestors {
    () => {
        impl Client {
            pub fn get_mempool_ancestors(&self, txid: Txid) -> Result<GetMempoolAncestors> {
                // Equivalent to self.call("getmempoolancestors", &[into_json(txid)?, into_json(false)?])
                self.call("getmempoolancestors", &[into_json(txid)?])
            }

            pub fn get_mempool_ancestors_verbose(
                &self,
                txid: Txid,
            ) -> Result<GetMempoolAncestorsVerbose> {
                self.call("getmempoolancestors", &[into_json(txid)?, into_json(true)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempooldescendants`
#[macro_export]
macro_rules! impl_client_v19__getmempooldescendants {
    () => {
        impl Client {
            pub fn get_mempool_descendants(&self, txid: Txid) -> Result<GetMempoolDescendants> {
                // Equivalent to self.call("getmempooldescendants", &[into_json(txid)?, into_json(false)?])
                self.call("getmempooldescendants", &[into_json(txid)?])
            }

            pub fn get_mempool_descendants_verbose(
                &self,
                txid: Txid,
            ) -> Result<GetMempoolDescendantsVerbose> {
                self.call("getmempooldescendants", &[into_json(txid)?, into_json(true)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolentry`
#[macro_export]
macro_rules! impl_client_v19__getmempoolentry {
    () => {
        impl Client {
            pub fn get_mempool_entry(&self, txid: Txid) -> Result<GetMempoolEntry> {
                self.call("getmempoolentry", &[into_json(txid)?])
            }
        }
    };
}
