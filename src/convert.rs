//! Conversions to and from nouns.

use crate::Noun;
use std::fmt::{self, Display, Formatter};

/// Errors that occur when converting from a noun.
#[derive(Debug)]
pub enum Error {
    AtomToUint,
    AtomToStr,
    ImplType,
    MissingValue,
    UnexpectedAtom,
    UnexpectedCell,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::AtomToUint => write!(
                f,
                "the atom is too large to fit in the unsigned integer type"
            ),
            Self::AtomToStr => write!(f, "the atom is not composed of valid UTF-8 bytes"),
            Self::ImplType => write!(f, "an error specific to the implementing type occurred"),
            Self::MissingValue => write!(f, "the noun does not have a value at this axis"),
            Self::UnexpectedAtom => write!(f, "an atom was encountered when a cell was expected"),
            Self::UnexpectedCell => write!(f, "a cell was encountered when an atom was expected"),
        }
    }
}

/// Converts a noun to this type.
pub trait FromNoun: Sized {
    /// Convert a noun into this type, consuming the noun.
    fn from_noun(noun: &Noun) -> Result<Self, Error>;
}

/// Converts this type into a noun.
pub trait IntoNoun {
    /// The error to return if this type could not be converted into a noun.
    type Error;

    /// Converts this type into a noun, consuming the implementing type, and returning an error if
    /// the type cannot be converted.
    fn into_noun(self) -> Result<Noun, Self::Error>;
}
