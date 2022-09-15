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

/// Converts a `&Noun` of the form `[a0 a1 ... aN 0]` (i.e. a null-terminated list) to a
/// [`Vec`] of `$elem_type`, returning a [`convert::Error`] on error.
///
/// `$elem_type` must implement [`TryFrom<&Noun>`].
///
/// The resulting vector does not include the null terminator.
#[macro_export]
macro_rules! convert {
    ($noun:expr => Vec<$elem_type:ty>) => {{
        use $crate::{convert::Error, noun::Noun};
        match $noun {
            Noun::Atom(atom) => {
                if atom.is_null() {
                    Ok(Vec::new())
                } else {
                    Err(Error::UnexpectedAtom)
                }
            }
            mut noun => {
                let mut elems = Vec::new();
                loop {
                    match noun {
                        Noun::Atom(atom) => {
                            if atom.is_null() {
                                break Ok(elems);
                            } else {
                                break Err(Error::ExpectedNull);
                            }
                        }
                        Noun::Cell(cell) => match <$elem_type>::try_from(cell.head_ref()) {
                            Ok(elem) => {
                                elems.push(elem);
                                noun = cell.tail_ref();
                            }
                            Err(err) => break Err(err),
                        },
                    }
                }
            }
        }
    }};
}
