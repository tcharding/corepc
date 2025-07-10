// SPDX-License-Identifier: CC0-1.0

use super::{Send, SendError, UnloadWallet};
use crate::model;

impl UnloadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::UnloadWallet {
        model::UnloadWallet { warnings: vec![self.warning] }
    }
}

impl Send {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::Send, SendError> {
        use SendError as E;

        let txid = self.txid.as_ref().map(|s| s.parse()).transpose().map_err(E::Txid)?;

        let hex = self
            .hex
            .as_ref()
            .map(|h| bitcoin::consensus::encode::deserialize_hex(h))
            .transpose()
            .map_err(E::Hex)?;

        let psbt = self.psbt.as_ref().map(|p| p.parse()).transpose().map_err(E::Psbt)?;

        Ok(model::Send { complete: self.complete, txid, hex, psbt })
    }
}
