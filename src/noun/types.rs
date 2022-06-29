//! Assorted [`Noun`] implementations.

use crate::{
    atom::{types::VecAtom, Atom},
    cell::{types::RcCell, Cell},
    noun::Noun,
    serdes::{Cue, Jam},
};
use std::rc::Rc;

#[derive(Eq, Clone, Debug, Hash)]
pub enum EnumNoun<A, C>
where
    A: Atom,
    C: Cell,
    Self: Noun<A, C>,
{
    Atom(A),
    Cell(C),
}

impl Cue<VecAtom, RcCell> for EnumNoun<VecAtom, RcCell> {
    fn new_cell(head: Rc<Self>, tail: Rc<Self>) -> RcCell {
        RcCell::new(head, tail)
    }
}

impl Jam<'_, VecAtom, RcCell> for EnumNoun<VecAtom, RcCell> {
    fn cell_as_parts(cell: &RcCell) -> (&Self, &Self) {
        (cell.head(), cell.tail())
    }
}

impl Noun<VecAtom, RcCell> for EnumNoun<VecAtom, RcCell> {
    fn get(&self, axis: usize) -> Option<&Self> {
        if let Self::Cell(cell) = self {
            match axis {
                0 | 1 => Some(self),
                2 => Some(&*cell.head()),
                3 => Some(&*cell.tail()),
                n if n % 2 == 0 => (&*cell.head()).get(axis / 2),
                _ => (&*cell.tail()).get(axis / 2),
            }
        } else {
            None
        }
    }

    fn as_atom(&self) -> Result<&VecAtom, ()> {
        match self {
            Self::Atom(atom) => Ok(atom),
            _ => Err(()),
        }
    }

    fn as_cell(&self) -> Result<&RcCell, ()> {
        match self {
            Self::Cell(cell) => Ok(cell),
            _ => Err(()),
        }
    }

    fn into_atom(self) -> Result<VecAtom, Self> {
        match self {
            Self::Atom(atom) => Ok(atom),
            _ => Err(self),
        }
    }

    fn into_cell(self) -> Result<RcCell, Self> {
        match self {
            Self::Cell(cell) => Ok(cell),
            _ => Err(self),
        }
    }
}

impl PartialEq for EnumNoun<VecAtom, RcCell> {
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

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{atom::Atom, convert::IntoNoun};
    use std::rc::Rc;

    #[test]
    fn noun_get() {
        fn run_test<A, C, N>()
        where
            A: Atom + IntoNoun<A, C, N>,
            C: Cell<A, N> + IntoNoun<A, C, N>,
            N: Noun<A, C>,
        {
            {
                let _4 = Rc::new(A::from_u8(4).into_noun_unchecked());
                let _5 = Rc::new(A::from_u8(5).into_noun_unchecked());
                let _6 = Rc::new(A::from_u8(6).into_noun_unchecked());
                let _14 = Rc::new(A::from_u8(14).into_noun_unchecked());
                let _15 = Rc::new(A::from_u8(15).into_noun_unchecked());

                let tt = Rc::new(C::new(_14, _15).into_noun_unchecked());
                let t = Rc::new(C::new(_6, tt.clone()).into_noun_unchecked());
                let h = Rc::new(C::new(_4, _5).into_noun_unchecked());
                let noun = C::new(h.clone(), t.clone()).into_noun_unchecked();

                assert_eq!(noun.get(1), Some(&noun));
                assert_eq!(noun.get(2), Some(&*h));
                assert_eq!(noun.get(3), Some(&*t));
                assert_eq!(noun.get(7), Some(&*tt));
                assert_eq!(noun.get(12), None);
            }
        }

        run_test::<VecAtom, RcCell, EnumNoun<VecAtom, RcCell>>();
    }
}
*/
