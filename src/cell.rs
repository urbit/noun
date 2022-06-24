use crate::{atom::Atom, Noun};

pub trait Cell<A, N>
where
    A: Atom<Self, N>,
    N: Noun<A, Self>,
    Self: Sized,
{
    type Head;
    type Tail;

    fn new(head: Self::Head, tail: Self::Tail) -> Self;

    fn head(&self) -> &Self::Head;

    fn tail(&self) -> &Self::Tail;

    fn head_as_noun(&self) -> &N;

    fn tail_as_noun(&self) -> &N;

    fn as_parts(&self) -> (&Self::Head, &Self::Tail) {
        (self.head(), self.tail())
    }

    fn into_parts(self) -> (Self::Head, Self::Tail);

    fn into_noun(self) -> N;
}
