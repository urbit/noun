use crate::{
    serdes::{Cue, Jam},
    types::{atom::Atom, cell::Cell},
    Cell as _Cell, Noun as _Noun,
};
use bitstream_io::BitWrite;
use std::rc::Rc;

#[derive(Eq, Clone, Debug, Hash)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl Cue<Atom, Cell> for Noun {
    fn new_cell(head: Rc<Self>, tail: Rc<Self>) -> Cell {
        Cell::new(head, tail)
    }
}

impl Jam<Atom, Cell> for Noun {
    fn jam(self, _sink: &mut impl BitWrite) -> Result<(), ()> {
        todo!()
    }
}

impl _Noun<Atom, Cell> for Noun {
    fn get(&self, idx: usize) -> Option<&Self> {
        if let Self::Cell(cell) = self {
            match idx {
                0 | 1 => Some(self),
                2 => Some(&*cell.head()),
                3 => Some(&*cell.tail()),
                n if n % 2 == 0 => (&*cell.head()).get(idx / 2),
                _ => (&*cell.tail()).get(idx / 2),
            }
        } else {
            None
        }
    }

    fn as_atom(&self) -> Result<&Atom, ()> {
        match self {
            Self::Atom(atom) => Ok(atom),
            _ => Err(()),
        }
    }

    fn as_cell(&self) -> Result<&Cell, ()> {
        match self {
            Self::Cell(cell) => Ok(cell),
            _ => Err(()),
        }
    }

    fn into_atom(self) -> Result<Atom, Self> {
        match self {
            Self::Atom(atom) => Ok(atom),
            _ => Err(self),
        }
    }

    fn into_cell(self) -> Result<Cell, Self> {
        match self {
            Self::Cell(cell) => Ok(cell),
            _ => Err(self),
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
