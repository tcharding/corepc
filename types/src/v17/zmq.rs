// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - zmq.
//!
//! Types for methods found under the `== Zmq ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getzmqnotifications`.
///
///> getzmqnotifications
///>
///> Returns information about the active ZeroMQ notifications.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetZmqNotifications {
    /// Type of notification.
    #[serde(rename = "type")]
    type_: String,
    /// Address of the publisher.
    address: String,
}
