// SPDX-License-Identifier: CC0-1.0

use bitcoin::{consensus, hex, Amount, Transaction, Txid};

use super::{
    CreateRawTransaction, FundRawTransaction, FundRawTransactionError, SendRawTransaction,
};
use crate::model;

impl CreateRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::CreateRawTransaction, consensus::encode::FromHexError> {
        let tx: Transaction = consensus::encode::deserialize_hex(&self.0)?;
        Ok(model::CreateRawTransaction(tx))
    }

    /// Converts json straight to a `bitcoin::Transaction`.
    pub fn transaction(self) -> Result<Transaction, consensus::encode::FromHexError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl FundRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::FundRawTransaction, FundRawTransactionError> {
        use FundRawTransactionError as E;

        let tx: Transaction = consensus::encode::deserialize_hex(&self.hex).map_err(E::Hex)?;
        let fee = Amount::from_btc(self.fee).map_err(E::Fee)?;

        Ok(model::FundRawTransaction { tx, fee, change_position: self.change_position })
    }

    /// Converts json straight to a `bitcoin::Txid`.
    pub fn tx(self) -> Result<Transaction, FundRawTransactionError> {
        let model = self.into_model()?;
        Ok(model.tx)
    }
}

impl SendRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendRawTransaction, hex::HexToArrayError> {
        let txid = self.0.parse::<Txid>()?;
        Ok(model::SendRawTransaction(txid))
    }

    /// Converts json straight to a `bitcoin::Txid`.
    pub fn txid(self) -> Result<Txid, hex::HexToArrayError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}
