pub mod r#enum;

use bitstream_io::{BitRead, BitWrite};
use std::{collections::HashMap, hash::Hash};

pub trait Atom: IntoNoun + Sized {
    fn new(val: Vec<u8>) -> Self;

    fn as_bytes(&self) -> &[u8];
}

pub trait Cell: IntoNoun + Sized {
    type Head;
    type Tail;

    fn new(head: Option<Self::Head>, tail: Option<Self::Tail>) -> Self;

    fn into_parts(self) -> (Option<Self::Head>, Option<Self::Tail>);
}

pub trait Noun: Hash + Sized {
    type A: Atom;
    type Cell: Cell;

    fn get(&self, idx: usize) -> Option<&Self>;

    fn into_atom(self) -> Result<Self::A, ()>;

    fn into_cell(self) -> Result<Self::Cell, ()>;
}

/// Unifying equality.
pub trait UnifyEq: Eq {
    type Ctx;

    fn eq(&self, other: &Self, _ctx: Self::Ctx) -> bool;
}

pub trait Cue: Noun + Sized {
    type A: Atom;
    type Cell: Cell;

    fn cue(mut src: impl BitRead) -> Result<Self, ()> {
        let mut _cache: HashMap<usize, Self> = HashMap::new();
        let mut start_idx = 0;
        let mut curr_idx = start_idx;
        let mut _noun: Self;
        loop {
            curr_idx += 1;
            match src.read_bit() {
                Ok(true) => {
                    curr_idx += 1;
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
                    let (cue_val, _bits_read) = Self::cue_val(&mut src)?;
                    let atom = <Self as Cue>::A::new(cue_val).into_noun()?;
                    /*
                    cache.insert(start_idx, atom);
                    */
                }
                Err(_) => {
                    todo!("IO error")
                }
            }
            start_idx = curr_idx;
        }
        todo!()
    }

    /// Read the length of an atom or backreference, returning (length, bits read).
    fn cue_val_len(src: &mut impl BitRead) -> Result<(u64, u32), ()> {
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

    /// Get a cued value (either an atom or backreference), returning (bytes, bits read).
    fn cue_val(src: &mut impl BitRead) -> Result<(Vec<u8>, u32), ()> {
        let (mut len, mut bits_read) = Self::cue_val_len(src)?;

        let mut val = Vec::new();
        while len >= u64::from(u8::BITS) {
            let byte: u8 = src.read(u8::BITS).expect("read chunk");
            bits_read += u8::BITS;
            val.push(byte);
            len -= u64::from(u8::BITS);
        }
        // Consume remaining bits.
        let len = u32::try_from(len).unwrap();
        let byte: u8 = src.read(len).expect("read chunk");
        bits_read += len;
        val.push(byte);

        Ok((val, bits_read))
    }
}

pub trait Jam: Noun + Sized {
    fn jam(self, sink: &mut impl BitWrite) -> Result<(), ()>;
}

/// Convert a noun into the implementing type.
pub trait FromNoun: Sized {
    type Noun: Noun;

    fn from_noun(noun: Self::Noun) -> Result<Self, ()>;
}

/// Convert the implementing type into a noun.
pub trait IntoNoun: Sized {
    type Noun: Noun;

    fn into_noun(self) -> Result<Self::Noun, ()>;
}
