// SPDX-License-Identifier: CC0-1.0

use super::UnloadWallet;
use crate::model;

impl UnloadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::UnloadWallet {
        model::UnloadWallet { warnings: vec![self.warning] }
    }
}
