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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{atom, cell, noun::Noun};

    #[test]
    fn convert() {
        impl TryFrom<&Noun> for String {
            type Error = Error;

            fn try_from(noun: &Noun) -> Result<Self, Self::Error> {
                if let Noun::Atom(noun) = noun {
                    if let Ok(noun) = noun.as_str() {
                        Ok(Self::from(noun))
                    } else {
                        Err(Error::AtomToStr)
                    }
                } else {
                    Err(Error::UnexpectedCell)
                }
            }
        }

        // Noun -> Vec<String>: expect success.
        {
            {
                let noun = Noun::from(atom!());
                let vec = convert!(&noun => Vec<String>).expect("Noun to Vec<String>");
                assert!(vec.is_empty());
            }

            {
                let noun = Noun::from(cell![atom!("hello"), atom!("world"), atom!()]);
                let vec = convert!(&noun => Vec<String>).expect("Noun to Vec<String>");
                assert_eq!(vec.len(), 2);
                assert_eq!(vec[0], "hello");
                assert_eq!(vec[1], "world");
            }
        }

        // Noun -> Vec<String>: expect failure.
        {
            {
                let noun = Noun::from(cell!["no", "null", "terminator"]);
                assert!(convert!(&noun => Vec<String>).is_err());
            }

            {
                let noun = Noun::from(cell![
                    Noun::from(cell!["unexpected", "cell"]),
                    Noun::from(atom!())
                ]);
                assert!(convert!(&noun => Vec<String>).is_err());
            }
        }
    }
}
