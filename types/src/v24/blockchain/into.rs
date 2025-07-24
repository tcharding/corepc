// SPDX-License-Identifier: CC0-1.0

use bitcoin::{OutPoint, Txid, Wtxid};

use super::{
    GetMempoolEntry, GetMempoolInfo, GetMempoolInfoError, GetTxSpendingPrevout,
    GetTxSpendingPrevoutError, GetTxSpendingPrevoutItem, MempoolEntry, MempoolEntryError,
};
use crate::model;

impl GetMempoolEntry {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolEntry, MempoolEntryError> {
        Ok(model::GetMempoolEntry(self.0.into_model()?))
    }
}

impl MempoolEntry {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::MempoolEntry, MempoolEntryError> {
        use MempoolEntryError as E;

        let vsize = Some(crate::to_u32(self.vsize, "vsize")?);
        let size = None;
        let weight = Some(crate::to_u32(self.weight, "weight")?);
        let time = crate::to_u32(self.time, "time")?;
        let height = crate::to_u32(self.height, "height")?;
        let descendant_count = crate::to_u32(self.descendant_count, "descendant_count")?;
        let descendant_size = crate::to_u32(self.descendant_size, "descendant_size")?;
        let ancestor_count = crate::to_u32(self.ancestor_count, "ancestor_count")?;
        let ancestor_size = crate::to_u32(self.ancestor_size, "ancestor_size")?;
        let wtxid = self.wtxid.parse::<Wtxid>().map_err(E::Wtxid)?;
        let fees = self.fees.into_model().map_err(E::Fees)?;
        let depends = self
            .depends
            .iter()
            .map(|txid| txid.parse::<Txid>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Depends)?;
        let spent_by = self
            .spent_by
            .iter()
            .map(|txid| txid.parse::<Txid>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::SpentBy)?;

        Ok(model::MempoolEntry {
            vsize,
            size,
            weight,
            time,
            height,
            descendant_count,
            descendant_size,
            ancestor_count,
            ancestor_size,
            wtxid,
            fees,
            depends,
            spent_by,
            bip125_replaceable: Some(self.bip125_replaceable),
            unbroadcast: Some(self.unbroadcast),
        })
    }
}

impl GetMempoolInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolInfo, GetMempoolInfoError> {
        let size = crate::to_u32(self.size, "size")?;
        let bytes = crate::to_u32(self.bytes, "bytes")?;
        let usage = crate::to_u32(self.usage, "usage")?;
        let max_mempool = crate::to_u32(self.max_mempool, "max_mempool")?;
        let mempool_min_fee = crate::btc_per_kb(self.mempool_min_fee)?;
        let min_relay_tx_fee = crate::btc_per_kb(self.min_relay_tx_fee)?;
        let incremental_relay_fee = crate::btc_per_kb(self.incremental_relay_fee)?;
        let unbroadcast_count = Some(crate::to_u32(self.unbroadcast_count, "unbroadcast_count")?);

        Ok(model::GetMempoolInfo {
            loaded: Some(self.loaded),
            size,
            bytes,
            usage,
            total_fee: Some(self.total_fee),
            max_mempool,
            mempool_min_fee,
            min_relay_tx_fee,
            incremental_relay_fee,
            unbroadcast_count,
            full_rbf: Some(self.full_rbf),
        })
    }
}

impl GetTxSpendingPrevout {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTxSpendingPrevout, GetTxSpendingPrevoutError> {
        let items =
            self.0.into_iter().map(|item| item.into_model()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetTxSpendingPrevout(items))
    }
}

impl GetTxSpendingPrevoutItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTxSpendingPrevoutItem, GetTxSpendingPrevoutError> {
        use GetTxSpendingPrevoutError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let outpoint = OutPoint { txid, vout: self.vout };
        let spending_txid =
            self.spending_txid.map(|id| id.parse::<Txid>().map_err(E::SpendingTxid)).transpose()?;

        Ok(model::GetTxSpendingPrevoutItem { outpoint, spending_txid })
    }
}
