use std::borrow::Cow;

use pallas_primitives::{alonzo, conway};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiEraCert<'b> {
    NotApplicable,
    AlonzoCompatible(Box<Cow<'b, alonzo::Certificate>>),
    Conway(Box<Cow<'b, conway::Certificate>>),
}

impl<'b> MultiEraCert<'b> {
    pub fn as_alonzo(&self) -> Option<&alonzo::Certificate> {
        match self {
            MultiEraCert::AlonzoCompatible(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_conway(&self) -> Option<&conway::Certificate> {
        match self {
            MultiEraCert::Conway(x) => Some(x),
            _ => None,
        }
    }
}
