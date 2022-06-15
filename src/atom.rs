use crate::{cell::Cell, IntoNoun, Noun};

pub trait Atom<C, N>
where
    C: Cell<Self, N>,
    N: Noun<Self, C>,
    Self: IntoNoun<Self, C, N> + Sized,
{
    type Error;

    fn as_vec(&self) -> Vec<u8>;

    fn as_u64(&self) -> Result<u64, <Self as Atom<C, N>>::Error>;
}
