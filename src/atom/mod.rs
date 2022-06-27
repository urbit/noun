//! An [atom] is an arbitrarily large unsigned integer.
//!
//! [atom]: https://urbit.org/docs/glossary/atom

pub mod types;

use bitstream_io::{BitReader, LittleEndian};
use std::{fmt::Debug, ops::Add, str};

macro_rules! uint_to_atom {
    ($uint:expr, $atom:ty) => {{
        let mut vec = Vec::from($uint.to_le_bytes());
        if let Some(idx) = vec.iter().rposition(|x| *x != 0) {
            let len = idx + 1;
            vec.truncate(len);
        }
        <$atom>::from(vec)
    }};
}

macro_rules! atom_to_uint {
    ($atom:expr, $uint:ty) => {{
        let atom = $atom.as_bytes();
        const N: usize = std::mem::size_of::<$uint>();
        let len = atom.len();
        if len <= N {
            let mut bytes: [u8; N] = [0; N];
            let _ = &mut bytes[..len].copy_from_slice(atom);
            Ok(<$uint>::from_le_bytes(bytes))
        } else {
            Err(())
        }
    }};
}

/// Interface to the atom data structure.
pub trait Atom: Add<Self>
        + Debug
        + Eq
        + From<Vec<u8>>
        + PartialEq<&'static str>
        + Sized
{
    fn from_u8(uint: u8) -> Self {
        uint_to_atom!(uint, Self)
    }

    fn from_u16(uint: u16) -> Self {
        uint_to_atom!(uint, Self)
    }

    fn from_u32(uint: u32) -> Self {
        uint_to_atom!(uint, Self)
    }

    fn from_u64(uint: u64) -> Self {
        uint_to_atom!(uint, Self)
    }

    fn from_u128(uint: u128) -> Self {
        uint_to_atom!(uint, Self)
    }

    fn from_usize(uint: usize) -> Self {
        uint_to_atom!(uint, Self)
    }

    /// Get the length in bytes of an atom. This is equivalent to `self.as_bytes().len()`.
    fn byte_len(&self) -> usize {
        self.as_bytes().len()
    }

    /// Get the length in bits of an atom.
    ///
    /// # Examples
    ///
    /// ```
    /// # use noun::atom::{types::VecAtom as Atom, Atom as _};
    /// let _7 = Atom::from_u8(7);
    /// assert_eq!(_7.bit_len(), 3);
    ///
    /// let _139 = Atom::from_u8(139);
    /// assert_eq!(_139.bit_len(), 8);
    ///
    /// let _256 = Atom::from_u16(256);
    /// assert_eq!(_256.bit_len(), 9);
    /// ```
    fn bit_len(&self) -> usize {
        let bytes = self.as_bytes();
        if let Some(last_byte) = bytes.last() {
            let byte_len = u32::try_from(bytes.len()).unwrap();
            let bit_len = u8::BITS * (byte_len - 1) + (u8::BITS - last_byte.leading_zeros());
            usize::try_from(bit_len).unwrap()
        } else {
            0
        }
    }

    fn as_bytes(&self) -> &[u8];

    fn as_bits(&self) -> BitReader<&[u8], LittleEndian> {
        BitReader::new(self.as_bytes())
    }

    fn as_u8(&self) -> Result<u8, ()> {
        atom_to_uint!(self, u8)
    }

    fn as_u16(&self) -> Result<u16, ()> {
        atom_to_uint!(self, u16)
    }

    fn as_u32(&self) -> Result<u32, ()> {
        atom_to_uint!(self, u32)
    }

    fn as_u64(&self) -> Result<u64, ()> {
        atom_to_uint!(self, u64)
    }

    fn as_u128(&self) -> Result<u128, ()> {
        atom_to_uint!(self, u128)
    }

    fn as_usize(&self) -> Result<usize, ()> {
        atom_to_uint!(self, usize)
    }

    fn as_str(&self) -> Result<&str, ()> {
        Ok(str::from_utf8(self.as_bytes()).map_err(|_| ())?)
    }
}
