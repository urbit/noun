//! Arbitrarily large unsigned integer.
//!
//! [Atom]s represent arbitrary binary data and can be added and compared to one another.
//!
//! [Atom]: https://urbit.org/docs/glossary/atom

pub mod types;

use bitstream_io::{BitReader, LittleEndian};
use std::{fmt::Debug, ops::Add, str};

/// Convert an unsigned integer into an atom.
macro_rules! uint_to_atom {
    ($uint:expr, $atom:ty) => {{
        <$atom>::from(Vec::from($uint.to_le_bytes()))
    }};
}

/// Convert an atom into an unsigned integer, returning an error if the byte width of the atom
/// exceeds the byte width of the target unsigned integer type.
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
///
/// A non-zero atom must not have trailing zero bytes.
pub trait Atom: Add + Debug + Eq + From<Vec<u8>> + Sized {
    /// Create a new atom from an 8-bit unsigned integer.
    fn from_u8(uint: u8) -> Self {
        uint_to_atom!(uint, Self)
    }

    /// Create a new atom from an 16-bit unsigned integer.
    fn from_u16(uint: u16) -> Self {
        uint_to_atom!(uint, Self)
    }

    /// Create a new atom from an 32-bit unsigned integer.
    fn from_u32(uint: u32) -> Self {
        uint_to_atom!(uint, Self)
    }

    /// Create a new atom from an 64-bit unsigned integer.
    fn from_u64(uint: u64) -> Self {
        uint_to_atom!(uint, Self)
    }

    /// Create a new atom from a 128-bit unsigned integer.
    fn from_u128(uint: u128) -> Self {
        uint_to_atom!(uint, Self)
    }

    /// Create a new atom from an pointer-sized unsigned integer.
    fn from_usize(uint: usize) -> Self {
        uint_to_atom!(uint, Self)
    }

    /// Create a new atom from a UTF-8 encoded string.
    fn from_string(string: String) -> Self {
        Self::from(string.into_bytes())
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

    /// Convert an atom into a byte slice.
    fn as_bytes(&self) -> &[u8];

    /// Convert an atom into a little-endian bitstream.
    fn as_bits(&self) -> BitReader<&[u8], LittleEndian> {
        BitReader::new(self.as_bytes())
    }

    /// Convert an atom into an 8-bit unsigned integer, returning an error if the atom is greater
    /// than `u8::MAX`.
    fn as_u8(&self) -> Result<u8, ()> {
        atom_to_uint!(self, u8)
    }

    /// Convert an atom into an 16-bit unsigned integer, returning an error if the atom is greater
    /// than `u16::MAX`.
    fn as_u16(&self) -> Result<u16, ()> {
        atom_to_uint!(self, u16)
    }

    /// Convert an atom into an 32-bit unsigned integer, returning an error if the atom is greater
    /// than `u32::MAX`.
    fn as_u32(&self) -> Result<u32, ()> {
        atom_to_uint!(self, u32)
    }

    /// Convert an atom into an 64-bit unsigned integer, returning an error if the atom is greater
    /// than `u64::MAX`.
    fn as_u64(&self) -> Result<u64, ()> {
        atom_to_uint!(self, u64)
    }

    /// Convert an atom into an 64-bit unsigned integer, returning an error if the atom is greater
    /// than `u128::MAX`.
    fn as_u128(&self) -> Result<u128, ()> {
        atom_to_uint!(self, u128)
    }

    /// Convert an atom into an 64-bit unsigned integer, returning an error if the atom is greater
    /// than `usize::MAX`.
    fn as_usize(&self) -> Result<usize, ()> {
        atom_to_uint!(self, usize)
    }

    /// Convert an atom into a string slice, returning an error if the atom is not composed of
    /// valid UTF-8 bytes.
    fn as_str(&self) -> Result<&str, ()> {
        str::from_utf8(self.as_bytes()).map_err(|_| ())
    }
}

/// Convenience macro for creating a new atom.
#[macro_export]
macro_rules! new_atom {
    // u8 -> Atom
    ($val:expr, u8 to $atom:ty) => {
        <$atom>::from_u8($val)
    };
    // u16 -> Atom
    ($val:expr, u16 to $atom:ty) => {
        <$atom>::from_u16($val)
    };
    // u32 -> Atom
    ($val:expr, u32 to $atom:ty) => {
        <$atom>::from_u32($val)
    };
    // u64 -> Atom
    ($val:expr, u64 to $atom:ty) => {
        <$atom>::from_u64($val)
    };
    // u128 -> Atom
    ($val:expr, u128 to $atom:ty) => {
        <$atom>::from_u128($val)
    };
    // usize -> Atom
    ($val:expr, usize to $atom:ty) => {
        <$atom>::from_usize($val)
    };
    // Vec<u8> -> Atom
    ($val:expr, Vec to $atom:ty) => {
        <$atom>::from($val)
    };
}
