pub mod atom;
pub mod cell;
pub mod serdes;

use crate::{
    atom::Atom,
    cell::Cell,
    serdes::{Cue, Jam},
};
use std::hash::Hash;

/// Convert a noun into the implementing type.
pub trait FromNoun<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    type Error;

    fn from_noun(noun: N) -> Result<Self, Self::Error>;
}

/// Convert the implementing type into a noun.
pub trait IntoNoun<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    type Error;

    fn into_noun(self) -> Result<N, Self::Error>;
}

pub trait Noun<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Cue<A, C, Self> + Jam<A, C, Self> + Eq + Hash + Sized,
{
    type Error;

    fn into_atom(self) -> Result<A, <Self as Noun<A, C>>::Error>;

    fn into_cell(self) -> Result<C, <Self as Noun<A, C>>::Error>;
}

