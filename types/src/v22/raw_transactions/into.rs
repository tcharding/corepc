// SPDX-License-Identifier: CC0-1.0

use bitcoin::Address;

use super::{DecodeScript, DecodeScriptError};
use crate::model;

impl DecodeScript {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodeScript, DecodeScriptError> {
        use DecodeScriptError as E;

        let address = match self.address {
            Some(addr) => Some(addr.parse::<Address<_>>().map_err(E::Address)?),
            None => None,
        };
        let addresses = match self.addresses {
            Some(addresses) => addresses
                .iter()
                .map(|s| s.parse::<Address<_>>())
                .collect::<Result<_, _>>()
                .map_err(E::Addresses)?,
            None => vec![],
        };
        let p2sh = self.p2sh.map(|s| s.parse::<Address<_>>()).transpose().map_err(E::P2sh)?;

        Ok(model::DecodeScript {
            script_pubkey: None,
            type_: self.type_,
            descriptor: None,
            address,
            required_signatures: self.required_signatures,
            addresses,
            p2sh,
            p2sh_segwit: self.p2sh_segwit,
        })
    }
}
