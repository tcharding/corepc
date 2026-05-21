// SPDX-License-Identifier: CC0-1.0

use bitcoin::consensus::encode;
use bitcoin::Transaction;

use super::{AbortPrivateBroadcast, GetPrivateBroadcastInfo};
use crate::model;

impl AbortPrivateBroadcast {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::AbortPrivateBroadcast, encode::FromHexError> {
        let txs = self
            .removed_transactions
            .iter()
            .map(|removed| encode::deserialize_hex::<Transaction>(&removed.hex))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(model::AbortPrivateBroadcast { removed_transactions: txs })
    }
}

impl GetPrivateBroadcastInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetPrivateBroadcastInfo, encode::FromHexError> {
        let txs = self
            .transactions
            .iter()
            .map(|tx| encode::deserialize_hex::<Transaction>(&tx.hex))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(model::GetPrivateBroadcastInfo { transactions: txs })
    }
}
