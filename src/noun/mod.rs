//! Finite size binary tree with atoms as leaves.
//!
//! [noun]: https://urbit.org/docs/glossary/noun

pub mod types;

use crate::{atom::Atom, cell::Cell};
use std::{fmt::Debug, hash::Hash};

/// Interface to the noun data structure.
pub trait Noun<A, C>
where
    A: Atom,
    C: Cell,
    Self: Debug + Eq + Hash + Sized,
{
    fn as_atom(&self) -> Result<&A, ()>;

    fn as_cell(&self) -> Result<&C, ()>;

    fn into_atom(self) -> Result<A, Self>;

    fn into_cell(self) -> Result<C, Self>;
}

/// Convenience macro for creating a new noun.
#[macro_export]
macro_rules! new_noun {
    // u8 -> Atom -> Noun
    ($val:expr, u8 to $atom:ty) => {
        <$atom>::from_u8($val).into_noun_unchecked()
    };
    // u16 -> Atom -> Noun
    ($val:expr, u16 to $atom:ty) => {
        <$atom>::from_u16($val).into_noun_unchecked()
    };
    // u32 -> Atom -> Noun
    ($val:expr, u32 to $atom:ty) => {
        <$atom>::from_u32($val).into_noun_unchecked()
    };
    // u64 -> Atom -> Noun
    ($val:expr, u64 to $atom:ty) => {
        <$atom>::from_u64($val).into_noun_unchecked()
    };
    // u128 -> Atom
    ($val:expr, u128 to $atom:ty) => {
        <$atom>::from_u128($val).into_noun_unchecked()
    };
    // usize -> Atom -> Noun
    ($val:expr, usize to $atom:ty) => {
        <$atom>::from_usize($val).into_noun_unchecked()
    };
    // Vec<u8> -> Atom -> Noun
    ($val:expr, Vec to $atom:ty) => {
        <$atom>::from($val).into_noun_unchecked()
    };
    // [a b] -> Cell -> Noun
    (($a:expr, $b:expr) to $cell:ty) => {
        <$cell>::new($a, $b).into_noun_unchecked()
    };
    // [a b c] -> Cell -> Noun
    (($a:expr, $b:expr, $c:expr) to $cell:ty) => {
        <$cell>::new($a, new_noun!(($b, $c) to $cell)).into_noun_unchecked()
    };
    // [a b c d] -> Cell -> Noun
    (($a:expr, $b:expr, $c:expr, $d:expr) to $cell:ty) => {
        <$cell>::new($a, new_noun!(($b, $c, $d) to $cell)).into_noun_unchecked()
    };
    // [a b c d e] -> Cell -> Noun
    (($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) to $cell:ty) => {
        <$cell>::new($a, new_noun!(($b, $c, $d, $e) to $cell)).into_noun_unchecked()
    };
}
