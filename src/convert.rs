//! Conversions to and from nouns.

use crate::Nounish;
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

/// Convert a noun into this type.
pub trait FromNoun<N: Nounish>: Sized {
    /// Converts a noun to this type, panicking if the conversion failed.
    fn from_noun(noun: N) -> Self;
}

/// Attempt to convert a noun into this type.
pub trait TryFromNoun<N: Nounish>: Sized {
    /// Converts a noun to this type, returning an error if the conversion failed.

    fn try_from_noun(noun: N) -> Result<Self, Error>;
}

/// Convert this type into a noun.
pub trait IntoNoun<N: Nounish> {
    /// Converts this type into a noun, panicking if the conversion failed.
    fn into_noun(self) -> N;
}

/// Attempt to convert this type into a noun.
pub trait TryIntoNoun<N: Nounish> {
    /// The error to return if this type could not be converted into a noun.
    type Error;

    /// Converts this type into a noun, returning an error if the conversion failed.
    fn try_into_noun(self) -> Result<N, Self::Error>;
}
