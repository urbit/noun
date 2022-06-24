//! Serialization and deserialization.

use crate::{atom::Atom, cell::Cell, noun::Noun};
use bitstream_io::{BitRead, BitWrite, BitWriter, LittleEndian};
use std::{
    collections::HashMap,
    fmt::Debug,
    io::Error,
    mem::{drop, size_of},
    rc::Rc,
};

/// (<some type>, bits read)
#[doc(hidden)]
pub type CueResult<T> = Result<(T, u32), ()>;

/// Deserialize a bitstream into a noun.
pub trait Cue<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Noun<A, C> + Debug + Sized,
{
    /// Decodes a bitstream into a noun.
    ///
    /// The bitstream is read from least significant bit to most significant bit and starts with a
    /// tag identifying whether the object following the tag is an atom, a cell, or a backreference
    /// to an object that was already decoded. The tag encodings are:
    /// - `0b0`: atom,
    /// - `0b01`: cell, and
    /// - `0b11`: backreference.
    ///
    /// Note that the tag for an atom is only a single bit whereas the tags for a cell and a
    /// backreference are both two bits.
    ///
    /// # Examples
    ///
    fn cue(mut src: impl BitRead) -> Result<Self, ()> {
        let mut cache = HashMap::new();
        let (noun, _) = Self::decode(&mut src, &mut cache, 0)?;

        // Dropping the cache guarantees that the top level noun has exactly one reference, which
        // makes it safe to move out of the Rc.
        drop(cache);
        let noun = Rc::try_unwrap(noun).unwrap();

        Ok(noun)
    }

    #[doc(hidden)]
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
                        let (noun, bits_read) = Self::decode_backref(src, cache)?;
                        Ok((noun, TAG_LEN + bits_read))
                    }
                    // Cell tag = 0b01.
                    Ok(false) => {
                        let (cell, bits_read) =
                            Self::decode_cell(src, cache, pos + u64::from(TAG_LEN))?;
                        cache.insert(pos, cell.clone());
                        Ok((cell, TAG_LEN + bits_read))
                    }
                    Err(_) => todo!("IO error"),
                }
            }
            // Atom tag = 0b0.
            Ok(false) => {
                const TAG_LEN: u32 = 1;
                let (atom, bits_read) = Self::decode_atom(src, Some(cache), Some(pos))?;
                Ok((atom, TAG_LEN + bits_read))
            }
            Err(_) => {
                todo!("I think this is when it's time to exit")
            }
        }
    }

    /// Decode the length of an atom or backreference.
    #[doc(hidden)]
    fn decode_len(src: &mut impl BitRead) -> CueResult<u64> {
        let len_of_len = src.read_unary1().expect("read bit length of length");
        // Length must be 63 bits or less.
        if len_of_len >= u64::BITS {
            todo!("too large")
        }
        let (len, bits_read) = if len_of_len == 0 {
            (0, 1)
        } else {
            // The most significant bit of the length is implicit because it's always 1.
            let len_bits = len_of_len - 1;
            let len: u64 = src.read(len_bits).expect("read length");
            let len = (1 << len_bits) | len;
            let bits_read = len_of_len + 1 + len_bits;
            (len, bits_read)
        };
        Ok((len, bits_read))
    }

    /// Decode an encoded atom from the bitstream. Note that the atom tag must already be consumed.
    #[doc(hidden)]
    fn decode_atom(
        src: &mut impl BitRead,
        cache: Option<&mut HashMap<u64, Rc<Self>>>,
        pos: Option<u64>,
    ) -> CueResult<Rc<Self>> {
        // Decode the atom length.
        let (mut bit_len, mut bits_read) = Self::decode_len(src)?;
        let atom = if bit_len == 0 {
            Rc::new(A::from_u8(0).into_noun())
        } else {
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
            Rc::new(A::from(val).into_noun())
        };

        if let (Some(cache), Some(pos)) = (cache, pos) {
            cache.insert(pos, atom.clone());
        }

        Ok((atom, bits_read))
    }

    /// Decode an encoded backreference from the bitstream. Note that the backreference tag must
    /// already be consumed.
    #[doc(hidden)]
    fn decode_backref(
        src: &mut impl BitRead,
        cache: &mut HashMap<u64, Rc<Self>>,
    ) -> CueResult<Rc<Self>> {
        let (idx, bits_read) = Self::decode_atom(src, None, None)?;
        // Convert index from atom to u64.
        let idx = {
            let bytes = idx.as_atom()?.as_bytes();
            if bytes.len() > size_of::<u64>() {
                todo!("idx is larger than 8 bytes")
            }
            let mut padded_bytes: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
            for i in 0..bytes.len() {
                padded_bytes[i] = bytes[i];
            }
            // XXX: watch out for endianness bug.
            u64::from_le_bytes(padded_bytes)
        };
        if let Some(noun) = cache.get(&idx) {
            Ok((noun.clone(), bits_read))
        } else {
            Err(())
        }
    }

    /// Decode a cell from the bitstream. Note that the cell tag must already be consumed. which
    #[doc(hidden)]
    fn decode_cell(
        src: &mut impl BitRead,
        cache: &mut HashMap<u64, Rc<Self>>,
        mut pos: u64,
    ) -> CueResult<Rc<Self>> {
        let (head, head_bits) = Self::decode(src, cache, pos)?;
        cache.insert(pos, head.clone());

        pos += u64::from(head_bits);

        let (tail, tail_bits) = Self::decode(src, cache, pos)?;
        cache.insert(pos, tail.clone());

        let cell = Rc::new(Self::new_cell(head, tail).into_noun());
        Ok((cell, head_bits + tail_bits))
    }

    /// Construct a new cell.
    ///
    /// The construction of a cell cannot be generalized using the `Cell` trait for use in this
    /// context because the `Cell::Head` and `Cell::Tail` traits are intentionally not bounded by
    /// the `Noun` trait, which would be too onerous on implementers. Beside cell construction,
    /// cueing (decoding) a jammed (encoded) noun is completely independent of the noun
    /// representation, so deserializing a serialized noun is completely independent of the noun
    /// representation, so implementing this single method on a particular noun type will result in
    /// a free implementation of cue.
    fn new_cell(head: Rc<Self>, tail: Rc<Self>) -> C;
}

/// (<some type>, bits written)
#[doc(hidden)]
pub type JamResult<T> = Result<(T, u32), ()>;

/// Serialize a noun into a bitstream.
pub trait Jam<'a, A, C>
where
    A: Atom<C, Self>,
    C: 'a + Cell<A, Self>,
    Self: 'a + Noun<A, C> + Sized,
{
    fn jam(&'a self) -> Result<Vec<u8>, ()> {
        let mut dst = BitWriter::endian(Vec::new(), LittleEndian);
        let mut cache = HashMap::new();
        let _ = Self::encode(self, &mut dst, &mut cache, 0)?;
        // Bits belonging to the last partial byte are discarded when BitWriter::into_writer() is
        // invoked, so we have to byte align.
        if !dst.byte_aligned() {
            if let Err(_) = dst.byte_align() {
                return Err(());
            }
        }
        let mut vec = dst.into_writer();
        // BitWriter::byte_align() pads with an unnecessary extra 0 in certain circumstances.
        if let Some(0) = vec.last() {
            vec.pop();
        }
        Ok(vec)
    }

    #[doc(hidden)]
    fn encode<'b>(
        noun: &'a Self,
        dst: &mut impl BitWrite,
        cache: &'b mut HashMap<&'a Self, u64>,
        pos: u64,
    ) -> JamResult<()> {
        if let Some(idx) = cache.get(noun) {
            Self::encode_backref(idx.clone(), noun, dst)
        } else if let Ok(atom) = noun.as_atom() {
            cache.insert(noun, pos);
            let tag_len = Tag::write(dst, Tag::Atom).expect("write tag");
            let (_, bits_written) = Self::encode_atom(atom, dst)?;
            Ok(((), tag_len + bits_written))
        } else if let Ok(cell) = noun.as_cell() {
            cache.insert(noun, pos);
            let tag_len = Tag::write(dst, Tag::Cell).expect("write tag");
            Self::encode_cell(cell, dst, cache, pos + u64::from(tag_len))
        } else {
            Err(())
        }
    }

    /// Encode the length of an atom or backreference.
    #[doc(hidden)]
    fn encode_len(len: u64, dst: &mut impl BitWrite) -> JamResult<()> {
        let len_of_len = u64::BITS - len.leading_zeros();
        dst.write_unary1(len_of_len)
            .expect("write bit length of length");
        let bits_written = if len_of_len == 0 {
            1
        } else {
            let len_bits = len_of_len - 1;
            // The most significant high bit of the length should not be
            // encoded because it's of course always high.
            let len = !(1 << len_bits) & len;
            dst.write(len_bits, len).expect("write length");
            let bits_written = len_of_len + 1 + len_bits;
            bits_written
        };
        Ok(((), bits_written))
    }

    #[doc(hidden)]
    fn encode_atom(atom: &A, dst: &mut impl BitWrite) -> JamResult<()> {
        let bit_len = atom.bit_len() as u64;
        let (_, mut bits_written) = Self::encode_len(bit_len, dst)?;

        if let Some((last_byte, full_bytes)) = atom.as_bytes().split_last() {
            dst.write_bytes(full_bytes).expect("write full bytes");
            dst.write(u8::BITS - last_byte.leading_zeros(), *last_byte)
                .expect("write last byte");
        }
        bits_written += u32::try_from(bit_len).expect("doesn't fit in u32");

        Ok(((), bits_written))
    }

    #[doc(hidden)]
    fn encode_backref(idx: u64, noun: &Self, dst: &mut impl BitWrite) -> JamResult<()> {
        if let Ok(atom) = noun.as_atom() {
            let idx_bit_len = u64::BITS - idx.leading_zeros();
            let atom_bit_len = u32::try_from(atom.bit_len()).expect("doesn't fit in u32");
            // Backreferences to atoms are only encoded if they're shorter than the atom it would
            // reference.
            if atom_bit_len <= idx_bit_len {
                let tag_len = Tag::write(dst, Tag::Atom).expect("write tag");
                let (_, bits_written) = Self::encode_atom(&atom, dst)?;
                return Ok(((), tag_len + bits_written));
            }
        }

        let tag_len = Tag::write(dst, Tag::BackRef).expect("write tag");
        let idx = A::from_u64(idx);
        let (_, bits_written) = Self::encode_atom(&idx, dst)?;
        Ok(((), tag_len + bits_written))
    }

    #[doc(hidden)]
    fn encode_cell<'b>(
        cell: &'a C,
        dst: &mut impl BitWrite,
        cache: &'b mut HashMap<&'a Self, u64>,
        pos: u64,
    ) -> JamResult<()> {
        let head = cell.head_as_noun();
        let (_, head_bits_written) = Self::encode(head, dst, cache, pos)?;

        let tail = cell.tail_as_noun();
        let pos = pos + u64::from(head_bits_written);
        let (_, tail_bits_written) = Self::encode(tail, dst, cache, pos)?;

        let bits_written = head_bits_written + tail_bits_written;
        Ok(((), bits_written))
    }
}

#[repr(u8)]
enum Tag {
    Atom = 0b0,
    Cell = 0b01,
    BackRef = 0b11,
}

impl Tag {
    fn write(dst: &mut impl BitWrite, tag: Self) -> Result<u32, Error> {
        let tag_len = match tag {
            Tag::Atom => 1,
            Tag::Cell => 2,
            Tag::BackRef => 2,
        };
        dst.write(tag_len, tag as u8)?;
        Ok(tag_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        atom::{types::Atom, Atom as _},
        cell::{types::Cell, Cell as _},
        noun::{types::Noun, Noun as _},
    };

    #[test]
    fn cue() -> Result<(), ()> {
        // 2 deserializes to 0.
        {
            let jammed_noun = Atom::from_u8(0b10);
            let bitstream = jammed_noun.as_bits();
            let noun = Noun::cue(bitstream)?;
            let atom = noun.as_atom()?;
            assert_eq!(atom, &Atom::from_u8(0));
        }

        // 12 deserializes to 1.
        {
            let jammed_noun = Atom::from_u8(0b1100);
            let bitstream = jammed_noun.as_bits();
            let noun = Noun::cue(bitstream)?;
            let atom = noun.as_atom()?;
            assert_eq!(atom, &Atom::from_u8(1));
        }

        // 72 deserializes to 2.
        {
            let jammed_noun = Atom::from_u8(0b1001000);
            let bitstream = jammed_noun.as_bits();
            let noun = Noun::cue(bitstream)?;
            let atom = noun.as_atom()?;
            assert_eq!(atom, &Atom::from_u8(2));
        }

        // 2480 deserializes to 19.
        {
            let jammed_noun = Atom::from_u16(0b100110110000);
            let bitstream = jammed_noun.as_bits();
            let noun = Noun::cue(bitstream)?;
            let atom = noun.as_atom()?;
            assert_eq!(atom, &Atom::from_u8(19));
        }

        // 817 deserializes to [1 1].
        {
            let jammed_noun = Atom::from_u16(0b1100110001);
            let bitstream = jammed_noun.as_bits();
            let noun = Noun::cue(bitstream)?;
            let cell = noun.as_cell()?;
            let (head, tail) = cell.as_parts();

            let head = head.as_atom()?;
            let tail = tail.as_atom()?;

            let _1 = Atom::from_u8(1);
            assert_eq!(head, &_1);
            assert_eq!(tail, &_1);
        }

        // 39.689 deserializes into [0 19].
        {
            let jammed_noun = Atom::from_u16(0b1001101100001001);
            let bitstream = jammed_noun.as_bits();
            let noun = Noun::cue(bitstream)?;
            let cell = noun.as_cell()?;
            let (head, tail) = cell.as_parts();

            let head = head.as_atom()?;
            let tail = tail.as_atom()?;

            let _0 = Atom::from_u8(0);
            let _19 = Atom::from_u8(19);
            assert_eq!(head, &_0);
            assert_eq!(tail, &_19);
        }

        // 4.952.983.169 deserializes into [10.000 10.000].
        {
            let jammed_noun = Atom::from_u64(0b100100111001110001000011010000001);
            let bitstream = jammed_noun.as_bits();
            let noun = Noun::cue(bitstream)?;
            let cell = noun.as_cell()?;
            let (head, tail) = cell.as_parts();

            let head = head.as_atom()?;
            let tail = tail.as_atom()?;

            let _10_000 = Atom::from_u16(10_000);
            assert_eq!(head, &_10_000);
            assert_eq!(tail, &_10_000);
        }

        // 1.301.217.674.263.809 serializes to [999.999.999 999.999.999].
        {
            let jammed_noun = Atom::from_u64(0b100100111110111001101011001001111111111110100000001);
            let bitstream = jammed_noun.as_bits();
            let noun = Noun::cue(bitstream)?;
            let cell = noun.as_cell()?;
            let (head, tail) = cell.as_parts();

            let head = head.as_atom()?;
            let tail = tail.as_atom()?;

            let _999_999_999 = Atom::from_u32(999_999_999);
            assert_eq!(head, &_999_999_999);
            assert_eq!(tail, &_999_999_999);
        }

        // 635.080.761.093 deserializes into [[107 110] [107 110]].
        {
            let jammed_noun = Atom::from_u64(0b1001001111011101110000110101111100000101);
            let bitstream = jammed_noun.as_bits();
            let noun = Noun::cue(bitstream)?;
            let cell = noun.as_cell()?;
            let (head, tail) = cell.as_parts();

            let head = head.as_cell()?;
            let head_head = head.head().as_atom()?;
            let head_tail = head.tail().as_atom()?;

            let tail = tail.as_cell()?;
            let tail_head = tail.head().as_atom()?;
            let tail_tail = tail.tail().as_atom()?;

            let _107 = Atom::from_u8(107);
            let _110 = Atom::from_u8(110);
            assert_eq!(head_head, &_107);
            assert_eq!(head_tail, &_110);
            assert_eq!(tail_head, &_107);
            assert_eq!(tail_tail, &_110);
        }

        Ok(())
    }

    #[test]
    fn jam() -> Result<(), ()> {
        // 0 serializes to 2.
        {
            let noun = Atom::from_u8(0).into_noun();
            let jammed_noun = noun.jam()?;
            assert_eq!(Atom::from(jammed_noun), Atom::from_u8(2));
        }

        // 1 serializes to 12.
        {
            let noun = Atom::from_u8(1).into_noun();
            let jammed_noun = noun.jam()?;
            assert_eq!(Atom::from(jammed_noun), Atom::from_u8(12));
        }

        // 2 serializes to 72.
        {
            let noun = Atom::from_u8(2).into_noun();
            let jammed_noun = noun.jam()?;
            assert_eq!(Atom::from(jammed_noun), Atom::from_u8(72));
        }

        // 19 serializes to 2480.
        {
            let noun = Atom::from_u8(19).into_noun();
            let jammed_noun = noun.jam()?;
            assert_eq!(Atom::from(jammed_noun), Atom::from_u16(2480));
        }

        // 581.949.002 serializes to 1.191.831.557.952.
        {
            let noun = Atom::from_u32(581_949_002).into_noun();
            let jammed_noun = noun.jam()?;
            assert_eq!(Atom::from(jammed_noun), Atom::from_u64(1_191_831_557_952));
        }

        // [0 19] serializes into 39.689.
        {
            let head = Rc::new(Atom::from_u8(0).into_noun());
            let tail = Rc::new(Atom::from_u8(19).into_noun());
            let noun = Cell::from_parts(head, tail).into_noun();
            let jammed_noun = noun.jam()?;
            assert_eq!(Atom::from(jammed_noun), Atom::from_u16(39_689));
        }

        // [1 1] serializes to 817.
        {
            let head = Rc::new(Atom::from_u8(1).into_noun());
            let tail = head.clone();
            let noun = Cell::from_parts(head, tail).into_noun();
            let jammed_noun = noun.jam()?;
            assert_eq!(Atom::from(jammed_noun), Atom::from_u16(817));
        }

        // [[222 444 888] [222 444 888]] serializes to 170.479.614.045.978.345.989.
        {
            let _222 = Atom::from_u8(222).into_noun();
            let _444 = Atom::from_u16(444).into_noun();
            let _888 = Atom::from_u16(888).into_noun();
            let head = Rc::new(
                Cell::from_parts(
                    Rc::new(_222),
                    Rc::new(Cell::from_parts(Rc::new(_444), Rc::new(_888)).into_noun()),
                )
                .into_noun(),
            );
            let tail = head.clone();
            let noun = Cell::from_parts(head, tail).into_noun();
            let jammed_noun = noun.jam()?;
            assert_eq!(
                Atom::from(jammed_noun),
                Atom::from_u128(170_479_614_045_978_345_989)
            );
        }

        Ok(())
    }
}
