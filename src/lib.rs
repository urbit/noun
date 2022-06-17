pub mod cue;

pub mod r#enum;

use bitstream_io::BitWrite;
use std::hash::Hash;

pub trait Atom<C, N>
where
    C: Cell<Self, N>,
    N: Noun<Self, C>,
    Self: IntoNoun<Self, C, N> + Sized,
{
    fn new(val: Vec<u8>) -> Self;

    fn as_bytes(&self) -> &[u8];
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

    fn into_parts(self) -> (Self::Head, Self::Tail);
}

pub trait Noun<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Hash + Sized,
{
    fn get(&self, idx: usize) -> Option<&Self>;

    fn into_atom(self) -> Result<A, ()>;

    fn into_cell(self) -> Result<C, ()>;
}

/// Unifying equality.
pub trait UnifyEq<C>
where
    Self: Eq,
{
    fn eq(&self, other: &Self, _ctx: C) -> bool;
}

pub trait Jam<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Noun<A, C> + Sized,
{
    fn jam(self, sink: &mut impl BitWrite) -> Result<(), ()>;
}

/// Convert a noun into the implementing type.
pub trait FromNoun<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
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
    fn into_noun(self) -> Result<N, ()>;
}
