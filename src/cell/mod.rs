//! An ordered pair of nouns.
//!
//! Cells have a head and a tail and can be compared to one another.

pub mod types;

use crate::{atom::Atom, noun::Noun};
use std::{fmt::Debug, rc::Rc};

/// Interface to the cell data structure.
pub trait Cell<A, N>
where
    A: Atom,
    N: Noun<A, Self>,
    Self: Debug + Eq + Sized,
{
    type Head;
    type Tail;

    /// Create a new cell from a head and a tail.
    fn from_parts(head: Self::Head, tail: Self::Tail) -> Self;

    /// Create a new cell from a pair of reference-counted nouns.
    fn from_pair(head: Rc<N>, tail: Rc<N>) -> Self;

    /// Returns the head of the cell.
    fn head(&self) -> &Self::Head;

    /// Returns the tail of the cell.
    fn tail(&self) -> &Self::Tail;

    /// Returns the head of the cell as a noun.
    fn head_as_noun(&self) -> &N;

    /// Returns the tail of the cell as a noun.
    fn tail_as_noun(&self) -> &N;

    /// Converts a cell into its head and tail.
    fn as_parts(&self) -> (&Self::Head, &Self::Tail) {
        (self.head(), self.tail())
    }

    /// Converts a cell into its head and tail as nouns.
    fn as_pair(&self) -> (&N, &N) {
        (self.head_as_noun(), self.tail_as_noun())
    }

    /// Converts a cell of the form `[h th tt]` into a 3-element tuple of nouns.
    fn as_triple(&self) -> Result<(&N, &N, &N), ()> {
        let h = self.head_as_noun();
        let (th, tt) = self.tail_as_noun().as_cell()?.as_pair();
        Ok((h, th, tt))
    }

    /// Converts a cell of the form `[h th tth ttt]` into a 4-element tuple of nouns.
    fn as_quad(&self) -> Result<(&N, &N, &N, &N), ()> {
        let (h, th, tt) = self.as_triple()?;
        let (tth, ttt) = tt.as_cell()?.as_pair();
        Ok((h, th, tth, ttt))
    }

    /// Converts a cell of the form `[h th tth ttth tttt]` into a 5-element tuple of nouns.
    fn as_quint(&self) -> Result<(&N, &N, &N, &N, &N), ()> {
        let (h, th, tth, ttt) = self.as_quad()?;
        let (ttth, tttt) = ttt.as_cell()?.as_pair();
        Ok((h, th, tth, ttth, tttt))
    }

    /// Converts a cell into its head and tail, consuming the cell.
    fn into_parts(self) -> (Self::Head, Self::Tail);
}
