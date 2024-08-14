//! Ledger primitives and cbor codec for the Byron era
//!
//! Handcrafted, idiomatic rust artifacts based on based on the [Byron CDDL](https://github.com/input-output-hk/cardano-ledger/blob/master/eras/byron/cddl-spec/byron.cddl) file in IOHK repo.

use pallas_codec::minicbor::{bytes::ByteVec, Decode, Encode};
use pallas_crypto::hash::Hash;

use pallas_codec::utils::{
    CborWrap, EmptyMap, KeepRaw, KeyValuePairs, MaybeIndefArray, OnlyRaw, TagWrap, ZeroOrOneArray,
};

// required for derive attrs to work
use pallas_codec::minicbor;

use std::hash::Hash as StdHash;

// Basic Cardano Types

pub type Blake2b256 = Hash<32>;

pub type TxId = Blake2b256;
pub type BlockId = Blake2b256;
pub type UpdId = Blake2b256;
pub type ByronHash = Blake2b256;

pub type Blake2b224 = Hash<28>;

pub type AddressId = Blake2b224;
pub type StakeholderId = Blake2b224;

pub type EpochId = u64;

#[derive(Encode, Decode, Debug, Clone)]
pub struct SlotId {
    #[n(0)]
    pub epoch: EpochId,

    #[n(1)]
    pub slot: u64,
}

pub type PubKey = ByteVec;
pub type Signature = ByteVec;

// Attributes

// quote from the CDDL file: at the moment we do not bother deserialising these,
// since they don't contain anything

// attributes = {* any => any}
pub type Attributes = EmptyMap;

// The cbor struct of the address payload is now defined in pallas-addresses.
// The primitives crate will treat addresses as a black-box vec of bytes.

// address = [ #6.24(bytes .cbor ([addressid, addrattr, addrtype])), u64 ]
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address {
    #[n(0)]
    pub payload: TagWrap<ByteVec, 24>,

    #[n(1)]
    pub crc: u32,
}

// Transactions

// txout = [address, u64]
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub struct TxOut {
    #[n(0)]
    pub address: Address,

    #[n(1)]
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, StdHash)]
pub enum TxIn {
    // [0, #6.24(bytes .cbor ([txid, u32]))]
    Variant0(CborWrap<(TxId, u32)>),

    // [u8 .ne 0, encoded-cbor]
    Other(u8, ByteVec),
}

impl<'b, C> minicbor::Decode<'b, C> for TxIn {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        d.array()?;

        let variant = d.u8()?;

        match variant {
            0 => Ok(TxIn::Variant0(d.decode_with(ctx)?)),
            x => Ok(TxIn::Other(x, d.decode_with(ctx)?)),
        }
    }
}

impl<C> minicbor::Encode<C> for TxIn {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        match self {
            TxIn::Variant0(x) => {
                e.array(2)?;
                e.u8(0)?;
                e.encode_with(x, ctx)?;

                Ok(())
            }
            TxIn::Other(a, b) => {
                e.array(2)?;
                e.u8(*a)?;
                e.encode_with(b, ctx)?;

                Ok(())
            }
        }
    }
}

// tx = [[+ txin], [+ txout], attributes]
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub struct Tx {
    #[n(0)]
    pub inputs: MaybeIndefArray<TxIn>,

    #[n(1)]
    pub outputs: MaybeIndefArray<TxOut>,

    #[n(2)]
    pub attributes: Attributes,
}

// txproof = [u32, hash, hash]
pub type TxProof = (u32, ByronHash, ByronHash);

pub type ValidatorScript = (u16, ByteVec);
pub type RedeemerScript = (u16, ByteVec);

#[derive(Debug, Clone)]
pub enum Twit {
    // [0, #6.24(bytes .cbor ([pubkey, signature]))]
    PkWitness(CborWrap<(PubKey, Signature)>),

    // [1, #6.24(bytes .cbor ([[u16, bytes], [u16, bytes]]))]
    ScriptWitness(CborWrap<(ValidatorScript, RedeemerScript)>),

    // [2, #6.24(bytes .cbor ([pubkey, signature]))]
    RedeemWitness(CborWrap<(PubKey, Signature)>),

    // [u8 .gt 2, encoded-cbor]
    Other(u8, ByteVec),
}

impl<'b, C> minicbor::Decode<'b, C> for Twit {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        d.array()?;

        let variant = d.u8()?;

        match variant {
            0 => Ok(Twit::PkWitness(d.decode_with(ctx)?)),
            1 => Ok(Twit::ScriptWitness(d.decode_with(ctx)?)),
            2 => Ok(Twit::RedeemWitness(d.decode_with(ctx)?)),
            x => Ok(Twit::Other(x, d.decode_with(ctx)?)),
        }
    }
}

impl<C> minicbor::Encode<C> for Twit {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        match self {
            Twit::PkWitness(x) => {
                e.array(2)?;
                e.u8(0)?;
                e.encode_with(x, ctx)?;

                Ok(())
            }
            Twit::ScriptWitness(x) => {
                e.array(2)?;
                e.u8(1)?;
                e.encode_with(x, ctx)?;

                Ok(())
            }
            Twit::RedeemWitness(x) => {
                e.array(2)?;
                e.u8(2)?;
                e.encode(x)?;

                Ok(())
            }
            Twit::Other(a, b) => {
                e.array(2)?;
                e.u8(*a)?;
                e.encode(b)?;

                Ok(())
            }
        }
    }
}

// Shared Seed Computation

// cddl note:
// This is encoded using the 'Binary' instance
// for Scrape.PublicKey
pub type VssPubKey = ByteVec;

// cddl note:
// This is encoded using the 'Binary' instance
// for Scrape.Secret.
pub type VssSec = ByteVec;

// cddl note:
// This is encoded using the 'Binary' instance
// for Scrape.EncryptedSi.
// TODO work out why this seems to be in a length 1 array
pub type VssEnc = MaybeIndefArray<ByteVec>;

// cddl note:
// This is encoded using the 'Binary' instance
// for Scrape.DecryptedShare
pub type VssDec = ByteVec;

// cddl note:
// This is encoded using the
// 'Binary' instance for Scrape.Proof
pub type VssProof = (ByteVec, ByteVec, ByteVec, MaybeIndefArray<ByteVec>);

//ssccomm = [pubkey, [{vsspubkey => vssenc},vssproof], signature]
pub type SscComm = (
    PubKey,
    (KeyValuePairs<VssPubKey, VssEnc>, VssProof),
    Signature,
);

//ssccomms = #6.258([* ssccomm])
pub type SscComms = TagWrap<MaybeIndefArray<SscComm>, 258>;

// sscopens = {stakeholderid => vsssec}
pub type SscOpens = KeyValuePairs<StakeholderId, VssSec>;

// sscshares = {addressid => [addressid, [* vssdec]]}
pub type SscShares = KeyValuePairs<AddressId, KeyValuePairs<AddressId, MaybeIndefArray<VssDec>>>;

// CDDL says: ssccert = [vsspubkey, pubkey, epochid, signature]
// this is what seems to work: ssccert = [vsspubkey, epochid, pubkey, signature]
pub type SscCert = (VssPubKey, EpochId, PubKey, Signature);

// ssccerts = #6.258([* ssccert])
pub type SscCerts = TagWrap<MaybeIndefArray<SscCert>, 258>;

#[derive(Debug, Clone)]
pub enum Ssc {
    Variant0(SscComms, SscCerts),
    Variant1(SscOpens, SscCerts),
    Variant2(SscShares, SscCerts),
    Variant3(SscCerts),
}

impl<'b, C> minicbor::Decode<'b, C> for Ssc {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        d.array()?;

        let variant = d.u8()?;

        match variant {
            0 => Ok(Ssc::Variant0(d.decode_with(ctx)?, d.decode_with(ctx)?)),
            1 => Ok(Ssc::Variant1(d.decode_with(ctx)?, d.decode_with(ctx)?)),
            2 => Ok(Ssc::Variant2(d.decode_with(ctx)?, d.decode_with(ctx)?)),
            3 => Ok(Ssc::Variant3(d.decode_with(ctx)?)),
            _ => Err(minicbor::decode::Error::message("invalid variant for ssc")),
        }
    }
}

impl<C> minicbor::Encode<C> for Ssc {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        match self {
            Ssc::Variant0(a, b) => {
                e.array(3)?;
                e.u8(0)?;
                e.encode_with(a, ctx)?;
                e.encode_with(b, ctx)?;

                Ok(())
            }
            Ssc::Variant1(a, b) => {
                e.array(3)?;
                e.u8(1)?;
                e.encode_with(a, ctx)?;
                e.encode_with(b, ctx)?;

                Ok(())
            }
            Ssc::Variant2(a, b) => {
                e.array(3)?;
                e.u8(2)?;
                e.encode_with(a, ctx)?;
                e.encode_with(b, ctx)?;

                Ok(())
            }
            Ssc::Variant3(x) => {
                e.array(2)?;
                e.u8(3)?;
                e.encode_with(x, ctx)?;

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SscProof {
    Variant0(ByronHash, ByronHash),
    Variant1(ByronHash, ByronHash),
    Variant2(ByronHash, ByronHash),
    Variant3(ByronHash),
}

impl<'b, C> minicbor::Decode<'b, C> for SscProof {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        d.array()?;

        let variant = d.u8()?;

        match variant {
            0 => Ok(SscProof::Variant0(d.decode_with(ctx)?, d.decode_with(ctx)?)),
            1 => Ok(SscProof::Variant1(d.decode_with(ctx)?, d.decode_with(ctx)?)),
            2 => Ok(SscProof::Variant2(d.decode_with(ctx)?, d.decode_with(ctx)?)),
            3 => Ok(SscProof::Variant3(d.decode_with(ctx)?)),
            _ => Err(minicbor::decode::Error::message(
                "invalid variant for sscproof",
            )),
        }
    }
}

impl<C> minicbor::Encode<C> for SscProof {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        match self {
            SscProof::Variant0(a, b) => {
                e.array(3)?;
                e.u8(0)?;
                e.encode_with(a, ctx)?;
                e.encode_with(b, ctx)?;

                Ok(())
            }
            SscProof::Variant1(a, b) => {
                e.array(3)?;
                e.u8(1)?;
                e.encode_with(a, ctx)?;
                e.encode_with(b, ctx)?;

                Ok(())
            }
            SscProof::Variant2(a, b) => {
                e.array(3)?;
                e.u8(2)?;
                e.encode_with(a, ctx)?;
                e.encode_with(b, ctx)?;

                Ok(())
            }
            SscProof::Variant3(x) => {
                e.array(2)?;
                e.u8(3)?;
                e.encode_with(x, ctx)?;

                Ok(())
            }
        }
    }
}

// Delegation

#[derive(Debug, Encode, Decode, Clone)]
pub struct Dlg {
    #[n(0)]
    pub epoch: EpochId,

    #[n(1)]
    pub issuer: PubKey,

    #[n(2)]
    pub delegate: PubKey,

    #[n(3)]
    pub certificate: Signature,
}

pub type DlgSig = (Dlg, Signature);

#[derive(Debug, Encode, Decode, Clone)]
pub struct Lwdlg {
    #[n(0)]
    pub epoch_range: (EpochId, EpochId),

    #[n(1)]
    pub issuer: PubKey,

    #[n(2)]
    pub delegate: PubKey,

    #[n(3)]
    pub certificate: Signature,
}

pub type LwdlgSig = (Lwdlg, Signature);

// Updates

pub type BVer = (u16, u16, u8);

#[derive(Debug, Clone)]
pub enum TxFeePol {
    //[0, #6.24(bytes .cbor ([bigint, bigint]))]
    Variant0(CborWrap<(i64, i64)>),

    // [u8 .gt 0, encoded-cbor]
    Other(u8, ByteVec),
}

impl<'b, C> minicbor::Decode<'b, C> for TxFeePol {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        d.array()?;

        let variant = d.u8()?;

        match variant {
            0 => Ok(TxFeePol::Variant0(d.decode_with(ctx)?)),
            x => Ok(TxFeePol::Other(x, d.decode_with(ctx)?)),
        }
    }
}

impl<C> minicbor::Encode<C> for TxFeePol {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        match self {
            TxFeePol::Variant0(x) => {
                e.array(2)?;
                e.u8(0)?;
                e.encode_with(x, ctx)?;

                Ok(())
            }
            TxFeePol::Other(a, b) => {
                e.array(2)?;
                e.u8(*a)?;
                e.encode_with(b, ctx)?;

                Ok(())
            }
        }
    }
}

#[derive(Debug, Encode, Decode, Clone)]
pub struct BVerMod {
    #[n(0)]
    pub script_version: ZeroOrOneArray<u16>,

    #[n(1)]
    pub slot_duration: ZeroOrOneArray<u64>,

    #[n(2)]
    pub max_block_size: ZeroOrOneArray<u64>,

    #[n(3)]
    pub max_header_size: ZeroOrOneArray<u64>,

    #[n(4)]
    pub max_tx_size: ZeroOrOneArray<u64>,

    #[n(5)]
    pub max_proposal_size: ZeroOrOneArray<u64>,

    #[n(6)]
    pub mpc_thd: ZeroOrOneArray<u64>,

    #[n(7)]
    pub heavy_del_thd: ZeroOrOneArray<u64>,

    #[n(8)]
    pub update_vote_thd: ZeroOrOneArray<u64>,

    #[n(9)]
    pub update_proposal_thd: ZeroOrOneArray<u64>,

    #[n(10)]
    pub update_implicit: ZeroOrOneArray<u64>,

    #[n(11)]
    pub soft_fork_rule: ZeroOrOneArray<(u64, u64, u64)>,

    #[n(12)]
    pub tx_fee_policy: ZeroOrOneArray<TxFeePol>,

    #[n(13)]
    pub unlock_stake_epoch: ZeroOrOneArray<EpochId>,
}

pub type UpData = (ByronHash, ByronHash, ByronHash, ByronHash);

#[derive(Debug, Encode, Decode, Clone)]
pub struct UpProp {
    #[n(0)]
    pub block_version: Option<BVer>,

    #[n(1)]
    pub block_version_mod: Option<BVerMod>,

    #[n(2)]
    pub software_version: Option<(String, u32)>,

    #[n(3)]
    // HACK: CDDL show a tag wrap 258, but chain data doesn't present the tag
    //pub data: TagWrap<(String, UpData), 258>,
    pub data: KeyValuePairs<String, UpData>,

    #[n(4)]
    pub attributes: Option<Attributes>,

    #[n(5)]
    pub from: Option<PubKey>,

    #[n(6)]
    pub signature: Option<Signature>,
}

#[derive(Debug, Encode, Decode, Clone)]
pub struct UpVote {
    #[n(0)]
    pub voter: PubKey,

    #[n(1)]
    pub proposal_id: UpdId,

    #[n(2)]
    pub vote: bool,

    #[n(3)]
    pub signature: Signature,
}

#[derive(Debug, Encode, Decode, Clone)]
pub struct Up {
    #[n(0)]
    pub proposal: ZeroOrOneArray<UpProp>,

    #[n(1)]
    pub votes: MaybeIndefArray<UpVote>,
}

// Blocks

pub type Difficulty = MaybeIndefArray<u64>;

#[derive(Debug, Clone)]
pub enum BlockSig {
    Signature(Signature),
    LwdlgSig(LwdlgSig),
    DlgSig(DlgSig),
}

impl<'b, C> minicbor::Decode<'b, C> for BlockSig {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        d.array()?;

        let variant = d.u8()?;

        match variant {
            0 => Ok(BlockSig::Signature(d.decode_with(ctx)?)),
            1 => Ok(BlockSig::LwdlgSig(d.decode_with(ctx)?)),
            2 => Ok(BlockSig::DlgSig(d.decode_with(ctx)?)),
            _ => Err(minicbor::decode::Error::message(
                "unknown variant for blocksig",
            )),
        }
    }
}

impl<C> minicbor::Encode<C> for BlockSig {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        match self {
            BlockSig::Signature(x) => {
                e.array(2)?;
                e.u8(0)?;
                e.encode(x)?;

                Ok(())
            }
            BlockSig::LwdlgSig(x) => {
                e.array(2)?;
                e.u8(1)?;
                e.encode(x)?;

                Ok(())
            }
            BlockSig::DlgSig(x) => {
                e.array(2)?;
                e.u8(2)?;
                e.encode(x)?;

                Ok(())
            }
        }
    }
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct BlockCons(
    #[n(0)] pub SlotId,
    #[n(1)] pub PubKey,
    #[n(2)] pub Difficulty,
    #[n(3)] pub BlockSig,
);

#[derive(Encode, Decode, Debug, Clone)]
pub struct BlockHeadEx {
    #[n(0)]
    pub block_version: BVer,

    #[n(1)]
    pub software_version: (String, u32),

    #[n(2)]
    pub attributes: Option<Attributes>,

    #[n(3)]
    pub extra_proof: ByronHash,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct BlockProof {
    #[n(0)]
    pub tx_proof: TxProof,

    #[n(1)]
    pub ssc_proof: SscProof,

    #[n(2)]
    pub dlg_proof: ByronHash,

    #[n(3)]
    pub upd_proof: ByronHash,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct BlockHead {
    #[n(0)]
    pub protocol_magic: u32,

    #[n(1)]
    pub prev_block: BlockId,

    #[n(2)]
    pub body_proof: BlockProof,

    #[n(3)]
    pub consensus_data: BlockCons,

    #[n(4)]
    pub extra_data: BlockHeadEx,
}

pub type Witnesses = MaybeIndefArray<Twit>;

#[derive(Debug, Encode, Decode, Clone)]
pub struct PseudoTxPayload<T1, T2>
where
    T1: std::clone::Clone,
    T2: std::clone::Clone,
{
    #[b(0)]
    pub transaction: T1,

    #[n(1)]
    pub witness: T2,
}

pub type TxPayload = PseudoTxPayload<Tx, Witnesses>;

pub type MintedTxPayload<'b> = PseudoTxPayload<KeepRaw<'b, Tx>, KeepRaw<'b, Witnesses>>;

pub type RawTxPayload<'b> = PseudoTxPayload<OnlyRaw<'b, Tx>, OnlyRaw<'b, Witnesses>>;

#[derive(Encode, Decode, Debug, Clone)]
pub struct PseudoBlockBody<T1>
where
    T1: std::clone::Clone,
{
    #[b(0)]
    pub tx_payload: MaybeIndefArray<T1>,

    #[b(1)]
    pub ssc_payload: Ssc,

    #[b(2)]
    pub dlg_payload: MaybeIndefArray<Dlg>,

    #[b(3)]
    pub upd_payload: Up,
}

pub type BlockBody = PseudoBlockBody<TxPayload>;

pub type MintedBlockBody<'b> = PseudoBlockBody<MintedTxPayload<'b>>;

pub type RawBlockBody<'b> = PseudoBlockBody<RawTxPayload<'b>>;

// Epoch Boundary Blocks

#[derive(Encode, Decode, Debug, Clone)]
pub struct EbbCons {
    #[n(0)]
    pub epoch_id: EpochId,

    #[n(1)]
    pub difficulty: Difficulty,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct EbbHead {
    #[n(0)]
    pub protocol_magic: u32,

    #[n(1)]
    pub prev_block: BlockId,

    #[n(2)]
    pub body_proof: ByronHash,

    #[n(3)]
    pub consensus_data: EbbCons,

    #[n(4)]
    pub extra_data: (Attributes,),
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct PseudoBlock<T1, T2>
where
    T1: std::clone::Clone,
    T2: std::clone::Clone,
{
    #[b(0)]
    pub header: T1,

    #[b(1)]
    pub body: T2,

    #[n(2)]
    pub extra: MaybeIndefArray<Attributes>,
}

pub type Block = PseudoBlock<BlockHead, BlockBody>;

pub type MintedBlock<'b> = PseudoBlock<KeepRaw<'b, BlockHead>, MintedBlockBody<'b>>;

pub type RawBlock<'b> = PseudoBlock<OnlyRaw<'b, BlockHead>, RawBlockBody<'b>>;

#[derive(Encode, Decode, Debug, Clone)]
pub struct PseudoEbBlock<T1>
where
    T1: std::clone::Clone,
{
    #[n(0)]
    pub header: T1,

    #[n(1)]
    pub body: MaybeIndefArray<StakeholderId>,

    #[n(2)]
    pub extra: MaybeIndefArray<Attributes>,
}

pub type EbBlock = PseudoEbBlock<EbbHead>;

pub type MintedEbBlock<'b> = PseudoEbBlock<KeepRaw<'b, EbbHead>>;

pub type RawEbBlock<'b> = PseudoEbBlock<OnlyRaw<'b, EbbHead>>;

#[cfg(test)]
mod tests {
    use super::{BlockHead, EbBlock, MintedBlock, MintedEbBlock, RawBlock, RawEbBlock};
    use pallas_codec::minicbor::{self, to_vec};

    const ED_TEST_BLOCKS: [&'static str; 1] = [include_str!("../../../test_data/genesis.block")];

    const TEST_BLOCKS: [&'static str; 8] = [
        include_str!("../../../test_data/byron1.block"),
        include_str!("../../../test_data/byron2.block"),
        include_str!("../../../test_data/byron3.block"),
        include_str!("../../../test_data/byron4.block"),
        include_str!("../../../test_data/byron5.block"),
        include_str!("../../../test_data/byron6.block"),
        include_str!("../../../test_data/byron7.block"),
        include_str!("../../../test_data/byron8.block"),
    ];

    #[test]
    fn boundary_block_isomorphic_decoding_encoding() {
        type BlockWrapper = (u16, EbBlock);

        for (idx, block_str) in ED_TEST_BLOCKS.iter().enumerate() {
            println!("decoding test block {}", idx + 1);
            let bytes = hex::decode(block_str).unwrap_or_else(|_| panic!("bad block file {idx}"));

            let block: BlockWrapper = minicbor::decode(&bytes[..])
                .unwrap_or_else(|_| panic!("error decoding cbor for file {idx}"));

            let bytes2 = to_vec(block)
                .unwrap_or_else(|_| panic!("error encoding block cbor for file {idx}"));

            assert_eq!(hex::encode(bytes), hex::encode(bytes2));
        }
    }

    #[test]
    fn boundary_minted_block_isomorphic_decoding_encoding() {
        type BlockWrapper<'b> = (u16, MintedEbBlock<'b>);

        for (idx, block_str) in ED_TEST_BLOCKS.iter().enumerate() {
            println!("decoding test block {}", idx + 1);
            let bytes = hex::decode(block_str).unwrap_or_else(|_| panic!("bad block file {idx}"));

            let block: BlockWrapper = minicbor::decode(&bytes[..])
                .unwrap_or_else(|_| panic!("error decoding cbor for file {idx}"));

            let bytes2 = to_vec(block)
                .unwrap_or_else(|_| panic!("error encoding block cbor for file {idx}"));

            assert_eq!(hex::encode(bytes), hex::encode(bytes2));
        }
    }

    #[test]
    fn boundary_raw_block_isomorphic_decoding_encoding() {
        type BlockWrapper<'b> = (u16, RawEbBlock<'b>);

        for (idx, block_str) in ED_TEST_BLOCKS.iter().enumerate() {
            println!("decoding test block {}", idx + 1);
            let bytes = hex::decode(block_str).unwrap_or_else(|_| panic!("bad block file {idx}"));

            let block: BlockWrapper = minicbor::decode(&bytes[..])
                .unwrap_or_else(|_| panic!("error decoding cbor for file {idx}"));

            let bytes2 = to_vec(block)
                .unwrap_or_else(|_| panic!("error encoding block cbor for file {idx}"));

            assert_eq!(hex::encode(bytes), hex::encode(bytes2));
        }
    }

    #[test]
    fn main_minted_block_isomorphic_decoding_encoding() {
        type BlockWrapper<'b> = (u16, MintedBlock<'b>);

        for (idx, block_str) in TEST_BLOCKS.iter().enumerate() {
            println!("decoding test block {}", idx + 1);
            let bytes = hex::decode(block_str).unwrap_or_else(|_| panic!("bad block file {idx}"));

            let block: BlockWrapper = minicbor::decode(&bytes[..])
                .unwrap_or_else(|_| panic!("error decoding cbor for file {idx}"));

            let bytes2 = to_vec(block)
                .unwrap_or_else(|_| panic!("error encoding block cbor for file {idx}"));

            assert_eq!(hex::encode(bytes), hex::encode(bytes2));
        }
    }

    #[test]
    fn main_raw_block_isomorphic_decoding_encoding() {
        type BlockWrapper<'b> = (u16, RawBlock<'b>);

        for (idx, block_str) in TEST_BLOCKS.iter().enumerate() {
            println!("decoding test block {}", idx + 1);
            let bytes = hex::decode(block_str).unwrap_or_else(|_| panic!("bad block file {idx}"));

            let block: BlockWrapper = minicbor::decode(&bytes[..])
                .unwrap_or_else(|_| panic!("error decoding cbor for file {idx}"));

            let bytes2 = to_vec(block)
                .unwrap_or_else(|_| panic!("error encoding block cbor for file {idx}"));

            assert_eq!(hex::encode(bytes), hex::encode(bytes2));
        }
    }

    #[test]
    fn header_isomorphic_decoding_encoding() {
        let subjects = [include_str!("../../../test_data/byron1.header")];

        for (idx, str) in subjects.iter().enumerate() {
            println!("decoding test header {}", idx + 1);
            let bytes = hex::decode(str).unwrap_or_else(|_| panic!("bad header file {idx}"));

            let block: BlockHead = minicbor::decode(&bytes[..])
                .unwrap_or_else(|_| panic!("error decoding cbor for file {idx}"));

            let bytes2 = to_vec(block)
                .unwrap_or_else(|_| panic!("error encoding header cbor for file {idx}"));

            assert_eq!(bytes, bytes2);
        }
    }
}
