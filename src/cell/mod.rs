//! A cell is an ordered pair of nouns.

pub mod types;

use crate::{atom::Atom, noun::Noun};
use std::rc::Rc;

/// Interface to the cell data structure.
pub trait Cell<A, N>
where
    A: Atom<Self, N>,
    N: Noun<A, Self>,
    Self: Sized,
{
    type Head;
    type Tail;

    fn from_parts(head: Self::Head, tail: Self::Tail) -> Self;

    fn from_pair(head: Rc<N>, tail: Rc<N>) -> Self;

    fn head(&self) -> &Self::Head;

    fn tail(&self) -> &Self::Tail;

    fn head_as_noun(&self) -> &N;

    fn tail_as_noun(&self) -> &N;

    fn as_parts(&self) -> (&Self::Head, &Self::Tail) {
        (self.head(), self.tail())
    }

    /// Unpack a cell of the form `[h t]`.
    fn as_pair(&self) -> (&N, &N) {
        (self.head_as_noun(), self.tail_as_noun())
    }

    /// Unpack a cell of the form `[h th tt]`.
    fn as_triple(&self) -> Result<(&N, &N, &N), ()> {
        let h = self.head_as_noun();
        let (th, tt) = self.tail_as_noun().as_cell()?.as_pair();
        Ok((h, th, tt))
    }

    /// Unpack a cell of the form `[h th tth ttt]`.
    fn as_quad(&self) -> Result<(&N, &N, &N, &N), ()> {
        let (h, th, tt) = self.as_triple()?;
        let (tth, ttt) = tt.as_cell()?.as_pair();
        Ok((h, th, tth, ttt))
    }

    /// Unpack a cell of the form `[h th tth ttth tttt]`.
    fn as_quint(&self) -> Result<(&N, &N, &N, &N, &N), ()> {
        let (h, th, tth, ttt) = self.as_quad()?;
        let (ttth, tttt) = ttt.as_cell()?.as_pair();
        Ok((h, th, tth, ttth, tttt))
    }

    fn into_parts(self) -> (Self::Head, Self::Tail);

    fn into_noun(self) -> N;
}
