//! Assorted [`Cell`] implementations.

use crate::{atom::types::VecAtom, cell::Cell, convert::IntoNoun, noun::types::EnumNoun};
use std::{hash::Hash, rc::Rc};

#[derive(Clone, Hash, Debug, Eq)]
pub struct RcCell {
    head: Rc<EnumNoun<VecAtom, Self>>,
    tail: Rc<EnumNoun<VecAtom, Self>>,
}

impl Cell for RcCell {
    type Head = Rc<EnumNoun<VecAtom, Self>>;
    type Tail = Self::Head;

    fn new(head: Self::Head, tail: Self::Tail) -> Self {
        Self { head, tail }
    }

    fn head(&self) -> &Self::Head {
        &self.head
    }

    fn tail(&self) -> &Self::Tail {
        &self.tail
    }

    fn into_parts(self) -> (Self::Head, Self::Tail) {
        (self.head, self.tail)
    }
}

impl IntoNoun<VecAtom, Self, EnumNoun<VecAtom, Self>> for RcCell {
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

impl PartialEq for RcCell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}
