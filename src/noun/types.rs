//! Assorted [`Noun`] implementations.

use crate::{
    atom::types::Atom,
    cell::{types::Cell, Cell as _},
    noun::Noun as _Noun,
    serdes::{Cue, Jam},
};
use std::rc::Rc;

#[derive(Eq, Clone, Debug, Hash)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl Cue<Atom, Cell> for Noun {
    fn new_cell(head: Rc<Self>, tail: Rc<Self>) -> Cell {
        Cell::from_parts(head, tail)
    }
}

impl Jam<'_, Atom, Cell> for Noun {}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noun_get() {
        /*
        /// Create a new Noun::Atom from a list of numbers.
        macro_rules! na {
            ($elem:expr , $n:expr) => {
                let vec = vec![$elem; $n];
                Noun::Atom(Atom::from(vec))
            };
            ($($x:expr),+ $(,)?) => {
                {
                    let mut vec = Vec::new();
                    $(
                        vec.push($x);

                     )*
                        Noun::Atom(Atom::from(vec))
                }
            };
        }

        /// Create a new cell from a pair of Option<Box<<>>.
        macro_rules! nc {
            ($head:expr, $tail:expr) => {
                Noun::Cell(Cell::from_parts($head, $tail))
            };
        }

        // [[4 5] [6 14 15]]
        let tt = nc!(Rc::new(na![14]), Rc::new(na![15]));
        let t = nc!(Rc::new(na![6]), Rc::new(tt.clone()));
        let h = nc!(Rc::new(na![4]), Rc::new(na![5]));
        let n = nc!(Rc::new(h.clone()), Rc::new(t.clone()));

        assert_eq!(n.get(1), Some(&n));
        assert_eq!(n.get(2), Some(&h));
        assert_eq!(n.get(3), Some(&t));
        assert_eq!(n.get(7), Some(&tt));
        assert_eq!(n.get(12), None);
        */
    }
}
