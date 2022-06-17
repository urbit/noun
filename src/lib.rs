pub mod r#enum;

use bitstream_io::{BitRead, BitWrite};
use std::{collections::HashMap, hash::Hash};

/// (<some type>, bits read)
type CueResult<T> = Result<(T, u32), ()>;

pub trait Atom<C, N>
where
    C: Cell<Self, N>,
    N: Noun<Self, C>,
    Self: IntoNoun<Self, C, N> + Sized,
{
    fn new(val: Vec<u8>) -> Self;

    fn as_bytes(&self) -> &[u8];
}

pub trait Cell<A, N>
where
    A: Atom<Self, N>,
    N: Noun<A, Self>,
    Self: IntoNoun<A, Self, N> + Sized,
{
    type Head;
    type Tail;

    fn new(head: Option<Self::Head>, tail: Option<Self::Tail>) -> Self;

    fn into_parts(self) -> (Option<Self::Head>, Option<Self::Tail>);
}

pub trait Noun<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Hash + Sized,
{
    fn get(&self, idx: usize) -> Option<&Self>;

    fn into_atom(self) -> Result<A, ()>;

    fn into_cell(self) -> Result<C, ()>;
}

/// Unifying equality.
pub trait UnifyEq<C>
where
    Self: Eq,
{
    fn eq(&self, other: &Self, _ctx: C) -> bool;
}

pub trait Cue<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Noun<A, C> + Sized,
{
    fn cue(mut src: impl BitRead) -> Result<Self, ()> {
        let mut cache: HashMap<usize, Self> = HashMap::new();
        let (noun, _) = Self::decode(&mut src, &mut cache, 0)?;
        Ok(noun)
    }

    /// Recursively decode a bitstream.
    fn decode(
        src: &mut impl BitRead,
        _cache: &mut HashMap<usize, Self>,
        _pos: usize,
    ) -> CueResult<Self> {
        loop {
            match src.read_bit() {
                Ok(true) => {
                    match src.read_bit() {
                        // Back reference tag = 0b11.
                        Ok(true) => {
                            todo!("back reference");
                        }
                        // Cell tag = 0b01.
                        Ok(false) => {
                            todo!("cell");
                        }
                        Err(_) => todo!("IO error"),
                    }
                }
                // Atom tag = 0b0.
                Ok(false) => {
                    let (_atom, _bits_read) = Self::decode_atom(src)?;
                    //cache.insert(start_idx, atom);
                    todo!()
                }
                Err(_) => {
                    todo!("IO error")
                }
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
    fn decode_atom(src: &mut impl BitRead) -> CueResult<Self> {
        // Decode the atom length.
        let (mut bit_len, mut bits_read) = Self::decode_len(src)?;

        // This will allocate an extra byte when bit_len is a multiple of u8::BITS, but it's worth it
        // to omit a branch.
        let byte_len = (bit_len / u64::from(u8::BITS)) + 1;
        let byte_len = usize::try_from(byte_len).expect("u64 doesn't fit in usize");

        let mut val = Vec::with_capacity(byte_len);
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

        let atom = A::new(val).into_noun().unwrap();

        Ok((atom, bits_read))
    }

    /// Decode a cell, returning (cell, bits read).
    fn decode_cell(_src: &mut impl BitRead) -> CueResult<Self> {
        todo!()
    }
}

pub trait Jam<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Noun<A, C> + Sized,
{
    fn jam(self, sink: &mut impl BitWrite) -> Result<(), ()>;
}

/// Convert a noun into the implementing type.
pub trait FromNoun<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    fn from_noun(noun: N) -> Result<Self, ()>;
}

/// Convert the implementing type into a noun.
pub trait IntoNoun<A, C, N>
where
    A: Atom<C, N>,
    C: Cell<A, N>,
    N: Noun<A, C>,
    Self: Sized,
{
    fn into_noun(self) -> Result<N, ()>;
}
