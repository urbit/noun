//! Assorted [`Cell`] implementations.

use crate::{atom::types::VecAtom, cell::Cell, noun::types::Noun};
use std::{hash::Hash, rc::Rc};

#[derive(Clone, Hash, Debug, Eq)]
pub struct RcCell {
    head: Rc<Noun<VecAtom, Self>>,
    tail: Rc<Noun<VecAtom, Self>>,
}

impl Cell<VecAtom, Noun<VecAtom, Self>> for RcCell {
    type Head = Rc<Noun<VecAtom, Self>>;
    type Tail = Self::Head;

    fn from_parts(head: Self::Head, tail: Self::Tail) -> Self {
        Self { head, tail }
    }

    fn from_pair(head: Rc<Noun<VecAtom, Self>>, tail: Rc<Noun<VecAtom, Self>>) -> Self {
        Self::from_parts(head, tail)
    }

    fn head(&self) -> &Self::Head {
        &self.head
    }

    fn tail(&self) -> &Self::Tail {
        &self.tail
    }

    fn head_as_noun(&self) -> &Noun<VecAtom, Self> {
        &*self.head
    }

    fn tail_as_noun(&self) -> &Noun<VecAtom, Self> {
        &*self.tail
    }

    fn into_parts(self) -> (Self::Head, Self::Tail) {
        (self.head, self.tail)
    }

    fn into_noun(self) -> Noun<VecAtom, Self> {
        Noun::Cell(self)
    }
}

impl PartialEq for RcCell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}
