//! Utilities to traverse over multi-era block data

use std::fmt::Display;

use thiserror::Error;

mod support;

mod assets;
mod auxiliary;
mod block;
mod cert;
mod era;
pub mod fees;
pub mod hashes;
mod header;
mod input;
mod meta;
mod output;
pub mod probe;
mod redeemers;
mod signers;
mod size;
pub mod time;
mod tx;
pub mod update;
mod withdrawals;
mod witnesses;

pub use assets::{MultiEraAsset, MultiEraPolicyAssets};
pub use block::{MultiEraBlock, MultiEraBlockWithRawAuxiliary};
pub use cert::MultiEraCert;
pub use era::{Era, Feature};
pub use header::MultiEraHeader;
pub use input::{MultiEraInput, OutputRef};
pub use meta::MultiEraMeta;
pub use output::MultiEraOutput;
pub use redeemers::MultiEraRedeemer;
pub use signers::MultiEraSigners;
pub use tx::{MultiEraTx, MultiEraTxWithRawAuxiliary};
pub use update::MultiEraUpdate;
pub use withdrawals::MultiEraWithdrawals;

// TODO: move to genesis crate
pub mod wellknown;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid CBOR structure: {0}")]
    InvalidCbor(String),

    #[error("Unknown CBOR structure: {0}")]
    UnknownCbor(String),

    #[error("Unknown era tag: {0}")]
    UnknownEra(u16),

    #[error("Invalid era for request: {0}")]
    InvalidEra(Era),

    #[error("Invalid UTxO ref: {0}")]
    InvalidUtxoRef(String),
}

impl Error {
    pub fn invalid_cbor(error: impl Display) -> Self {
        Error::InvalidCbor(format!("{error}"))
    }

    pub fn unknown_cbor(bytes: &[u8]) -> Self {
        Error::UnknownCbor(hex::encode(bytes))
    }

    pub fn invalid_utxo_ref(str: &str) -> Self {
        Error::InvalidUtxoRef(str.to_owned())
    }
}

pub trait ComputeHash<const BYTES: usize> {
    fn compute_hash(&self) -> pallas_crypto::hash::Hash<BYTES>;
}

pub trait OriginalHash<const BYTES: usize> {
    fn original_hash(&self) -> pallas_crypto::hash::Hash<BYTES>;
}
