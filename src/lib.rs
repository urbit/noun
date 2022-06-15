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
pub trait FromNoun: Sized {
    type Error;
    type Noun: Noun;

    fn from_noun(noun: Self::Noun) -> Result<Self, Self::Error>;
}

/// Convert the implementing type into a noun.
pub trait IntoNoun: Sized {
    type Error;
    type Noun: Noun;

    fn into_noun(self) -> Result<Self::Noun, Self::Error>;
}

pub trait Noun: Cue + Jam + Mug + Sized {
    type Atom: Atom;
    type Cell: Cell;
    type Error;

    fn into_atom(self) -> Result<Self::Atom, <Self as Noun>::Error>;

    fn into_cell(self) -> Result<Self::Cell, <Self as Noun>::Error>;
}

pub trait Mug: Eq + Hash {}
