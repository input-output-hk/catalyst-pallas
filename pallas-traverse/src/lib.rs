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
pub mod value;
pub mod withdrawals;
pub mod witnesses;

// TODO: move to genesis crate
pub mod wellknown;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Era {
    Byron,
    Shelley,
    Allegra, // time-locks
    Mary,    // multi-assets
    Alonzo,  // smart-contracts
    Babbage, // CIP-31/32/33
    Conway,  // governance CIP-1694
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Feature {
    TimeLocks,
    MultiAssets,
    Staking,
    SmartContracts,
    CIP31,
    CIP32,
    CIP33,
    CIP1694,
}

#[derive(Debug)]
pub enum MultiEraHeader<'b> {
    EpochBoundary(Cow<'b, KeepRaw<'b, byron::EbbHead>>),
    ShelleyCompatible(Cow<'b, KeepRaw<'b, alonzo::MintedHeader<'b>>>),
    BabbageCompatible(Cow<'b, KeepRaw<'b, babbage::MintedHeader<'b>>>),
    Byron(Cow<'b, KeepRaw<'b, byron::BlockHead>>),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraBlock<'b> {
    EpochBoundary(Box<byron::MintedEbBlock<'b>>),
    AlonzoCompatible(Box<alonzo::MintedBlock<'b>>, Era),
    Babbage(Box<babbage::MintedBlock<'b>>),
    Byron(Box<byron::MintedBlock<'b>>),
    Conway(Box<conway::MintedBlock<'b>>),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraTx<'b> {
    AlonzoCompatible(Box<Cow<'b, alonzo::MintedTx<'b>>>, Era),
    Babbage(Box<Cow<'b, babbage::MintedTx<'b>>>),
    Byron(Box<Cow<'b, byron::MintedTxPayload<'b>>>),
    Conway(Box<Cow<'b, conway::MintedTx<'b>>>),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraValue<'b> {
    Byron(u64),
    AlonzoCompatible(Cow<'b, alonzo::Value>),
    Conway(Cow<'b, conway::Value>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MultiEraOutput<'b> {
    AlonzoCompatible(Box<Cow<'b, alonzo::TransactionOutput>>, Era),
    Babbage(Box<Cow<'b, babbage::MintedTransactionOutput<'b>>>),
    Conway(Box<Cow<'b, conway::MintedTransactionOutput<'b>>>),
    Byron(Box<Cow<'b, byron::TxOut>>),
}

#[derive(Debug, Clone, PartialEq, Eq, StdHash)]
#[non_exhaustive]
pub enum MultiEraInput<'b> {
    Byron(Box<Cow<'b, byron::TxIn>>),
    AlonzoCompatible(Box<Cow<'b, alonzo::TransactionInput>>),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraCert<'b> {
    NotApplicable,
    AlonzoCompatible(Box<Cow<'b, alonzo::Certificate>>),
    Conway(Box<Cow<'b, conway::Certificate>>),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraRedeemer<'b> {
    AlonzoCompatible(Box<Cow<'b, alonzo::Redeemer>>),
    Conway(
        Box<Cow<'b, conway::RedeemersKey>>,
        Box<Cow<'b, conway::RedeemersValue>>,
    ),
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub enum MultiEraMeta<'b> {
    #[default]
    Empty,
    NotApplicable,
    AlonzoCompatible(&'b alonzo::Metadata),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraPolicyAssets<'b> {
    AlonzoCompatibleMint(
        &'b alonzo::PolicyId,
        &'b KeyValuePairs<alonzo::AssetName, i64>,
    ),
    AlonzoCompatibleOutput(
        &'b alonzo::PolicyId,
        &'b KeyValuePairs<alonzo::AssetName, u64>,
    ),
    ConwayMint(
        &'b alonzo::PolicyId,
        &'b NonEmptyKeyValuePairs<alonzo::AssetName, NonZeroInt>,
    ),
    ConwayOutput(
        &'b alonzo::PolicyId,
        &'b NonEmptyKeyValuePairs<alonzo::AssetName, PositiveCoin>,
    ),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraAsset<'b> {
    AlonzoCompatibleOutput(&'b alonzo::PolicyId, &'b alonzo::AssetName, u64),
    AlonzoCompatibleMint(&'b alonzo::PolicyId, &'b alonzo::AssetName, i64),
    ConwayOutput(&'b alonzo::PolicyId, &'b alonzo::AssetName, PositiveCoin),
    ConwayMint(&'b alonzo::PolicyId, &'b alonzo::AssetName, NonZeroInt),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraWithdrawals<'b> {
    NotApplicable,
    Empty,
    AlonzoCompatible(&'b alonzo::Withdrawals),
    Conway(&'b conway::Withdrawals),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraUpdate<'b> {
    Byron(u64, Box<Cow<'b, byron::UpProp>>),
    AlonzoCompatible(Box<Cow<'b, alonzo::Update>>),
    Babbage(Box<Cow<'b, babbage::Update>>),
    Conway(Box<Cow<'b, conway::Update>>),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraSigners<'b> {
    NotApplicable,
    Empty,
    AlonzoCompatible(&'b alonzo::RequiredSigners),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OutputRef(Hash<32>, u64);

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
