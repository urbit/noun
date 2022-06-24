//! Assorted [`Noun`] implementations.

use crate::{
    atom::types::Atom,
    cell::{types::Cell, Cell as _Cell},
    noun::Noun as _Noun,
    serdes::{Cue, Jam},
};

#[derive(Eq, Clone, Debug, Hash)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl Cue<Atom, Cell> for Noun {}

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
    use crate::create_test;
    use std::rc::Rc;

    #[test]
    fn noun_get() {
        create_test!(
            (),
            (),
            // [[4 5] [6 14 15]]
            {
                let _4 = Rc::new(A::from_u8(4).into_noun());
                let _5 = Rc::new(A::from_u8(5).into_noun());
                let _6 = Rc::new(A::from_u8(6).into_noun());
                let _14 = Rc::new(A::from_u8(14).into_noun());
                let _15 = Rc::new(A::from_u8(15).into_noun());

                let tt = Rc::new(C::from_pair(_14, _15).into_noun());
                let t = Rc::new(C::from_pair(_6, tt.clone()).into_noun());
                let h = Rc::new(C::from_pair(_4, _5).into_noun());
                let noun = C::from_pair(h.clone(), t.clone()).into_noun();

                assert_eq!(noun.get(1), Some(&noun));
                assert_eq!(noun.get(2), Some(&*h));
                assert_eq!(noun.get(3), Some(&*t));
                assert_eq!(noun.get(7), Some(&*tt));
                assert_eq!(noun.get(12), None);
            }
        );

        run_test::<Atom, Cell, Noun>();
    }
}
