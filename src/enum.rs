use crate::{
    Atom as _Atom, Cell as _Cell, Cue as _Cue, CueResult, IntoNoun as _IntoNoun, Jam as _Jam,
    Noun as _Noun,
};
use bitstream_io::{BitRead, BitWrite};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    rc::Rc,
};

#[derive(Eq, Clone, Debug, Hash)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl _Cue<Atom, Cell> for Noun {
    fn decode_cell(
        src: &mut impl BitRead,
        cache: &mut HashMap<usize, Rc<Self>>,
        head_start: usize,
    ) -> CueResult<Rc<Self>> {
        let (head, bits_read) = Self::decode(src, cache, head_start)?;
        let head = Rc::new(head);
        cache.insert(head_start, head.clone());

        let tail_start = head_start + usize::try_from(bits_read).expect("usize smaller than u32");

        let (tail, bits_read) = Self::decode(src, cache, tail_start)?;
        let tail = Rc::new(tail);
        cache.insert(tail_start, tail.clone());

        let cell = Rc::new(Self::Cell(Cell::new(Some(head), Some(tail))));

        let bits_read =
            u32::try_from(tail_start - head_start).expect("usize smaller than u32") + bits_read;

        Ok((cell, bits_read))
    }
}

impl _Jam<Atom, Cell> for Noun {
    fn jam(self, _sink: &mut impl BitWrite) -> Result<(), ()> {
        todo!()
    }
}

impl _Noun<Atom, Cell> for Noun {
    fn get(&self, idx: usize) -> Option<&Self> {
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

    fn into_atom(self) -> Result<Atom, ()> {
        match self {
            Self::Atom(atom) => Ok(atom),
            _ => Err(()),
        }
    }

    fn into_cell(self) -> Result<Cell, ()> {
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Atom(Vec<u8>);

impl _Atom<Cell, Noun> for Atom {
    fn new(val: Vec<u8>) -> Self {
        Self(val)
    }

    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl _IntoNoun<Self, Cell, Noun> for Atom {
    fn into_noun(self) -> Result<Noun, ()> {
        Ok(Noun::Atom(self))
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Cell {
    head: Option<Rc<Noun>>,
    tail: Option<Rc<Noun>>,
}

impl _Cell<Atom, Noun> for Cell {
    type Head = Rc<Noun>;
    type Tail = Self::Head;

    fn new(head: Option<Self::Head>, tail: Option<Self::Tail>) -> Self {
        Self { head, tail }
    }

    fn into_parts(self) -> (Option<Self::Head>, Option<Self::Tail>) {
        (self.head, self.tail)
    }
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

impl _IntoNoun<Atom, Self, Noun> for Cell {
    fn into_noun(self) -> Result<Noun, ()> {
        Ok(Noun::Cell(self))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitstream_io::{BigEndian, BitRead, BitReader, LittleEndian};

    #[test]
    fn bitstream() -> Result<(), std::io::Error> {
        // Read a byte at a time.
        {
            // LSB first.
            {
                let vec: Vec<u8> = vec![0x0, 0xa, 0xb, 0xc];
                let mut bitstream: BitReader<&[_], LittleEndian> = BitReader::new(&vec[..]);

                let val: u8 = bitstream.read(u8::BITS)?;
                assert_eq!(val, vec[0]);

                let val: u8 = bitstream.read(u8::BITS)?;
                assert_eq!(val, vec[1]);

                let val: u8 = bitstream.read(u8::BITS)?;
                assert_eq!(val, vec[2]);

                let val: u8 = bitstream.read(u8::BITS)?;
                assert_eq!(val, vec[3]);
            }

            // MSB first.
            {
                let vec: Vec<u8> = vec![0x0, 0xa, 0xb, 0xc];
                let mut bitstream: BitReader<&[_], BigEndian> = BitReader::new(&vec[..]);

                let val: u8 = bitstream.read(u8::BITS)?;
                assert_eq!(val, vec[0]);

                let val: u8 = bitstream.read(u8::BITS)?;
                assert_eq!(val, vec[1]);

                let val: u8 = bitstream.read(u8::BITS)?;
                assert_eq!(val, vec[2]);

                let val: u8 = bitstream.read(u8::BITS)?;
                assert_eq!(val, vec[3]);
            }
        }

        // Read a word at a time.
        {
            // LSB first.
            {
                let vec: Vec<u8> = vec![0x0, 0xa, 0xb, 0xc];
                let mut bitstream: BitReader<&[_], LittleEndian> = BitReader::new(&vec[..]);

                let val: u32 = bitstream.read(u32::BITS)?;
                assert_eq!(val, 0xc0b0a00);
            }

            // MSB first.
            {
                let vec: Vec<u8> = vec![0x0, 0xa, 0xb, 0xc];
                let mut bitstream: BitReader<&[_], BigEndian> = BitReader::new(&vec[..]);

                let val: u32 = bitstream.read(u32::BITS)?;
                assert_eq!(val, 0xa0b0c);
            }
        }

        // Count bits.
        {
            // LSB first.
            {
                let vec: Vec<u8> = vec![0x0, 0xa, 0xb, 0xf];
                let mut bitstream: BitReader<&[_], LittleEndian> = BitReader::new(&vec[..]);

                let len: u32 = bitstream.read_unary1()?;
                assert_eq!(len, 9);
            }

            // MSB first.
            {
                let vec: Vec<u8> = vec![0xf0, 0xa, 0xb, 0x0];
                let mut bitstream: BitReader<&[_], BigEndian> = BitReader::new(&vec[..]);

                let len: u32 = bitstream.read_unary0()?;
                assert_eq!(len, 4);
            }
        }

        Ok(())
    }

    #[test]
    fn noun_cue() {}

    #[test]
    fn noun_cue_atom() -> Result<(), ()> {
        {
            let vec: Vec<u8> = vec![0x7, 0x4];
            let mut bitstream: BitReader<&[_], LittleEndian> = BitReader::new(&vec[..]);
            let mut curr_idx = 0;

            let (atom, bits_read) = Noun::decode_atom(&mut bitstream)?;
            assert_eq!(bits_read, 15);
            match atom {
                Noun::Atom(Atom(val)) => {
                    assert_eq!(val[0], 0x8);
                }
                _ => return Err(()),
            }
        }

        {
            let vec: Vec<u8> = vec![0x17, 0x84];
            let mut bitstream: BitReader<&[_], LittleEndian> = BitReader::new(&vec[..]);

            let (atom, bits_read) = Noun::decode_atom(&mut bitstream)?;
            assert_eq!(bits_read, 16);
            match atom {
                Noun::Atom(Atom(val)) => {
                    assert_eq!(val[0], 0x8);
                    assert_eq!(val[1], 0x1);
                }
                _ => return Err(()),
            }
        }

        Ok(())
    }

    #[test]
    fn noun_get() {
        /// Wrap a value in an Option<Box<>>.
        macro_rules! b {
            ($inner:expr) => {
                Some(Box::new($inner))
            };
        }

        /// Create a new Noun::Atom from a list of numbers.
        macro_rules! na {
            ($elem:expr , $n:expr) => {
                let vec = vec![$elem; $n];
                Noun::Atom(Atom(vec))
            };
            ($($x:expr),+ $(,)?) => {
                {
                    let mut vec = Vec::new();
                    $(
                        vec.push($x);

                     )*
                        Noun::Atom(Atom(vec))
                }
            };
        }

        /// Create a new cell from a pair of Option<Box<<>>.
        macro_rules! nc {
            ($head:expr, $tail:expr) => {
                Noun::Cell(Cell {
                    head: $head,
                    tail: $tail,
                })
            };
        }

        // [[4 5] [6 14 15]]
        let tt = nc!(b!(na![14]), b!(na![15]));
        let t = nc!(b!(na![6]), b!(tt.clone()));
        let h = nc!(b!(na![4]), b!(na![5]));
        let n = nc!(b!(h.clone()), b!(t.clone()));

        assert_eq!(n.get(1), Some(&n));
        assert_eq!(n.get(2), Some(&h));
        assert_eq!(n.get(3), Some(&t));
        assert_eq!(n.get(7), Some(&tt));
        assert_eq!(n.get(12), None);
    }
}
