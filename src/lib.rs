pub mod r#enum;

use bitstream_io::{BitRead, BitWrite};
use std::hash::Hash;

pub trait Atom: IntoNoun + Sized {
    type Error;
    type Val;

    fn new(val: Self::Val) -> Self;

    fn as_bytes(&self) -> &[u8];
}

pub trait Cell: IntoNoun + Sized {
    type Head;
    type Tail;

    fn new(head: Option<Self::Head>, tail: Option<Self::Tail>) -> Self;

    fn into_parts(self) -> (Option<Self::Head>, Option<Self::Tail>);
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
    type Atom;
    type Cell;
    type Error;

    fn cue(src: impl BitRead) -> Result<Self, <Self as Cue>::Error>;

    /// Read the length of an atom or backreference, returning (length, bits read).
    fn cue_len(src: &mut impl BitRead) -> Result<(u64, u32), ()> {
        let len_of_len = src.read_unary0().expect("count high bits");
        // Length must be 63 bits or less.
        if len_of_len >= u64::BITS {
            todo!("too large")
        }

        let len: u64 = src.read(len_of_len).expect("get length");
        // Most significant bit of the length is always one and always omitted, so add it back now.
        let len = (1 << len_of_len) | len;

        let bits_read = 2 * len_of_len + 1;

        Ok((len, bits_read))
    }
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
