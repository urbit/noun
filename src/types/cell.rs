use crate::{
    types::{atom::Atom, noun::Noun},
    Cell as _Cell, IntoNoun,
};
use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

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

impl IntoNoun<Atom, Self, Noun> for Cell {
    fn as_noun(&self) -> Result<Noun, ()> {
        Err(())
    }

    fn into_noun(self) -> Result<Noun, ()> {
        Ok(Noun::Cell(self))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}
