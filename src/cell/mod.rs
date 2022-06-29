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
