pub mod r#enum;

use bitstream_io::{BitRead, BitWrite};
use std::hash::Hash;

pub trait Atom: IntoNoun + Sized {
    type Error;

    fn as_bytes(&self) -> &[u8];
}

pub trait Cell: IntoNoun + Sized {
    type Noun: Noun;

    fn into_parts(self) -> (Option<<Self as Cell>::Noun>, Option<<Self as Cell>::Noun>);
}

pub trait Noun: Hash + Sized {
    type Atom: Atom;
    type Cell: Cell;
    type Error;

    fn get(&self, idx: usize) -> Option<&Self>;

    fn into_atom(self) -> Result<Self::Atom, <Self as Noun>::Error>;

    fn into_cell(self) -> Result<Self::Cell, <Self as Noun>::Error>;
}

/// Unifying equality.
pub trait UnifyEq: Eq {
    type Ctx;

    fn eq(&self, other: &Self, _ctx: Self::Ctx) -> bool;
}

pub trait Cue: Noun + Sized {
    type Error;

    fn cue(src: impl BitRead) -> Result<Self, <Self as Cue>::Error>;

    /// Read the length of an atom or backreference.
    fn len(src: &mut impl BitRead) -> Result<(Self, usize), ()>;
}

pub trait Jam: Noun + Sized {
    type Error;

    fn jam(self, sink: &mut impl BitWrite) -> Result<(), <Self as Jam>::Error>;
}

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
