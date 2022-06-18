use crate::{Atom, Cell, Noun};
use bitstream_io::BitRead;
use std::{
    collections::HashMap,
    fmt::Debug,
    mem::{drop, size_of},
    rc::Rc,
};

/// (<some type>, bits read)
pub type CueResult<T> = Result<(T, u32), ()>;

pub trait Cue<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Noun<A, C> + Debug + Sized,
{
    fn cue(mut src: impl BitRead) -> Result<Self, ()> {
        let mut cache = HashMap::new();
        let (noun, _) = Self::decode(&mut src, &mut cache, 0)?;

        // Dropping the cache guarantees that the top level noun has exactly one reference, which
        // makes it safe to move out of the Rc.
        drop(cache);
        let noun = Rc::try_unwrap(noun).unwrap();

        Ok(noun)
    }

    /// Recursively decode a bitstream.
    fn decode(
        src: &mut impl BitRead,
        cache: &mut HashMap<u64, Rc<Self>>,
        pos: u64,
    ) -> CueResult<Rc<Self>> {
        match src.read_bit() {
            Ok(true) => {
                const TAG_LEN: u32 = 2;
                match src.read_bit() {
                    // Back reference tag = 0b11.
                    Ok(true) => {
                        let (noun, bits_read) = Self::decode_backref(src, cache, pos)?;
                        Ok((noun, TAG_LEN + bits_read))
                    }
                    // Cell tag = 0b01.
                    Ok(false) => {
                        let (cell, bits_read) = Self::decode_cell(src, cache, pos)?;
                        Ok((cell, TAG_LEN + bits_read))
                    }
                    Err(_) => todo!("IO error"),
                }
            }
            // Atom tag = 0b0.
            Ok(false) => {
                const TAG_LEN: u32 = 1;
                let (atom, bits_read) = Self::decode_atom(src, Some(cache), pos)?;
                Ok((atom, TAG_LEN + bits_read))
            }
            Err(_) => {
                todo!("I think this is when it's time to exit")
            }
        }
    }

    /// Decode the length in bits of an atom, returning (len, bits read).
    fn decode_len(src: &mut impl BitRead) -> CueResult<u64> {
        let len_of_len = src.read_unary0().expect("count high bits");
        // Length must be 63 bits or less.
        if len_of_len >= u64::BITS {
            todo!("too large")
        }

        let len: u64 = src.read(len_of_len).expect("get length");
        // Most significant bit of the length is always one and always omitted, so add it back now.
        let len = (1 << len_of_len) | len;

        let bits_read = 2 * len_of_len + 1;
        Ok((len, bits_read))
    }

    /// Decode an atom, returning (atom, bits read).
    ///
    /// src: bitstream.
    /// cache: mapping from bitstream index to encoded noun starting at that index.
    /// pos: bitstream index that the encoded atom starts at.
    fn decode_atom(
        src: &mut impl BitRead,
        cache: Option<&mut HashMap<u64, Rc<Self>>>,
        pos: u64,
    ) -> CueResult<Rc<Self>> {
        // Decode the atom length.
        let (mut bit_len, mut bits_read) = Self::decode_len(src)?;

        let mut val = {
            // This will allocate an extra byte when bit_len is a multiple of u8::BITS, but it's
            // worth it to omit a branch.
            let byte_len = (bit_len / u64::from(u8::BITS)) + 1;
            let byte_len = usize::try_from(byte_len).expect("u64 doesn't fit in usize");
            Vec::with_capacity(byte_len)
        };
        while bit_len > u64::from(u8::BITS) {
            let byte: u8 = src.read(u8::BITS).expect("read chunk");
            bits_read += u8::BITS;
            val.push(byte);
            bit_len -= u64::from(u8::BITS);
        }
        // Consume remaining bits.
        let bit_len = u32::try_from(bit_len).unwrap();
        let byte: u8 = src.read(bit_len).expect("read chunk");
        bits_read += bit_len;
        val.push(byte);

        let atom = Rc::new(A::from(val).into_noun().unwrap());
        if let Some(cache) = cache {
            cache.insert(pos, atom.clone());
        }

        Ok((atom, bits_read))
    }

    /// Decode a backreference, returning (noun, bits read).
    ///
    /// src: bitstream.
    /// cache: mapping from bitstream index to encoded noun starting at that index.
    /// pos: bitstream index that the encoded backreference starts at.
    fn decode_backref(
        src: &mut impl BitRead,
        cache: &mut HashMap<u64, Rc<Self>>,
        pos: u64,
    ) -> CueResult<Rc<Self>> {
        let (idx, bits_read) = Self::decode_atom(src, None, pos)?;
        let (first, rest) = idx.as_atom()?.as_bytes().split_at(size_of::<u64>());
        if rest.len() > 0 {
            todo!("idx is larger than 8 bytes")
        }
        // XXX: watch out for endianness bug.
        let idx = u64::from_le_bytes(first.try_into().unwrap());
        if let Some(noun) = cache.get(&idx) {
            Ok((noun.clone(), bits_read))
        } else {
            Err(())
        }
    }

    /// Decode a cell, returning (cell, bits read). A default implementation because a cell is a
    /// self referential data type, and its construction cannot be generalized using the Cell
    /// trait.
    ///
    /// src: bitstream.
    /// cache: mapping from bitstream index to encoded noun starting at that index.
    /// head_start: bitstream index that the head of the encoded cell starts at.
    fn decode_cell(
        src: &mut impl BitRead,
        cache: &mut HashMap<u64, Rc<Self>>,
        pos: u64,
    ) -> CueResult<Rc<Self>>;
}
