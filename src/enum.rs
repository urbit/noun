use crate::{
    Atom as _Atom, Cell as _Cell, Cue as _Cue, IntoNoun as _IntoNoun, Jam as _Jam, Noun as _Noun,
};
use std::hash::{Hash, Hasher};

#[derive(Eq, Hash)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl _Cue for Noun {
    type Error = ();

    fn cue(_jammed_val: Vec<u8>) -> Result<Self, <Self as _Cue>::Error> {
        todo!()
    }
}

impl _Jam for Noun {
    type Error = ();

    fn jam(self) -> Result<Vec<u8>, <Self as _Jam>::Error> {
        todo!()
    }
}

impl _Noun for Noun {
    type Atom = Atom;
    type Cell = Cell;
    type Error = ();

    fn get(&self, idx: usize) -> Option<&Noun> {
        if let Self::Cell(cell) = self {
            match idx {
                0 | 1 => Some(self),
                2 => Some(&*cell.head.as_ref()?),
                3 => Some(&*cell.tail.as_ref()?),
                n if n % 2 == 0 => (&*cell.head.as_ref()?).get(idx / 2),
                _ => (&*cell.tail.as_ref()?).get(idx / 2),
            }
        } else {
            None
        }
    }

    fn into_atom(self) -> Result<<Self as _Noun>::Atom, <Self as _Noun>::Error> {
        match self {
            Self::Atom(atom) => Ok(atom),
            _ => Err(()),
        }
    }

    fn into_cell(self) -> Result<<Self as _Noun>::Cell, <Self as _Noun>::Error> {
        match self {
            Self::Cell(cell) => Ok(cell),
            _ => Err(()),
        }
    }
}

impl PartialEq for Noun {
    fn eq(&self, other: &Self) -> bool {
        if let (Self::Atom(this), Self::Atom(that)) = (self, other) {
            this == that
        } else if let (Self::Cell(this), Self::Cell(that)) = (self, other) {
            this == that
        } else {
            false
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct Atom(Vec<u8>);

impl _Atom for Atom {
    type Error = ();

    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl _IntoNoun for Atom {
    type Error = ();
    type Noun = Noun;

    fn into_noun(self) -> Result<Self::Noun, Self::Error> {
        Ok(Noun::Atom(self))
    }
}

#[derive(Eq)]
pub struct Cell {
    head: Option<Box<Noun>>,
    tail: Option<Box<Noun>>,
}

impl _Cell for Cell {
    type Noun = Noun;

    fn into_parts(self) -> (Option<<Self as _Cell>::Noun>, Option<<Self as _Cell>::Noun>) {
        (self.head.map(|n| *n), self.tail.map(|n| *n))
    }
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

impl _IntoNoun for Cell {
    type Error = ();
    type Noun = Noun;

    fn into_noun(self) -> Result<Self::Noun, Self::Error> {
        Ok(Noun::Cell(self))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}
