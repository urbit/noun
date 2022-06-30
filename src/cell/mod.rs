//! Ordered pair of nouns.
//!
//! Cells have a head and a tail and can be compared to one another.

pub mod types;

use std::fmt::Debug;

/// Interface to the cell data structure.
pub trait Cell
where
    Self: Debug + Eq + Sized,
{
    /// Head of the cell.
    type Head;

    /// Tail of the cell.
    type Tail;

    /// Create a new cell from a head and a tail.
    fn new(head: Self::Head, tail: Self::Tail) -> Self;

    /// Returns the head of the cell.
    fn head(&self) -> &Self::Head;

    /// Returns the tail of the cell.
    fn tail(&self) -> &Self::Tail;

    /// Converts a cell into its head and tail, consuming the cell.
    fn into_parts(self) -> (Self::Head, Self::Tail);
}

/// Convenience macro for creating a new cell.
#[macro_export]
macro_rules! new_cell {
    // [a b] -> Cell
    (($a:expr, $b:expr) to $cell:ty) => {
        <$cell>::new($a, $b)
    };
    // [a b c] -> Cell
    (($a:expr, $b:expr, $c:expr) to $cell:ty) => {
        <$cell>::new($a, cell!(($b, $c) to $cell))
    };
    // [a b c d] -> Cell
    (($a:expr, $b:expr, $c:expr, $d:expr) to $cell:ty) => {
        <$cell>::new($a, cell!(($b, $c, $d) to $cell))
    };
    // [a b c d e] -> Cell
    (($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) to $cell:ty) => {
        <$cell>::new($a, cell!(($b, $c, $d, $e) to $cell))
    };
}
