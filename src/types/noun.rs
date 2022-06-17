use crate::{
    cue::{Cue, CueResult},
    jam::Jam,
    types::{atom::Atom, cell::Cell},
    Cell as _Cell, Noun as _Noun,
};
use bitstream_io::{BitRead, BitWrite};
use std::{collections::HashMap, rc::Rc};

#[derive(Eq, Clone, Debug, Hash)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl Cue<Atom, Cell> for Noun {
    fn decode_cell(
        src: &mut impl BitRead,
        cache: &mut HashMap<usize, Rc<Self>>,
        head_start: usize,
    ) -> CueResult<Rc<Self>> {
        let (head, bits_read) = Self::decode(src, cache, head_start)?;
        cache.insert(head_start, head.clone());

        let tail_start = head_start + usize::try_from(bits_read).expect("usize smaller than u32");

        let (tail, bits_read) = Self::decode(src, cache, tail_start)?;
        cache.insert(tail_start, tail.clone());

        let cell = Rc::new(Self::Cell(Cell::new(head, tail)));

        let bits_read =
            u32::try_from(tail_start - head_start).expect("usize smaller than u32") + bits_read;

        Ok((cell, bits_read))
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
