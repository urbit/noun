//! Serialization and deserialization.

use crate::atom::Atom;
use std::{
    fmt::{self, Display, Formatter},
    result,
};

/// Errors that occur when serializing/deserializing.
#[derive(Debug)]
pub enum Error {
    AtomConstruction,
    CacheMiss,
    InvalidBackref,
    InvalidLen,
    InvalidTag,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self {
            Self::AtomConstruction => write!(f, "the bit could not be pushed"),
            Self::CacheMiss => write!(f, "the cache does not contain the requested value"),
            Self::InvalidBackref => write!(f, "the backreference is malformed"),
            Self::InvalidLen => write!(f, "the encoded length is corrupt"),
            Self::InvalidTag => write!(f, "the tag is corrupt"),
        }
    }
}

/// A specialized `Result` type for serialization/deserialization operations that return
/// `serdes::Error` on error.
pub type Result<T> = std::result::Result<T, Error>;

/// Serialize this noun type into a bitstream.
pub trait Jam {
    /// Performs bitwise serialization of this noun type in accordance with the [jam] protocol,
    /// returning the resulting bitstream as an atom.
    ///
    /// TODO
    ///
    /// [jam]: https://developers.urbit.org/reference/hoon/stdlib/2p#jam
    fn jam(self) -> Atom;
}

/// Deserialize a bitstream into this noun type.
pub trait Cue: Sized {
    /// Performs bitwise deserialization of a bitstream in accordance with the [cue] protocol,
    /// returning the resulting noun type.
    ///
    /// TODO
    ///
    /// [cue]: https://developers.urbit.org/reference/hoon/stdlib/2p#cue
    fn cue(jammed_noun: Atom) -> Result<Self>;
}
