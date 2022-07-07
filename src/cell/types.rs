//! Assorted [`Cell`] implementations.

use crate::{
    atom::{types::VecAtom, Atom},
    cell::Cell,
    convert::IntoNoun,
    noun::{types::EnumNoun, Noun},
};
use std::{hash::Hash, rc::Rc};

#[derive(Clone, Hash, Debug, Eq)]
pub struct RcCell<A>
where
    A: Atom,
    EnumNoun<A, Self>: Noun<A, Self>,
    Self: Cell,
{
    head: Rc<EnumNoun<A, Self>>,
    tail: Rc<EnumNoun<A, Self>>,
}

impl Cell for RcCell<VecAtom> {
    type Head = Rc<EnumNoun<VecAtom, Self>>;
    type Tail = Self::Head;

    fn new(head: Self::Head, tail: Self::Tail) -> Self {
        Self { head, tail }
    }

    fn as_parts(&self) -> (&Self::Head, &Self::Tail) {
        (&self.head, &self.tail)
    }

    fn to_parts(&self) -> (Self::Head, Self::Tail) {
        (self.head.clone(), self.tail.clone())
    }

    fn into_parts(self) -> (Self::Head, Self::Tail) {
        (self.head, self.tail)
    }
}

impl IntoNoun<VecAtom, Self, EnumNoun<VecAtom, Self>> for RcCell<VecAtom> {
    fn to_noun(&self) -> Result<EnumNoun<VecAtom, Self>, ()> {
        unimplemented!("An EnumNoun cannot be constructed from &RcCell.");
    }

    fn to_noun_unchecked(&self) -> EnumNoun<VecAtom, Self> {
        unimplemented!("An EnumNoun cannot be constructed from &RcCell.");
    }

    fn into_noun(self) -> Result<EnumNoun<VecAtom, Self>, ()> {
        Ok(self.into_noun_unchecked())
    }

    fn into_noun_unchecked(self) -> EnumNoun<VecAtom, Self> {
        EnumNoun::Cell(self)
    }
}

impl PartialEq for RcCell<VecAtom> {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}
