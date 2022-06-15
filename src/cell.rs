use crate::{atom::Atom, IntoNoun, Noun};

pub trait Cell<A, N>
where
    A: Atom<Self, N>,
    N: Noun<A, Self>,
    Self: IntoNoun<A, Self, N> + Sized,
{
    fn get(&self, idx: usize) -> Option<N>;

    fn into_parts(self) -> (Option<N>, Option<N>);
}
