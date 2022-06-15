use crate::{cell::Cell, Noun};

pub trait Atom<C, N>
where
    C: Cell<Self, N>,
    N: Noun<Self, C>,
    Self: Sized,
{
    type Error;

    fn as_vec(&self) -> Vec<u8>;

    fn as_u64(&self) -> Result<u64, Self::Error>;

    fn into_noun(self) -> N;
}
