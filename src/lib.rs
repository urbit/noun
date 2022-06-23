pub mod serdes;
pub mod types;

use bitstream_io::{BitReader, LittleEndian};
use std::{default::Default, hash::Hash, ops::Add, str};

macro_rules! to_uint {
    ($atom:expr, $uint:ty) => {{
        let atom = $atom.as_bytes();
        const N: usize = std::mem::size_of::<$uint>();
        let len = atom.len();
        if len <= N {
            let mut bytes: [u8; N] = [0; N];
            let _ = &mut bytes[..len].copy_from_slice(atom);
            Ok(<$uint>::from_le_bytes(bytes))
        } else {
            Err(())
        }
    }};
}

pub trait Atom<C, N>
where
    C: Cell<Self, N>,
    N: Noun<Self, C>,
    Self: Add<Self>
        + Add<u8>
        + Add<u16>
        + Add<u32>
        + Add<u64>
        + Add<u128>
        + Add<usize>
        + Default
        + Eq
        + From<u8>
        + From<u16>
        + From<u32>
        + From<u64>
        + From<u128>
        + From<usize>
        + From<Vec<u8>>
        + IntoNoun<Self, C, N>
        + Sized,
{
    fn as_bytes(&self) -> &[u8];

    fn as_bits(&self) -> BitReader<&[u8], LittleEndian> {
        BitReader::new(self.as_bytes())
    }

    fn as_u8(&self) -> Result<u8, ()> {
        to_uint!(self, u8)
    }

    fn as_u16(&self) -> Result<u16, ()> {
        to_uint!(self, u16)
    }

    fn as_u32(&self) -> Result<u32, ()> {
        to_uint!(self, u32)
    }

    fn as_u64(&self) -> Result<u64, ()> {
        to_uint!(self, u64)
    }

    fn as_u128(&self) -> Result<u128, ()> {
        to_uint!(self, u128)
    }

    fn as_usize(&self) -> Result<usize, ()> {
        to_uint!(self, usize)
    }

    fn as_str(&self) -> Result<&str, ()> {
        Ok(str::from_utf8(self.as_bytes()).map_err(|_| ())?)
    }
}

pub trait Cell<A, N>
where
    A: Atom<Self, N>,
    N: Noun<A, Self>,
    Self: IntoNoun<A, Self, N> + Sized,
{
    type Head;
    type Tail;

    fn new(head: Self::Head, tail: Self::Tail) -> Self;

    fn head(&self) -> &Self::Head;

    fn tail(&self) -> &Self::Tail;

    fn as_parts(&self) -> (&Self::Head, &Self::Tail) {
        (self.head(), self.tail())
    }

    fn into_parts(self) -> (Self::Head, Self::Tail);
}

pub trait Noun<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Hash + Sized,
{
    fn get(&self, idx: usize) -> Option<&Self>;

    fn as_atom(&self) -> Result<&A, ()>;

    fn as_cell(&self) -> Result<&C, ()>;

    fn into_atom(self) -> Result<A, Self>;

    fn into_cell(self) -> Result<C, Self>;
}

/// Unifying equality.
pub trait UnifyEq<C>
where
    Self: Eq,
{
    fn eq(&self, other: &Self, _ctx: C) -> bool;
}

/// Convert a noun into the implementing type.
pub trait FromNoun<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    fn from_noun_ref(noun: &N) -> Result<Self, ()>;

    fn from_noun(noun: N) -> Result<Self, ()>;
}

/// Convert the implementing type into a noun.
pub trait IntoNoun<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    fn as_noun(&self) -> Result<N, ()>;

    fn into_noun(self) -> Result<N, ()>;
}
