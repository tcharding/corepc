// SPDX-License-Identifier: CC0-1.0

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::{Amount, BlockHash, SignedAmount, Transaction, Txid};

use super::{GetBalances, GetBalancesMine, GetBalancesWatchOnly, GetTransaction, GetTransactionError};
use crate::model;

impl GetBalances {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalances, ParseAmountError> {
        let mine = self.mine.into_model()?;
        let watch_only = self.watch_only.map(|watch_only| watch_only.into_model()).transpose()?;

        Ok(model::GetBalances { mine, watch_only })
    }
}

impl GetBalancesMine {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalancesMine, ParseAmountError> {
        let trusted = Amount::from_btc(self.trusted)?;
        let untrusted_pending = Amount::from_btc(self.untrusted_pending)?;
        let immature = Amount::from_btc(self.immature)?;
        let used = self.used.map(Amount::from_btc).transpose()?;

        Ok(model::GetBalancesMine { trusted, untrusted_pending, immature, used })
    }
}

impl GetBalancesWatchOnly {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalancesWatchOnly, ParseAmountError> {
        let trusted = Amount::from_btc(self.trusted)?;
        let untrusted_pending = Amount::from_btc(self.untrusted_pending)?;
        let immature = Amount::from_btc(self.immature)?;

        Ok(model::GetBalancesWatchOnly { trusted, untrusted_pending, immature })
    }
}

impl GetTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransaction, GetTransactionError> {
        use GetTransactionError as E;

        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let fee = self.fee.map(|fee| SignedAmount::from_btc(fee).map_err(E::Fee)).transpose()?;
        let block_hash =
            self.block_hash.map(|s| s.parse::<BlockHash>().map_err(E::BlockHash)).transpose()?;
        let block_index =
            self.block_index.map(|idx| crate::to_u32(idx, "block_index")).transpose()?;
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wallet_conflicts = self
            .wallet_conflicts
            .into_iter()
            .map(|s| s.parse::<Txid>().map_err(E::WalletConflicts))
            .collect::<Result<Vec<_>, _>>()?;
        let tx = encode::deserialize_hex::<Transaction>(&self.hex).map_err(E::Tx)?;
        let details = self
            .details
            .into_iter()
            .map(|d| d.into_model().map_err(E::Details))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(model::GetTransaction {
            amount,
            fee,
            confirmations: self.confirmations,
            generated: None, // v20 and later only.
            trusted: self.trusted,
            block_hash,
            block_height: None, // v20 and later only.
            block_index,
            block_time: self.block_time,
            txid,
            wtxid: None, // v24 and later only.
            wallet_conflicts,
            replaced_by_txid: None,  // v23 and later only.
            replaces_txid: None,     // v23 and later only.
            mempool_conflicts: None, // v28 and later only.
            to: None,                // v23 and later only.
            time: self.time,
            time_received: self.time_received,
            comment: None, // v20 to v24 only.
            bip125_replaceable: self.bip125_replaceable.into_model(),
            parent_descriptors: None, // v24 and later only.
            details,
            decoded: self.decoded,
            last_processed_block: None, // v26 and later only.
            tx,
        })
    }
}
