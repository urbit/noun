//! A [noun] is a finite size binary tree with atoms as leaves.
//!
//! [noun]: https://urbit.org/docs/glossary/noun

pub mod types;

use crate::{atom::Atom, cell::Cell};
use std::{fmt::Debug, hash::Hash};

/// Interface to the noun data structure.
pub trait Noun<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Debug + Eq + Hash + Sized,
{
    fn get(&self, idx: usize) -> Option<&Self>;

    fn as_atom(&self) -> Result<&A, ()>;

    fn as_cell(&self) -> Result<&C, ()>;

    fn into_atom(self) -> Result<A, Self>;

    fn into_cell(self) -> Result<C, Self>;
}

/// Convert a noun into the implementing type.
pub trait FromNoun<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    fn from_noun_ref(noun: &N) -> Result<Self, ()>;

    fn from_noun(noun: N) -> Result<Self, ()> {
        Self::from_noun_ref(&noun)
    }
}

/// Convert the implementing type into a noun.
pub trait IntoNoun<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    fn as_noun(&self) -> Result<N, ()>;

    fn into_noun(self) -> Result<N, ()> {
        Self::as_noun(&self)
    }
}
