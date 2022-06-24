use crate::{atom::types::Atom, cell::Cell as _Cell, noun::types::Noun};
use std::{hash::Hash, rc::Rc};

#[derive(Clone, Hash, Debug, Eq)]
pub struct Cell {
    head: Rc<Noun>,
    tail: Rc<Noun>,
}

impl _Cell<Atom, Noun> for Cell {
    type Head = Rc<Noun>;
    type Tail = Self::Head;

    fn new(head: Self::Head, tail: Self::Tail) -> Self {
        Self { head, tail }
    }

    fn head(&self) -> &Self::Head {
        &self.head
    }

    fn tail(&self) -> &Self::Tail {
        &self.tail
    }

    fn head_as_noun(&self) -> &Noun {
        &*self.head
    }

    fn tail_as_noun(&self) -> &Noun {
        &*self.tail
    }

    fn into_parts(self) -> (Self::Head, Self::Tail) {
        (self.head, self.tail)
    }

    fn into_noun(self) -> Noun {
        Noun::Cell(self)
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}
