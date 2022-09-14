//! Arbiratily large unsigned integers.
//!
//! An [atom] is an arbitrarily large unsigned integer represented as a little-endian contiguous
//! sequence of bytes. An atom can be:
//! - created a single bit at a time or from other types that can be easily converted into atoms
//!   like primitive unsigned integers, strings, and string slices;
//! - iterated over a single bit at a time;
//! - compared to other atoms and other atom-like types;
//! - used as an addend;
//! - pretty-printed as a hexadecimal number;
//! - converted into a noun, a primitive unsigned integer type, or a string slice.
//!
//! [atom]: https://developers.urbit.org/reference/glossary/atom

use std::{
    collections::hash_map::DefaultHasher,
    ffi::OsStr,
    fmt::{Display, Error, Formatter},
    hash::Hasher,
    ops::{Add, Div, Rem, Sub},
    str::{self, Utf8Error},
};

/// Returns the length in bits of a sequence of bytes.
fn bit_len(bytes: &[u8]) -> usize {
    if let Some(last_byte) = bytes.last() {
        let byte_len = u32::try_from(bytes.len()).expect("usize to u32");
        let bit_len = u8::BITS * (byte_len - 1) + (u8::BITS - last_byte.leading_zeros());
        usize::try_from(bit_len).expect("u32 to usize")
    } else {
        0
    }
}

/// A bitwise [`Atom`] builder.
pub struct Builder {
    bytes: Vec<u8>,
    bit_idx: usize,
}

impl Builder {
    /// Creates an empty atom builder.
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            bit_idx: 0,
        }
    }

    /// Returns the current bitwise position of the end of this builder.
    pub fn pos(&self) -> usize {
        self.bit_idx
    }

    /// Pushes a bit onto the end of this builder.
    pub fn push_bit(&mut self, bit: bool) {
        let u8_bits = usize::try_from(u8::BITS).expect("u32 to usize");
        let byte_idx = self.bit_idx / u8_bits;
        if byte_idx == self.bytes.len() {
            self.bytes.push(0);
        }
        let byte = &mut self.bytes[byte_idx];
        let shift = self.bit_idx % u8_bits;
        if bit {
            *byte |= 1 << shift;
        } else {
            *byte &= !(1 << shift);
        }
        self.bit_idx += 1;
    }

    /// Converts this builder into an `Atom`, consuming the builder.
    pub fn into_atom(self) -> Atom {
        let bytes = self.bytes;
        let bit_len = bit_len(&bytes[..]);
        Atom { bytes, bit_len }
    }
}

/// An arbitrarily large unsigned integer represented as a [`Vec<u8>`].
#[derive(Eq, Clone, Debug, Hash)]
pub struct Atom {
    bytes: Vec<u8>,
    bit_len: usize,
}

/// Converts an atom into an unsigned integer, returning `None` if the byte width of the atom
/// exceeds the byte width of the target unsigned integer type.
macro_rules! atom_as_uint {
    ($atom:expr, $uint:ty) => {{
        let atom = $atom.as_bytes();
        const N: usize = std::mem::size_of::<$uint>();
        let len = atom.len();
        if len <= N {
            let mut bytes: [u8; N] = [0; N];
            let _ = &mut bytes[..len].copy_from_slice(atom);
            Some(<$uint>::from_le_bytes(bytes))
        } else {
            None
        }
    }};
}

impl Atom {
    /// Creates an empty atom builder.
    ///
    /// This method is equivalent to `Builder::new()`.
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Creates the atom `0`.
    pub const fn null() -> Self {
        Self {
            bytes: Vec::new(),
            bit_len: 0,
        }
    }

    /// Returns `true` if this atom is null (i.e. the atom `0`).
    pub const fn is_null(&self) -> bool {
        self.bit_len() == 0
    }

    /// Returns the length in bits of this atom.
    pub const fn bit_len(&self) -> usize {
        self.bit_len
    }

    /// Computes the hash of this atom.
    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(self.as_bytes());
        hasher.finish()
    }

    /// Converts this atom into a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Converts this atom into a string slice, returning an error if the atom is not composed of
    /// valid UTF-8 bytes.
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts this atom into an 8-bit unsigned integer, returning `None` if the atom is greater
    /// than `u8::MAX`.
    ///
    /// # Examples
    /// ```
    /// # use noun::{atom::Atom, atom};
    /// let uint = u8::MAX;
    /// let atom = atom!(uint);
    /// assert_eq!(atom.as_u8().unwrap(), uint);
    /// ```
    pub fn as_u8(&self) -> Option<u8> {
        atom_as_uint!(self, u8)
    }

    /// Converts this atom into an 16-bit unsigned integer, returning `None` if the atom is greater
    /// than `u16::MAX`.
    ///
    /// # Examples
    /// ```
    /// # use noun::{atom::Atom, atom};
    /// let uint = u16::MAX;
    /// let atom = atom!(uint);
    /// assert_eq!(atom.as_u16().unwrap(), uint);
    /// ```
    pub fn as_u16(&self) -> Option<u16> {
        atom_as_uint!(self, u16)
    }

    /// Converts this atom into an 32-bit unsigned integer, returning `None` if the atom is greater
    /// than `u32::MAX`.
    ///
    /// # Examples
    /// ```
    /// # use noun::{atom::Atom, atom};
    /// let uint = u32::MAX;
    /// let atom = atom!(uint);
    /// assert_eq!(atom.as_u32().unwrap(), uint);
    /// ```
    pub fn as_u32(&self) -> Option<u32> {
        atom_as_uint!(self, u32)
    }

    /// Converts this atom into an 64-bit unsigned integer, returning `None` if the atom is greater
    /// than `u64::MAX`.
    ///
    /// # Examples
    /// ```
    /// # use noun::{atom::Atom, atom};
    /// let uint = u64::MAX;
    /// let atom = atom!(uint);
    /// assert_eq!(atom.as_u64().unwrap(), uint);
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        atom_as_uint!(self, u64)
    }

    /// Converts this atom into an 128-bit unsigned integer, returning `None` if the atom is greater
    /// than `u128::MAX`.
    ///
    /// # Examples
    /// ```
    /// # use noun::{atom::Atom, atom};
    /// let uint = u128::MAX;
    /// let atom = atom!(uint);
    /// assert_eq!(atom.as_u128().unwrap(), uint);
    /// ```
    pub fn as_u128(&self) -> Option<u128> {
        atom_as_uint!(self, u128)
    }

    /// Converts this atom into a pointer-sized unsigned integer, returning `None` if the atom is
    /// greater than `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// # use noun::{atom::Atom, atom};
    /// let uint = usize::MAX;
    /// let atom = atom!(uint);
    /// assert_eq!(atom.as_usize().unwrap(), uint);
    /// ```
    pub fn as_usize(&self) -> Option<usize> {
        atom_as_uint!(self, usize)
    }

    /// Copies this atom into a byte vector.
    pub fn to_vec(&self) -> Vec<u8> {
        Vec::from(self.as_bytes())
    }

    /// Converts this atom into a byte vector, consuming the atom.
    ///
    /// This method does not allocate on the heap.
    pub fn into_vec(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns a bitwise iterator over this atom.
    pub fn iter(&self) -> Iter {
        Iter {
            atom: self,
            bit_idx: 0,
            bit_mask: 0b1,
        }
    }
}

impl Add for Atom {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!("{} + {}", self, rhs)
    }
}

/// Add an unsigned integer primitive to an atom.
macro_rules! atom_add_uint {
    ($uint:ty) => {
        impl Add<$uint> for Atom {
            type Output = Self;

            fn add(self, rhs: $uint) -> Self::Output {
                todo!("{} + {}", self, rhs)
            }
        }

        impl Add<$uint> for &Atom {
            type Output = Atom;

            fn add(self, rhs: $uint) -> Self::Output {
                todo!("{} + {}", self, rhs)
            }
        }
    };
}

atom_add_uint!(u8);
atom_add_uint!(u16);
atom_add_uint!(u32);
atom_add_uint!(u64);
atom_add_uint!(u128);
atom_add_uint!(usize);

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "0x")?;
        if self.bytes.is_empty() {
            write!(f, "0")
        } else {
            for (i, byte) in (&self.bytes).iter().enumerate() {
                if i > 0 && i % 4 == 0 {
                    write!(f, ".")?;
                }
                write!(f, "{:x}", byte)?;
            }
            Ok(())
        }
    }
}

impl Div for Atom {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!("{} / {}", self, rhs)
    }
}

/// Divide an atom by an unsigned integer primitive.
macro_rules! atom_div_uint {
    ($uint:ty) => {
        impl Div<$uint> for Atom {
            type Output = Self;

            fn div(self, rhs: $uint) -> Self::Output {
                todo!("{} / {}", self, rhs)
            }
        }

        impl Div<$uint> for &Atom {
            type Output = Atom;

            fn div(self, rhs: $uint) -> Self::Output {
                todo!("{} / {}", self, rhs)
            }
        }
    };
}

atom_div_uint!(u8);
atom_div_uint!(u16);
atom_div_uint!(u32);
atom_div_uint!(u64);
atom_div_uint!(u128);
atom_div_uint!(usize);

impl TryFrom<&OsStr> for Atom {
    type Error = ();

    fn try_from(string: &OsStr) -> Result<Self, Self::Error> {
        Ok(Self::from(string.to_str().ok_or(())?))
    }
}

impl From<&str> for Atom {
    fn from(string: &str) -> Self {
        let bytes = string.as_bytes().to_vec();
        let bit_len = bit_len(&bytes[..]);
        Self { bytes, bit_len }
    }
}

impl From<String> for Atom {
    fn from(string: String) -> Self {
        Self::from(string.into_bytes())
    }
}

/// Convert an unsigned integer primitive into an atom.
macro_rules! atom_from_uint {
    ($uint:ty) => {
        impl From<$uint> for Atom {
            fn from(uint: $uint) -> Self {
                Atom::from(Vec::from(uint.to_le_bytes()))
            }
        }
    };
}

atom_from_uint!(u8);
atom_from_uint!(u16);
atom_from_uint!(u32);
atom_from_uint!(u64);
atom_from_uint!(u128);
atom_from_uint!(usize);

impl From<Vec<u8>> for Atom {
    fn from(mut vec: Vec<u8>) -> Self {
        let len = match vec.iter().rposition(|x| *x != 0) {
            Some(idx) => idx + 1,
            None => 0,
        };
        vec.truncate(len);
        let bit_len = bit_len(&vec[..]);
        Self {
            bytes: vec,
            bit_len,
        }
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl PartialEq<&Self> for Atom {
    fn eq(&self, other: &&Self) -> bool {
        self.bytes == other.bytes
    }
}

impl PartialEq<str> for Atom {
    fn eq(&self, other: &str) -> bool {
        if let Ok(string) = str::from_utf8(self.as_bytes()) {
            string == other
        } else {
            false
        }
    }
}

impl PartialEq<&str> for Atom {
    fn eq(&self, other: &&str) -> bool {
        if let Ok(string) = str::from_utf8(self.as_bytes()) {
            string == *other
        } else {
            false
        }
    }
}

/// Compares an atom to an unsigned integer primitive.
macro_rules! atom_eq_uint {
    ($uint:ty, $as_uint:ident) => {
        impl PartialEq<$uint> for Atom {
            fn eq(&self, other: &$uint) -> bool {
                if let Some(uint) = self.$as_uint() {
                    uint == *other
                } else {
                    false
                }
            }
        }
    };
}

atom_eq_uint!(u8, as_u8);
atom_eq_uint!(u16, as_u16);
atom_eq_uint!(u32, as_u32);
atom_eq_uint!(u64, as_u64);
atom_eq_uint!(u128, as_u128);
atom_eq_uint!(usize, as_usize);

impl Rem for Atom {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!("{} % {}", self, rhs)
    }
}

/// Get the remainder of an atom divided by an unsigned integer primitive.
macro_rules! atom_rem_uint {
    ($uint:ty) => {
        impl Rem<$uint> for Atom {
            type Output = Self;

            fn rem(self, rhs: $uint) -> Self::Output {
                todo!("{} % {}", self, rhs)
            }
        }

        impl Rem<$uint> for &Atom {
            type Output = Atom;

            fn rem(self, rhs: $uint) -> Self::Output {
                todo!("{} % {}", self, rhs)
            }
        }
    };
}

atom_rem_uint!(u8);
atom_rem_uint!(u16);
atom_rem_uint!(u32);
atom_rem_uint!(u64);
atom_rem_uint!(u128);
atom_rem_uint!(usize);

impl Sub for Atom {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!("{} - {}", self, rhs)
    }
}

/// Subtract an unsigned integer primitive from an atom.
macro_rules! atom_sub_uint {
    ($uint:ty) => {
        impl Sub<$uint> for Atom {
            type Output = Self;

            fn sub(self, rhs: $uint) -> Self::Output {
                todo!("{} - {}", self, rhs)
            }
        }

        impl Sub<$uint> for &Atom {
            type Output = Atom;

            fn sub(self, rhs: $uint) -> Self::Output {
                todo!("{} - {}", self, rhs)
            }
        }
    };
}

atom_sub_uint!(u8);
atom_sub_uint!(u16);
atom_sub_uint!(u32);
atom_sub_uint!(u64);
atom_sub_uint!(u128);
atom_sub_uint!(usize);

/// An iterator over the bits of an [`Atom`].
///
/// Iteration starts with the least significant bit of the [`Atom`] and ends with the most
/// significant bit.
pub struct Iter<'a> {
    /// Atom being interated over.
    atom: &'a Atom,
    /// Index of the current bit.
    bit_idx: usize,
    /// Mask to access current bit.
    bit_mask: u8,
}

impl<'a> Iter<'_> {
    /// Returns the current bitwise position of this iterator.
    pub fn pos(&self) -> usize {
        self.bit_idx
    }
}

impl Iterator for Iter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit_idx == self.atom.bit_len {
            return None;
        }
        let byte_idx = self.bit_idx / usize::try_from(u8::BITS).expect("u32 to usize");
        let bit = (self.atom.bytes[byte_idx] & self.bit_mask) != 0;
        self.bit_mask = self.bit_mask.rotate_left(1);
        self.bit_idx += 1;
        Some(bit)
    }
}

/// Creates a new atom from an expression.
///
/// [`Atom`] must implement [`From`] for the type of the expression. This is syntactic sugar for
/// `Atom::from()`.
#[macro_export]
macro_rules! atom {
    () => {
        $crate::atom::Atom::null()
    };
    ($atom_src:expr) => {
        $crate::atom::Atom::from($atom_src)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_len() {
        {
            let num = 0b111u8.to_le_bytes();
            assert_eq!(super::bit_len(&num[..]), 3);
        }

        {
            let num = 0b10001011u8.to_le_bytes();
            assert_eq!(super::bit_len(&num[..]), 8);
        }

        {
            let num = 0b100000000u16.to_le_bytes();
            assert_eq!(super::bit_len(&num[..]), 9);
        }

        {
            let num = [
                0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
                0x37,
            ];
            assert_eq!(super::bit_len(&num[..]), 134);
        }
    }

    #[test]
    fn is_null() {
        assert!(Atom::from(0u8).is_null());
        assert!(!Atom::from(1u8).is_null());
    }

    #[test]
    fn iter() {
        {
            let atom = atom!(0b0u8);
            let mut atom_iter = atom.iter();
            assert_eq!(None, atom_iter.next());
        }

        {
            let atom = atom!(0b10u8);
            let mut atom_iter = atom.iter();
            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(true), atom_iter.next());
            assert_eq!(None, atom_iter.next());
        }

        {
            let atom = atom!(0x2f004u32);
            let mut atom_iter = atom.iter();
            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(true), atom_iter.next());
            assert_eq!(Some(false), atom_iter.next());

            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(false), atom_iter.next());

            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(false), atom_iter.next());

            assert_eq!(Some(true), atom_iter.next());
            assert_eq!(Some(true), atom_iter.next());
            assert_eq!(Some(true), atom_iter.next());
            assert_eq!(Some(true), atom_iter.next());

            assert_eq!(Some(false), atom_iter.next());
            assert_eq!(Some(true), atom_iter.next());

            assert_eq!(None, atom_iter.next());
        }
    }

    #[test]
    fn partial_eq() {
        {
            let lh = atom!("The Importance of Being Ernest");
            let rh = atom!("The Importance of Being Ernest");
            assert_eq!(lh, rh);
        }

        {
            let lh = atom!("Oh, to be a glove");
            let rh = atom!("upon that hand.");
            assert_ne!(lh, rh);
        }

        {
            let string = "hello";
            let atom = atom!(string);
            assert_eq!(atom, string);
        }

        {
            let atom = atom!("hello");
            assert_ne!(atom, "goodbye");
        }

        {
            macro_rules! uint_eq_test {
                ($uint:expr) => {
                    let atom = atom!($uint);
                    assert_eq!(atom, $uint);
                };
            }

            uint_eq_test!(0u8);
            uint_eq_test!(107u8);
            uint_eq_test!(16_000u16);
            uint_eq_test!(949_543_111u32);
            uint_eq_test!(184_884_819u64);
            uint_eq_test!(19_595_184_881_994_188_181u128);
            uint_eq_test!(10_101_044_481_818usize);
        }

        {
            macro_rules! uint_ne_test {
                ($atom:expr, $uint:expr) => {
                    let atom = atom!($atom);
                    assert_ne!(atom, $uint);
                };
            }

            uint_ne_test!(97u8, 103u8);
            uint_ne_test!(98u8, 64_222u16);
            uint_ne_test!(99u8, 777_919_400u32);
            uint_ne_test!(100u8, 881_944_000_887u64);
            uint_ne_test!(881_944_000_887u64, 21_601_185_860_100_176_183u128);
            uint_ne_test!(64_222u16, 127usize);
        }
    }
}
