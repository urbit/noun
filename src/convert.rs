//! Conversions to and from nouns.

use std::fmt::{self, Display, Formatter};

/// Errors that occur when converting from a noun.
#[derive(Debug)]
pub enum Error {
    /// An atom could not be converted into an unsigned integer.
    AtomToUint,
    /// An atom could not be converted into a string.
    AtomToStr,
    /// A null atom was expected.
    ExpectedNull,
    /// An error specific to the implementing type occurred.
    ImplType,
    /// No value exists at a particular axis of a cell.
    MissingValue,
    /// Encountered an atom when a cell was expected.
    UnexpectedAtom,
    /// Encountered a cell when an atom was expected.
    UnexpectedCell,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::AtomToUint => write!(
                f,
                "the atom is too large to fit in the unsigned integer type"
            ),
            Self::AtomToStr => write!(f, "the atom is not composed of valid UTF-8 bytes"),
            Self::ExpectedNull => write!(f, "a null atom was expected"),
            Self::ImplType => write!(f, "an error specific to the implementing type occurred"),
            Self::MissingValue => write!(f, "the noun does not have a value at this axis"),
            Self::UnexpectedAtom => write!(f, "an atom was encountered when a cell was expected"),
            Self::UnexpectedCell => write!(f, "a cell was encountered when an atom was expected"),
        }
    }
}
