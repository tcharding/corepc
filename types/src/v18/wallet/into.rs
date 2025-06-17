// SPDX-License-Identifier: CC0-1.0

use bitcoin::amount::ParseAmountError;
use bitcoin::{Address, Amount, ScriptBuf, SignedAmount, Txid};

pub use self::error::ListReceivedByLabelError;
// TODO: Use explicit imports?
use super::*;
use crate::model;
use crate::v17::ListUnspentItemError;

impl GetReceivedByLabel {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetReceivedByLabel, ParseAmountError> {
        let amount = bitcoin::Amount::from_btc(self.0)?;
        Ok(model::GetReceivedByLabel(amount))
    }
}

impl ListReceivedByLabel {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListReceivedByLabel, ListReceivedByLabelError> {
        self.0
            .into_iter()
            .map(|item| item.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map(model::ListReceivedByLabel)
    }
}

impl ListReceivedByLabelItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListReceivedByLabelItem, ListReceivedByLabelError> {
        use ListReceivedByLabelError as E;

        let amount = Amount::from_btc(self.amount).map_err(E::Amount)?;
        let confirmations = crate::to_u32(self.confirmations, "confirmations")?;

        Ok(model::ListReceivedByLabelItem {
            involves_watch_only: self.involves_watch_only,
            amount,
            confirmations,
            label: self.label,
        })
    }
}

impl ListUnspent {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListUnspent, ListUnspentItemError> {
        self.0
            .into_iter()
            .map(|item| item.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map(model::ListUnspent)
    }
}

impl ListUnspentItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListUnspentItem, ListUnspentItemError> {
        use ListUnspentItemError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let vout = crate::to_u32(self.vout, "vout")?;
        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubkey)?;

        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let confirmations = crate::to_u32(self.confirmations, "confirmations")?;
        let redeem_script = self
            .redeem_script
            .map(|hex| ScriptBuf::from_hex(&hex).map_err(E::RedeemScript))
            .transpose()?;
        Ok(model::ListUnspentItem {
            txid,
            vout,
            address,
            label: self.label,
            script_pubkey,
            amount,
            confirmations,
            redeem_script,
            spendable: self.spendable,
            solvable: self.solvable,
            descriptor: self.descriptor,
            safe: self.safe,
        })
    }
}
