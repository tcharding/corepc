// SPDX-License-Identifier: CC0-1.0

use bitcoin::consensus::encode;
use bitcoin::{BlockHash, Psbt, SignedAmount, Transaction, Txid};

use super::{
    CreateWallet, GetBalances, GetBalancesError, GetTransaction, GetTransactionError,
    LastProcessedBlock, LastProcessedBlockError, LoadWallet, UnloadWallet, WalletProcessPsbt,
    WalletProcessPsbtError,
};
use crate::model;

impl CreateWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::CreateWallet {
        model::CreateWallet { name: self.name, warnings: self.warnings.unwrap_or_default() }
    }

    /// Returns the created wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

impl GetBalances {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalances, GetBalancesError> {
        use GetBalancesError as E;

        let mine = self.mine.into_model().map_err(E::Mine)?;
        let watch_only = self
            .watch_only
            .map(|watch_only| watch_only.into_model())
            .transpose()
            .map_err(E::WatchOnly)?;
        let last_processed_block = self
            .last_processed_block
            .map(|l| l.into_model())
            .transpose()
            .map_err(E::LastProcessedBlock)?;

        Ok(model::GetBalances { mine, watch_only, last_processed_block })
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
        let block_height =
            self.block_height.map(|h| crate::to_u32(h, "block_height")).transpose()?;
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wtxid = self.wtxid.map(|s| s.parse::<Txid>().map_err(E::Wtxid)).transpose()?;
        let wallet_conflicts = self
            .wallet_conflicts
            .into_iter()
            .map(|s| s.parse::<Txid>().map_err(E::WalletConflicts))
            .collect::<Result<Vec<_>, _>>()?;
        let replaced_by_txid = self
            .replaced_by_txid
            .map(|s| s.parse::<Txid>().map_err(E::ReplacedByTxid))
            .transpose()?;
        let replaces_txid =
            self.replaces_txid.map(|s| s.parse::<Txid>().map_err(E::ReplacesTxid)).transpose()?;
        let tx = encode::deserialize_hex::<Transaction>(&self.hex).map_err(E::Tx)?;
        let details = self
            .details
            .into_iter()
            .map(|d| d.into_model().map_err(E::Details))
            .collect::<Result<Vec<_>, _>>()?;
        let last_processed_block = self
            .last_processed_block
            .map(|l| l.into_model())
            .transpose()
            .map_err(E::LastProcessedBlock)?;

        Ok(model::GetTransaction {
            amount,
            fee,
            confirmations: self.confirmations,
            generated: self.generated,
            trusted: self.trusted,
            block_hash,
            block_height,
            block_index,
            block_time: self.block_time,
            txid,
            wtxid,
            wallet_conflicts,
            replaced_by_txid,
            replaces_txid,
            mempool_conflicts: None, // v28 and later only.
            to: self.to,
            time: self.time,
            time_received: self.time_received,
            comment: self.comment,
            bip125_replaceable: self.bip125_replaceable.into_model(),
            parent_descriptors: self.parent_descriptors,
            details,
            decoded: self.decoded,
            last_processed_block,
            tx,
        })
    }
}

impl LastProcessedBlock {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::LastProcessedBlock, LastProcessedBlockError> {
        let hash = self.hash.parse::<BlockHash>().map_err(LastProcessedBlockError::Hash)?;
        let height = crate::to_u32(self.height, "height")?;
        Ok(model::LastProcessedBlock { height, hash })
    }
}

impl LoadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::LoadWallet {
        model::LoadWallet { name: self.name, warnings: self.warnings.unwrap_or_default() }
    }

    /// Returns the loaded wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

impl UnloadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::UnloadWallet {
        model::UnloadWallet { warnings: self.warnings.unwrap_or_default() }
    }
}

impl WalletProcessPsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::WalletProcessPsbt, WalletProcessPsbtError> {
        use WalletProcessPsbtError as E;

        let psbt = self.psbt.parse::<Psbt>().map_err(E::Psbt)?;
        let hex =
            self.hex.as_ref().map(|h| encode::deserialize_hex(h)).transpose().map_err(E::Hex)?;
        Ok(model::WalletProcessPsbt { psbt, complete: self.complete, hex })
    }
}
