// SPDX-License-Identifier: CC0-1.0

use bitcoin::hashes::sha256;
use bitcoin::{Amount, BlockHash};

use super::{DumpTxOutSet, DumpTxOutSetError, GetTxOutSetInfo, GetTxOutSetInfoError};
use crate::model;

impl DumpTxOutSet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DumpTxOutSet, DumpTxOutSetError> {
        use DumpTxOutSetError as E;

        let coins_written = Amount::from_btc(self.coins_written).map_err(E::CoinsWritten)?;
        let base_hash = self.base_hash.parse::<BlockHash>().map_err(E::BaseHash)?;
        let base_height = crate::to_u32(self.base_height, "base_height")?;
        let tx_out_set_hash =
            self.tx_out_set_hash.parse::<sha256::Hash>().map_err(E::TxOutSetHash)?;
        let n_chain_tx = crate::to_u32(self.n_chain_tx, "n_chain_tx")?;

        Ok(model::DumpTxOutSet {
            coins_written,
            base_hash,
            base_height,
            path: self.path,
            tx_out_set_hash,
            n_chain_tx,
        })
    }
}

impl GetTxOutSetInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTxOutSetInfo, GetTxOutSetInfoError> {
        use GetTxOutSetInfoError as E;

        let height = crate::to_u32(self.height, "height")?;
        let best_block = self.best_block.parse::<BlockHash>().map_err(E::BestBlock)?;
        let transactions = crate::to_u32(self.transactions, "transactions")?;
        let tx_outs = crate::to_u32(self.tx_outs, "tx_outs")?;
        let bogo_size = crate::to_u32(self.bogo_size, "bogo_size")?;
        let disk_size = crate::to_u32(self.disk_size, "disk_size")?;
        let total_amount = Amount::from_btc(self.total_amount).map_err(E::TotalAmount)?;

        Ok(model::GetTxOutSetInfo {
            height,
            best_block,
            transactions,
            tx_outs,
            bogo_size,
            hash_serialized_2: None, // v17 to v25 only.
            hash_serialized_3: self.hash_serialized_3,
            disk_size,
            total_amount,
        })
    }
}
