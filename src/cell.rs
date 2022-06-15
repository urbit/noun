use crate::{atom::Atom, Noun};

pub trait Cell<A, N>
where
    A: Atom<Self, N>,
    N: Noun<A, Self>,
    Self: Sized,
{
    fn get(&self, idx: usize) -> Option<N>;

    fn into_parts(self) -> (Option<N>, Option<N>);

    fn into_noun(self) -> N;
}
