// SPDX-License-Identifier: CC0-1.0

use bitcoin::{hex, Amount, Txid};

use super::GetMempoolCluster;
use crate::model;

impl GetMempoolCluster {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolCluster, hex::HexToArrayError> {
        // TODO: Use combinators.
        let mut chunks = vec![];
        for chunk in self.chunks {
            let txs =
                chunk.txs.iter().map(|txid| txid.parse::<Txid>()).collect::<Result<Vec<_>, _>>()?;
            chunks.push(model::Chunk {
                chunk_fee: Amount::from_sat(chunk.chunk_fee),
                chunk_weight: chunk.chunk_weight,
                txs,
            })
        }

        Ok(model::GetMempoolCluster {
            cluster_weight: self.cluster_weight,
            tx_count: self.tx_count,
            chunks,
        })
    }
}
