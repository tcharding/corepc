// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v29` - util.
//!
//! Types for methods found under the `== Util ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getdescriptorinfo`.
///
/// > getdescriptorinfo "descriptor"
/// >
/// > Analyses a descriptor.
/// > Returns information about the descriptor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetDescriptorInfo {
    /// The descriptor in canonical form, without private keys. For a multipath descriptor, only the
    /// first will be returned.
    pub descriptor: String,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided
    /// descriptor specifies multipath derivation elements.
    pub multipath_expansion: Option<Vec<String>>,
    /// The checksum for the input descriptor.
    pub checksum: String,
    /// Whether the descriptor is ranged.
    #[serde(rename = "isrange")]
    pub is_range: bool,
    /// Whether the descriptor is solvable.
    #[serde(rename = "issolvable")]
    pub is_solvable: bool,
    /// Whether the input descriptor contained at least one private key.
    #[serde(rename = "hasprivatekeys")]
    pub has_private_keys: bool,
}
