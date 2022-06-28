use crate::{atom::Atom, cell::Cell, noun::Noun};

/// Convert a noun into the implementing type.
pub trait FromNoun<A, C, N>
where
    A: Atom,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    fn from_noun_ref(noun: &N) -> Result<Self, ()>;

    fn from_noun(noun: N) -> Result<Self, ()>;
}

/// Convert the implementing type into a noun.
pub trait IntoNoun<A, C, N>
where
    A: Atom,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    fn as_noun(&self) -> Result<N, ()>;

    fn as_noun_unchecked(&self) -> N;

    fn into_noun(self) -> Result<N, ()>;

    fn into_noun_unchecked(self) -> N;
}
