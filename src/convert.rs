//! Conversions to and from noun types.

use crate::noun::Noun;

#[derive(Debug)]
pub enum Error {
    AtomToUint,
    AtomToStr,
    DestType,
    NotEnoughElements,
    UnexpectedAtom,
    UnexpectedCell,
}

/// Convert from a noun.
pub trait FromNoun: Sized {
    /// Convert a noun into the implementing type.
    fn from_noun_ref(noun: &Noun) -> Result<Self, Error>;

    /// Convert a noun into the implementing type, consuming the noun.
    fn from_noun(noun: Noun) -> Result<Self, Error>;
}

/// Convert into a noun.
pub trait IntoNoun {
    type Error;

    /// Convert the implementing type into a noun, returning an error if the type cannot be
    /// converted.
    fn to_noun(&self) -> Result<Noun, Self::Error>;

    /// Convert the implementing type into a noun, consuming the noun, and returning an error if the type cannot be
    /// converted.
    fn into_noun(self) -> Result<Noun, Self::Error>;
}
