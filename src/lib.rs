pub mod r#enum;

use std::hash::Hash;

pub trait Atom: IntoNoun + Sized {
    type Error;

    fn as_bytes(&self) -> &[u8];
}

pub trait Cell: IntoNoun + Sized {
    type Noun: Noun;

    fn into_parts(self) -> (Option<<Self as Cell>::Noun>, Option<<Self as Cell>::Noun>);
}

pub trait Noun: Cue + Eq + Hash + Jam + Sized {
    type Atom: Atom;
    type Cell: Cell;
    type Error;

    fn get(&self, idx: usize) -> Option<&Self>;

    fn into_atom(self) -> Result<Self::Atom, <Self as Noun>::Error>;

    fn into_cell(self) -> Result<Self::Cell, <Self as Noun>::Error>;
}

pub trait Cue: Sized {
    type Error;

    fn cue(jammed_val: Vec<u8>) -> Result<Self, <Self as Cue>::Error>;
}

pub trait Jam: Sized {
    type Error;

    fn jam(self) -> Result<Vec<u8>, <Self as Jam>::Error>;
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
