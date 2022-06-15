use crate::{atom::Atom, cell::Cell, FromNoun, IntoNoun, Noun};

/// Jamming a type involves two steps:
/// (1) Convert the implementing type into a noun.
/// (2) Serialize the noun from (1).
///
/// If the implementing type also implements Noun, then (1) is trivial.
pub trait Jam<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: IntoNoun<A, C, N> + Sized,
{
    type Error;

    fn jam(self) -> Result<Vec<u8>, <Self as Jam<A, C, N>>::Error>;
}

/// Cueing a type involves two steps:
/// (1) Deserialize the jammed value into a noun.
/// (2) Convert the noun from (1) into the implementing type.
///
/// If the implementing type also implements Noun, then (2) is trivial.
pub trait Cue<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: FromNoun<A, C, N> + Sized,
{
    type Error;

    fn cue(jammed_val: Vec<u8>) -> Result<Self, <Self as Cue<A, C, N>>::Error>;
}
