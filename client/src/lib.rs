// SPDX-License-Identifier: CC0-1.0

//! Support for connecting to Bitcoin Core via JSON-RPC.

/// Re-export the `rust-bitcoin` crate.
pub extern crate bitcoin;

/// Re-export the `corepc-types` crate.
pub extern crate types;

#[cfg(feature = "client-sync")]
#[macro_use]
pub mod client_sync;

use serde::{Deserialize, Serialize};

/// Wrap an amount and serialize it as BTC.
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AmountSerBtc(
    #[serde(with = "bitcoin::amount::serde::as_btc")]
    bitcoin::Amount
);

/// Wrap an amount and serialize it as sats.
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AmountSerSat(
    #[serde(with = "bitcoin::amount::serde::as_sat")]
    bitcoin::Amount
);
