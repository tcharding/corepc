// SPDX-License-Identifier: CC0-1.0

use bitcoin::consensus::encode;
use bitcoin::{block, hex, Block, BlockHash, CompactTarget, Txid, Weight, Work};

// TODO: Use explicit imports?
use super::*;

impl GetBestBlockHash {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBestBlockHash, hex::HexToArrayError> {
        let hash = self.0.parse::<BlockHash>()?;
        Ok(model::GetBestBlockHash(hash))
    }

    /// Converts json straight to a `bitcoin::BlockHash`.
    pub fn block_hash(self) -> Result<BlockHash, hex::HexToArrayError> { Ok(self.into_model()?.0) }
}

impl GetBlockVerbosityZero {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockVerbosityZero, encode::FromHexError> {
        let block = encode::deserialize_hex(&self.0)?;
        Ok(model::GetBlockVerbosityZero(block))
    }

    /// Converts json straight to a `bitcoin::Block`.
    pub fn block(self) -> Result<Block, encode::FromHexError> { Ok(self.into_model()?.0) }
}

impl GetBlockVerbosityOne {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockVerbosityOne, GetBlockVerbosityOneError> {
        use GetBlockVerbosityOneError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;
        let stripped_size =
            self.stripped_size.map(|size| crate::to_u32(size, "stripped_size")).transpose()?;
        let weight = Weight::from_wu(self.weight); // FIXME: Confirm this uses weight units.
        let version = block::Version::from_consensus(self.version);
        let tx = self
            .tx
            .iter()
            .map(|t| t.parse::<Txid>().map_err(E::Hash))
            .collect::<Result<Vec<_>, _>>()?;
        let median_time = self.median_time.map(|t| crate::to_u32(t, "median_time")).transpose()?;
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;
        let previous_block_hash = self
            .previous_block_hash
            .map(|s| s.parse::<BlockHash>())
            .transpose()
            .map_err(E::PreviousBlockHash)?;
        let next_block_hash = self
            .next_block_hash
            .map(|s| s.parse::<BlockHash>())
            .transpose()
            .map_err(E::NextBlockHash)?;

        Ok(model::GetBlockVerbosityOne {
            hash,
            confirmations: self.confirmations,
            size: crate::to_u32(self.size, "size")?,
            stripped_size,
            weight,
            height: crate::to_u32(self.height, "height")?,
            version,
            merkle_root: self.merkle_root, // TODO: Use hash, which one depends on segwit or not.
            tx,
            time: crate::to_u32(self.time, "time")?,
            median_time,
            nonce: crate::to_u32(self.nonce, "nonce")?,
            bits,
            difficulty: self.difficulty,
            chain_work,
            n_tx: crate::to_u32(self.n_tx, "n_tx")?,
            previous_block_hash,
            next_block_hash,
        })
    }
}

impl GetBlockchainInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockchainInfo, GetBlockchainInfoError> {
        use GetBlockchainInfoError as E;

        let chain = Network::from_core_arg(&self.chain).map_err(E::Chain)?;
        let best_block_hash =
            self.best_block_hash.parse::<BlockHash>().map_err(E::BestBlockHash)?;
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;
        let prune_height =
            self.prune_height.map(|h| crate::to_u32(h, "prune_height")).transpose()?;
        let prune_target_size =
            self.prune_target_size.map(|h| crate::to_u32(h, "prune_target_size")).transpose()?;
        let softforks = BTreeMap::new(); // TODO: Handle softforks stuff.

        Ok(model::GetBlockchainInfo {
            chain,
            blocks: crate::to_u32(self.blocks, "blocks")?,
            headers: crate::to_u32(self.headers, "headers")?,
            best_block_hash,
            difficulty: self.difficulty,
            median_time: crate::to_u32(self.median_time, "median_time")?,
            verification_progress: self.verification_progress,
            initial_block_download: self.initial_block_download,
            chain_work,
            size_on_disk: self.size_on_disk,
            pruned: self.pruned,
            prune_height,
            automatic_pruning: self.automatic_pruning,
            prune_target_size,
            softforks,
            warnings: vec![self.warnings],
        })
    }
}

impl Bip9SoftforkStatus {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::Bip9SoftforkStatus {
        use model::Bip9SoftforkStatus::*;

        match self {
            Self::Defined => Defined,
            Self::Started => Started,
            Self::LockedIn => LockedIn,
            Self::Active => Active,
            Self::Failed => Failed,
        }
    }
}

impl GetBlockCount {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::GetBlockCount { model::GetBlockCount(self.0) }
}

impl GetBlockHash {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockHash, hex::HexToArrayError> {
        let hash = self.0.parse::<BlockHash>()?;
        Ok(model::GetBlockHash(hash))
    }

    /// Converts json straight to a `bitcoin::BlockHash`.
    pub fn block_hash(self) -> Result<BlockHash, hex::HexToArrayError> { Ok(self.into_model()?.0) }
}

impl GetBlockHeader {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockHeader, GetBlockHeaderError> {
        use GetBlockHeaderError as E;

        let v = Vec::from_hex(&self.0).map_err(E::Hex)?;
        let header = encode::deserialize::<block::Header>(&v).map_err(E::Consensus)?;

        Ok(model::GetBlockHeader(header))
    }

    /// Converts json straight to a `bitcoin::BlockHeader`.
    pub fn block_header(self) -> Result<block::Header, GetBlockHeaderError> {
        Ok(self.into_model()?.0)
    }
}

impl GetBlockHeaderVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockHeaderVerbose, GetBlockHeaderVerboseError> {
        use GetBlockHeaderVerboseError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;
        let version = block::Version::from_consensus(self.version);
        let merkle_root = self.merkle_root.parse::<TxMerkleNode>().map_err(E::MerkleRoot)?;
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let chain_work = Work::from_unprefixed_hex(&self.bits).map_err(E::ChainWork)?;
        let previous_block_hash = self
            .previous_block_hash
            .map(|s| s.parse::<BlockHash>().map_err(E::PreviousBlockHash))
            .transpose()?;
        let next_block_hash = self
            .next_block_hash
            .map(|s| s.parse::<BlockHash>().map_err(E::NextBlockHash))
            .transpose()?;

        Ok(model::GetBlockHeaderVerbose {
            hash,
            confirmations: self.confirmations,
            height: crate::to_u32(self.height, "height")?,
            version,
            merkle_root,
            time: crate::to_u32(self.time, "time")?,
            median_time: crate::to_u32(self.median_time, "median_time")?,
            nonce: crate::to_u32(self.nonce, "nonce")?,
            bits,
            difficulty: self.difficulty,
            chain_work,
            n_tx: self.n_tx,
            previous_block_hash,
            next_block_hash,
        })
    }

    /// Converts json straight to a `bitcoin::BlockHeader`.
    pub fn block_header(self) -> Result<block::Header, hex::HexToArrayError> { todo!() }
}

impl GetBlockStats {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockStats, GetBlockStatsError> {
        use GetBlockStatsError as E;

        // `FeeRate::sat_per_vb` returns an option if value overflows.
        let average_fee_rate = FeeRate::from_sat_per_vb(self.average_fee_rate);
        let block_hash = self.block_hash.parse::<BlockHash>().map_err(E::BlockHash)?;
        let fee_rate_percentiles = self
            .fee_rate_percentiles
            .iter()
            .map(|vb| FeeRate::from_sat_per_vb(*vb))
            .collect::<Vec<Option<FeeRate>>>();
        let max_fee_rate = FeeRate::from_sat_per_vb(self.max_fee_rate);
        let minimum_fee_rate = FeeRate::from_sat_per_vb(self.minimum_fee_rate);

        // FIXME: Double check that these values are virtual bytes and not weight units.
        let segwit_total_weight = Weight::from_vb(self.segwit_total_weight);
        let total_weight = Weight::from_vb(self.total_weight);

        Ok(model::GetBlockStats {
            average_fee: Amount::from_sat(self.average_fee),
            average_fee_rate,
            average_tx_size: crate::to_u32(self.average_tx_size, "average_tx_size")?,
            block_hash,
            fee_rate_percentiles,
            height: crate::to_u32(self.height, "height")?,
            inputs: crate::to_u32(self.inputs, "inputs")?,
            max_fee: Amount::from_sat(self.max_fee),
            max_fee_rate,
            max_tx_size: crate::to_u32(self.max_tx_size, "max_tx_size")?,
            median_fee: Amount::from_sat(self.median_fee),
            median_time: crate::to_u32(self.median_time, "median_time")?,
            median_tx_size: crate::to_u32(self.median_tx_size, "median_tx_size")?,
            minimum_fee: Amount::from_sat(self.minimum_fee),
            minimum_fee_rate,
            minimum_tx_size: crate::to_u32(self.minimum_tx_size, "minimum_tx_size")?,
            outputs: crate::to_u32(self.outputs, "outputs")?,
            subsidy: Amount::from_sat(self.subsidy),
            segwit_total_size: crate::to_u32(self.segwit_total_size, "segwit_total_size")?,
            segwit_total_weight,
            segwit_txs: crate::to_u32(self.segwit_txs, "segwit_txs")?,
            time: crate::to_u32(self.time, "time")?,
            total_out: Amount::from_sat(self.total_out),
            total_size: crate::to_u32(self.total_size, "total_size")?,
            total_weight,
            total_fee: Amount::from_sat(self.total_fee),
            txs: crate::to_u32(self.txs, "txs")?,
            utxo_increase: self.utxo_increase,
            utxo_size_increase: self.utxo_size_increase,
        })
    }
}

impl GetChainTips {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetChainTips, ChainTipsError> {
        let v = self.0.into_iter().map(|item| item.into_model()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetChainTips(v))
    }
}

impl ChainTips {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ChainTips, ChainTipsError> {
        use ChainTipsError as E;

        Ok(model::ChainTips {
            height: crate::to_u32(self.height, "height")?,
            hash: self.hash.parse::<BlockHash>().map_err(E::Hash)?,
            branch_length: crate::to_u32(self.branch_length, "branch_length")?,
            status: self.status.into_model(),
        })
    }
}

impl ChainTipsStatus {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::ChainTipsStatus {
        use model::ChainTipsStatus::*;

        match self {
            Self::Invalid => Invalid,
            Self::HeadersOnly => HeadersOnly,
            Self::ValidHeaders => ValidHeaders,
            Self::ValidFork => ValidFork,
            Self::Active => Active,
        }
    }
}

impl GetChainTxStats {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetChainTxStats, GetChainTxStatsError> {
        use GetChainTxStatsError as E;

        let window_final_block_hash =
            self.window_final_block_hash.parse::<BlockHash>().map_err(E::WindowFinalBlockHash)?;
        let window_tx_count =
            self.window_tx_count.map(|h| crate::to_u32(h, "window_tx_count")).transpose()?;
        let window_interval =
            self.window_interval.map(|h| crate::to_u32(h, "window_interval")).transpose()?;
        let tx_rate = self.tx_rate.map(|h| crate::to_u32(h, "tx_rate")).transpose()?;

        Ok(model::GetChainTxStats {
            time: crate::to_u32(self.time, "time")?,
            tx_count: crate::to_u32(self.tx_count, "tx_count")?,
            window_final_block_hash,
            window_block_count: crate::to_u32(self.window_block_count, "window_block_count")?,
            window_tx_count,
            window_interval,
            tx_rate,
        })
    }
}

impl GetDifficulty {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::GetDifficulty { model::GetDifficulty(self.0) }
}

impl GetMempoolAncestors {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolAncestors, hex::HexToArrayError> {
        let v = self.0.iter().map(|t| t.parse::<Txid>()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetMempoolAncestors(v))
    }
}

impl GetMempoolAncestorsVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolAncestorsVerbose, MapMempoolEntryError> {
        use MapMempoolEntryError as E;

        let mut map = BTreeMap::new();
        for (k, v) in self.0.into_iter() {
            let txid = k.parse::<Txid>().map_err(E::Txid)?;
            let relative = v.into_model().map_err(E::MempoolEntry)?;
            map.insert(txid, relative);
        }
        Ok(model::GetMempoolAncestorsVerbose(map))
    }
}

impl GetMempoolDescendants {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolDescendants, hex::HexToArrayError> {
        let v = self.0.iter().map(|t| t.parse::<Txid>()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetMempoolDescendants(v))
    }
}

impl GetMempoolDescendantsVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetMempoolDescendantsVerbose, MapMempoolEntryError> {
        use MapMempoolEntryError as E;

        let mut map = BTreeMap::new();
        for (k, v) in self.0.into_iter() {
            let txid = k.parse::<Txid>().map_err(E::Txid)?;
            let relative = v.into_model().map_err(E::MempoolEntry)?;
            map.insert(txid, relative);
        }
        Ok(model::GetMempoolDescendantsVerbose(map))
    }
}

impl MempoolEntry {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::MempoolEntry, MempoolEntryError> {
        use MempoolEntryError as E;

        let size = Some(crate::to_u32(self.size, "size")?);
        let weight = None;
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
        })
    }
}

impl MempoolEntryFees {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::MempoolEntryFees, MempoolEntryFeesError> {
        use MempoolEntryFeesError as E;

        Ok(model::MempoolEntryFees {
            base: Amount::from_btc(self.base).map_err(E::Base)?,
            modified: Amount::from_btc(self.modified).map_err(E::Modified)?,
            ancestor: Amount::from_btc(self.ancestor).map_err(E::MempoolEntry)?,
            descendant: Amount::from_btc(self.descendant).map_err(E::Descendant)?,
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

        Ok(model::GetMempoolInfo {
            size,
            bytes,
            usage,
            max_mempool,
            mempool_min_fee,
            min_relay_tx_fee,
        })
    }
}

impl GetRawMempool {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetRawMempool, hex::HexToArrayError> {
        let v = self.0.iter().map(|t| t.parse::<Txid>()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GetRawMempool(v))
    }
}

impl GetRawMempoolVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetRawMempoolVerbose, MapMempoolEntryError> {
        use MapMempoolEntryError as E;

        let mut map = BTreeMap::new();
        for (k, v) in self.0.into_iter() {
            let txid = k.parse::<Txid>().map_err(E::Txid)?;
            let relative = v.into_model().map_err(E::MempoolEntry)?;
            map.insert(txid, relative);
        }
        Ok(model::GetRawMempoolVerbose(map))
    }
}

impl GetTxOut {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTxOut, GetTxOutError> {
        use GetTxOutError as E;

        let best_block = self.best_block.parse::<BlockHash>().map_err(E::BestBlock)?;
        let tx_out = TxOut {
            value: Amount::from_btc(self.value).map_err(E::Value)?,
            script_pubkey: ScriptBuf::from_hex(&self.script_pubkey.hex).map_err(E::ScriptPubkey)?,
        };

        let addresses = self
            .script_pubkey
            .addresses
            .into_iter()
            .map(|address| address.parse::<Address<_>>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Addresses)?;

        Ok(model::GetTxOut {
            best_block,
            confirmations: self.confirmations,
            tx_out,
            addresses,
            coinbase: self.coinbase,
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
        let hash_serialized_2 = Some(self.hash_serialized_2); // TODO: Convert this to a hash type.
        let disk_size = crate::to_u32(self.disk_size, "disk_size")?;
        let total_amount = Amount::from_btc(self.total_amount).map_err(E::TotalAmount)?;

        Ok(model::GetTxOutSetInfo {
            height,
            best_block,
            transactions,
            tx_outs,
            bogo_size,
            hash_serialized_2,
            disk_size,
            total_amount,
        })
    }
}

impl VerifyTxOutProof {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::VerifyTxOutProof, hex::HexToArrayError> {
        let proofs = self.0.iter().map(|t| t.parse::<Txid>()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::VerifyTxOutProof(proofs))
    }
}
