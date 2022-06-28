//! Assorted [`Atom`] implementations.

use crate::{atom::Atom, cell::types::RcCell, convert::IntoNoun, noun::types::EnumNoun};
use std::{hash::Hash, ops::Add, str};

/// Atom represented as a vector of bytes.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VecAtom(Vec<u8>);

impl Add for VecAtom {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Atom for VecAtom {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for VecAtom {
    fn from(val: Vec<u8>) -> Self {
        Self(val)
    }
}

impl From<&str> for VecAtom {
    fn from(val: &str) -> Self {
        Self(val.as_bytes().to_vec())
    }
}

impl IntoNoun<Self, RcCell, EnumNoun<Self, RcCell>> for VecAtom {
    fn as_noun(&self) -> Result<EnumNoun<Self, RcCell>, ()> {
        unimplemented!("An EnumNoun cannot be constructed from &VecAtom.");
    }

    fn as_noun_unchecked(&self) -> EnumNoun<Self, RcCell> {
        unimplemented!("An EnumNoun cannot be constructed from &VecAtom.");
    }

    fn into_noun(self) -> Result<EnumNoun<Self, RcCell>, ()> {
        Ok(self.into_noun_unchecked())
    }

    fn into_noun_unchecked(self) -> EnumNoun<Self, RcCell> {
        EnumNoun::Atom(self)
    }
}

impl PartialEq<str> for VecAtom {
    fn eq(&self, other: &str) -> bool {
        if let Ok(string) = str::from_utf8(self.as_bytes()) {
            string == other
        } else {
            false
        }
    }
}

impl PartialEq<&str> for VecAtom {
    fn eq(&self, other: &&str) -> bool {
        if let Ok(string) = str::from_utf8(self.as_bytes()) {
            string == *other
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_uint() -> Result<(), ()> {
        fn run_test<A>() -> Result<(), ()>
        where
            A: Atom,
        {
            {
                let val = u8::MAX;
                let atom = A::from_u8(val);
                assert_eq!(atom.as_u8()?, val);
            }

            {
                let val = u16::MAX;
                let atom = A::from_u16(val);
                assert_eq!(atom.as_u16()?, val);
            }

            {
                let val = u32::MAX;
                let atom = A::from_u32(val);
                assert_eq!(atom.as_u32()?, val);
            }

            {
                let val = u64::MAX;
                let atom = A::from_u64(val);
                assert_eq!(atom.as_u64()?, val);
            }

            {
                let val = u128::MAX;
                let atom = A::from_u128(val);
                assert_eq!(atom.as_u128()?, val);
            }

            {
                let val = usize::MAX;
                let atom = A::from_usize(val);
                assert_eq!(atom.as_usize()?, val);
            }

            Ok(())
        }

        run_test::<VecAtom>()?;
        Ok(())
    }

    #[test]
    fn partialeq() {
        fn run_test<'a, A>()
        where
            A: Atom + PartialEq<&'a str>,
        {
            {
                let vec = vec![b'h', b'e', b'l', b'l', b'o'];
                let atom = A::from(vec);
                assert_eq!(atom, "hello");
            }
        }

        run_test::<VecAtom>();
    }
}
