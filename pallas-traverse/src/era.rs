use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

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

impl Era {
    #[allow(clippy::match_like_matches_macro)]
    pub fn has_feature(&self, feature: Feature) -> bool {
        match feature {
            Feature::Staking => self.ge(&Era::Shelley),
            Feature::MultiAssets => self.ge(&Era::Mary),
            Feature::TimeLocks => self.ge(&Era::Allegra),
            Feature::SmartContracts => self.ge(&Era::Alonzo),
            Feature::CIP31 => self.ge(&Era::Babbage),
            Feature::CIP32 => self.ge(&Era::Babbage),
            Feature::CIP33 => self.ge(&Era::Babbage),
            Feature::CIP1694 => self.ge(&Era::Conway),
        }
    }
}

// for consistency, we use the same tag convention used by the node's cbor
// encoding
impl TryFrom<u16> for Era {
    type Error = crate::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Era::Byron),
            1 => Ok(Era::Byron),
            2 => Ok(Era::Shelley),
            3 => Ok(Era::Allegra),
            4 => Ok(Era::Mary),
            5 => Ok(Era::Alonzo),
            6 => Ok(Era::Babbage),
            7 => Ok(Era::Conway),
            x => Err(crate::Error::UnknownEra(x)),
        }
    }
}

impl From<Era> for u16 {
    fn from(other: Era) -> Self {
        match other {
            Era::Byron => 1,
            Era::Shelley => 2,
            Era::Allegra => 3,
            Era::Mary => 4,
            Era::Alonzo => 5,
            Era::Babbage => 6,
            Era::Conway => 7,
        }
    }
}

impl Display for Era {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Era::Byron => write!(f, "Byron"),
            Era::Shelley => write!(f, "Shelley"),
            Era::Allegra => write!(f, "Allegra"),
            Era::Mary => write!(f, "Mary"),
            Era::Alonzo => write!(f, "Alonzo"),
            Era::Babbage => write!(f, "Babbage"),
            Era::Conway => write!(f, "Conway"),
        }
    }
}
