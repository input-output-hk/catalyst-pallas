use pallas_crypto::hash::Hash;
use pallas_primitives::alonzo;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraSigners<'b> {
    NotApplicable,
    Empty,
    AlonzoCompatible(&'b alonzo::RequiredSigners),
}

impl Default for MultiEraSigners<'_> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<'b> MultiEraSigners<'b> {
    pub fn as_alonzo(&self) -> Option<&alonzo::RequiredSigners> {
        match self {
            Self::AlonzoCompatible(x) => Some(x),
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::AlonzoCompatible(x) => x.is_empty(),
            _ => true,
        }
    }

    pub fn collect<'a, T>(&'a self) -> T
    where
        T: FromIterator<&'a Hash<28>>,
    {
        match self {
            Self::NotApplicable => std::iter::empty().collect(),
            Self::Empty => std::iter::empty().collect(),
            Self::AlonzoCompatible(x) => x.iter().collect(),
        }
    }
}
