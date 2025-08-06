// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::{address, bip32, hex, key, witness_program, witness_version};

use super::GetAddressInfoEmbeddedError;
use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetAddressInfo` type into the model type.
#[derive(Debug)]
pub enum GetAddressInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `script_pubkey` field failed.
    ScriptPubkey(hex::HexToBytesError),
    /// The `witness_version` field's value was too big for a u8.
    WitnessVersionValue(i64),
    /// Conversion of the `witness_version` field failed.
    WitnessVersion(witness_version::TryFromError),
    /// Conversion of the `witness_program` field hex string to bytes failed.
    WitnessProgramBytes(hex::HexToBytesError),
    /// Conversion of the `witness_program` field failed.
    WitnessProgram(witness_program::Error),
    /// Conversion of the `hex` field failed.
    Hex(hex::HexToBytesError),
    /// Conversion of the `pubkeys` field failed.
    Pubkeys(key::ParsePublicKeyError),
    /// Conversion of the `pubkey` field failed.
    Pubkey(key::ParsePublicKeyError),
    /// Conversion of the `embedded` field failed.
    Embedded(GetAddressInfoEmbeddedError),
    /// Conversion of the `hd_key_path` field failed.
    HdKeyPath(bip32::Error),
    /// Conversion of the `hd_seed_id` field failed.
    HdSeedId(hex::HexToArrayError),
    /// Conversion of the `hd_master_fingerprint` field failed.
    HdMasterFingerprint(hex::HexToArrayError),
}

impl fmt::Display for GetAddressInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetAddressInfoError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::ScriptPubkey(ref e) =>
                write_err!(f, "conversion of the `script_pubkey` field failed"; e),
            E::WitnessVersionValue(v) => write!(f, "invalid witness version number: {}", v),
            E::WitnessVersion(ref e) =>
                write_err!(f, "conversion of the `witness_version` field failed"; e),
            E::WitnessProgramBytes(ref e) =>
                write_err!(f, "conversion of the `witness_program` field hex string to bytes failed"; e),
            E::WitnessProgram(ref e) =>
                write_err!(f, "conversion of the `witness_program` field failed"; e),
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Pubkeys(ref e) => write_err!(f, "conversion of the `pubkeys` field failed"; e),
            E::Pubkey(ref e) => write_err!(f, "conversion of the `pubkey` failed"; e),
            E::Embedded(ref e) => write_err!(f, "conversion of the `embedded` field failed"; e),
            E::HdKeyPath(ref e) => write_err!(f, "conversion of the `hd_key_path` field failed"; e),
            E::HdSeedId(ref e) => write_err!(f, "conversion of the `hd_seed_id` field failed"; e),
            E::HdMasterFingerprint(ref e) =>
                write_err!(f, "conversion of the `hd_master_fingerprint` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetAddressInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetAddressInfoError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::ScriptPubkey(ref e) => Some(e),
            E::WitnessVersionValue(_) => None,
            E::WitnessVersion(ref e) => Some(e),
            E::WitnessProgramBytes(ref e) => Some(e),
            E::WitnessProgram(ref e) => Some(e),
            E::Hex(ref e) => Some(e),
            E::Pubkeys(ref e) => Some(e),
            E::Pubkey(ref e) => Some(e),
            E::Embedded(ref e) => Some(e),
            E::HdKeyPath(ref e) => Some(e),
            E::HdSeedId(ref e) => Some(e),
            E::HdMasterFingerprint(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetAddressInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ListReceivedByLabel` type into the model type.
#[derive(Debug)]
pub enum ListReceivedByLabelError {
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for ListReceivedByLabelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ListReceivedByLabelError::*;

        match *self {
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListReceivedByLabelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ListReceivedByLabelError::*;

        match *self {
            Amount(ref e) => Some(e),
            Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ListReceivedByLabelError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
