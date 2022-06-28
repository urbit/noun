//! Conversions to and from noun types.

use crate::{atom::Atom, cell::Cell, noun::Noun};

/// Convert from a noun.
pub trait FromNoun<A, C, N>
where
    A: Atom,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    /// Convert a noun into the implementing type.
    fn from_noun_ref(noun: &N) -> Result<Self, ()>;

    /// Convert a noun into the implementing type, consuming the noun.
    fn from_noun(noun: N) -> Result<Self, ()>;
}

/// Convert into a noun.
pub trait IntoNoun<A, C, N>
where
    A: Atom,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    /// Convert the implementing type into a noun, returning an error if the type cannot be
    /// converted.
    fn to_noun(&self) -> Result<N, ()>;

    /// Convert the implementing type into a noun, panicking if the type cannot be converted.
    fn to_noun_unchecked(&self) -> N;

    /// Convert the implementing type into a noun, consuming the noun, and returning an error if the type cannot be
    /// converted.
    fn into_noun(self) -> Result<N, ()>;

    /// Convert the implementing type into a noun, consuming the noun, and panicking if the type cannot be
    /// converted.
    fn into_noun_unchecked(self) -> N;
}
