pub mod atom;
pub mod cell;
pub mod noun;

#[cfg(test)]
mod tests {
    use super::{atom::*, cell::*, noun::*};
    use crate::{cue::Cue as _, Atom as _, Cell as _, Noun as _};
    use bitstream_io::{BigEndian, BitRead, BitReader, LittleEndian};
    use std::rc::Rc;

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

            let (atom, bits_read) = Noun::decode_atom(&mut bitstream, None, 0)?;
            let atom = Rc::try_unwrap(atom).expect("more than 1 reference");
            assert_eq!(bits_read, 15);
            match atom {
                Noun::Atom(atom) => {
                    let bytes = atom.as_bytes();
                    assert_eq!(bytes[0], 0x8);
                }
                _ => return Err(()),
            }
        }

        {
            let vec: Vec<u8> = vec![0x17, 0x84];
            let mut bitstream: BitReader<&[_], LittleEndian> = BitReader::new(&vec[..]);

            let (atom, bits_read) = Noun::decode_atom(&mut bitstream, None, 0)?;
            let atom = Rc::try_unwrap(atom).expect("more than 1 reference");
            assert_eq!(bits_read, 16);
            match atom {
                Noun::Atom(atom) => {
                    let bytes = atom.as_bytes();
                    assert_eq!(bytes[0], 0x8);
                    assert_eq!(bytes[1], 0x1);
                }
                _ => return Err(()),
            }
        }

        Ok(())
    }

    #[test]
    fn noun_get() {
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
                Noun::Cell(Cell::new($head, $tail))
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
    }
}
