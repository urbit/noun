use crate::{
    types::{cell::Cell, noun::Noun},
    Atom as _Atom, IntoNoun,
};
use std::{hash::Hash, ops::Add, str};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Atom(Vec<u8>);

impl Add for Atom {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Add<u8> for Atom {
    type Output = Self;

    fn add(self, _rhs: u8) -> Self::Output {
        todo!()
    }
}

impl Add<u16> for Atom {
    type Output = Self;

    fn add(self, _rhs: u16) -> Self::Output {
        todo!()
    }
}

impl Add<u32> for Atom {
    type Output = Self;

    fn add(self, _rhs: u32) -> Self::Output {
        todo!()
    }
}

impl Add<u64> for Atom {
    type Output = Self;

    fn add(self, _rhs: u64) -> Self::Output {
        todo!()
    }
}

impl Add<u128> for Atom {
    type Output = Self;

    fn add(self, _rhs: u128) -> Self::Output {
        todo!()
    }
}

impl Add<usize> for Atom {
    type Output = Self;

    fn add(self, _rhs: usize) -> Self::Output {
        todo!()
    }
}

impl _Atom<Cell, Noun> for Atom {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn into_noun(self) -> Noun {
        Noun::Atom(self)
    }
}

impl From<Vec<u8>> for Atom {
    fn from(val: Vec<u8>) -> Self {
        Self(val)
    }
}

impl From<&str> for Atom {
    fn from(val: &str) -> Self {
        Self(val.as_bytes().to_vec())
    }
}

impl IntoNoun<Self, Cell, Noun> for Atom {
    fn as_noun(&self) -> Result<Noun, ()> {
        Err(())
    }

    fn into_noun(self) -> Result<Noun, ()> {
        Ok(Noun::Atom(self))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_uint() -> Result<(), ()> {
        {
            let val = u8::MAX;
            let atom = Atom::from_u8(val);
            assert_eq!(atom.as_u8()?, val);
        }

        {
            let val = u16::MAX;
            let atom = Atom::from_u16(val);
            assert_eq!(atom.as_u16()?, val);
        }

        {
            let val = u32::MAX;
            let atom = Atom::from_u32(val);
            assert_eq!(atom.as_u32()?, val);
        }

        {
            let val = u64::MAX;
            let atom = Atom::from_u64(val);
            assert_eq!(atom.as_u64()?, val);
        }

        {
            let val = u128::MAX;
            let atom = Atom::from_u128(val);
            assert_eq!(atom.as_u128()?, val);
        }

        {
            let val = usize::MAX;
            let atom = Atom::from_usize(val);
            assert_eq!(atom.as_usize()?, val);
        }

        Ok(())
    }

    #[test]
    fn partialeq() {
        {
            let vec = vec![b'h', b'e', b'l', b'l', b'o'];
            let atom = Atom::from(vec);
            assert_eq!(atom, "hello");
        }
    }
}
