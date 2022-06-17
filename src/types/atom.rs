use crate::{
    types::{cell::Cell, noun::Noun},
    Atom as _Atom, IntoNoun,
};
use std::{hash::Hash, ops::Add};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Atom(Vec<u8>);

impl Add for Atom {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        self
    }
}

impl _Atom<Cell, Noun> for Atom {
    fn new(val: Vec<u8>) -> Self {
        Self(val)
    }

    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl IntoNoun<Self, Cell, Noun> for Atom {
    fn into_noun(self) -> Result<Noun, ()> {
        Ok(Noun::Atom(self))
    }
}
