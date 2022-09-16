//! Conversions to and from [`Noun`](crate::noun::Noun).

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

/// Converts [`Noun`](crate::Noun)s to and from other complex types.
///
/// There are three forms of this macro:
///
/// - Convert a [`&Noun`] of the form `[e0 e1 ... eN 0]` (a null-terminated list) to a
///   [`Vec`]`<$elem_type>`, returning [`Result`]`<`[`Vec`]`<$elem_type>, `[`Error`]`>`.
///
///   `$elem_type` must implement [`TryFrom`]`<`[`&Noun`]`>`.
///
///   The resulting [`Vec`] does not include the null terminator.
///
/// ```
/// # use noun::{convert, noun::Noun};
/// let noun = Noun::null();
/// let vec = convert!(&noun => Vec<String>).unwrap();
/// assert!(vec.is_empty());
/// ```
///
/// ```
/// # use noun::{atom::Atom, cell::Cell, convert, noun::Noun};
/// let noun = Noun::from(Cell::from([
///     Atom::from("hello"),
///     Atom::from("world"),
///     Atom::null(),
/// ]));
/// let vec = convert!(&noun => Vec<String>).unwrap();
/// assert_eq!(vec, vec!["hello", "world"]);
/// ```
///
/// - Convert a [`&Noun`] of the form `[[k0 v0] [k1 v1] ... [kN vN] 0]` (a null-terminated map) to a
///   [`HashMap`]`<$key_type, $val_type>`, returning [`Result`]`<`[`HashMap`]`<$key_type, $val_type>,
///   `[`Error`]`>`.
///
///   `$key_type` and `$val_type` must each implement [`TryFrom`]`<`[`&Noun`]`>`.
///
///   The resulting [`HashMap`] does not include the null terminator.
///
/// ```
/// # use noun::{cell::Cell, convert, noun::Noun};
/// let noun = Noun::null();
/// let map = convert!(&noun => HashMap<&str, &str>).unwrap();
/// assert_eq!(map.len(), 0);
/// ```
///
/// ```
/// # use noun::{cell::Cell, convert, noun::Noun};
/// let noun = Noun::from(Cell::from([
///     Noun::from(Cell::from(["Ruth", "Babe"])),
///     Noun::from(Cell::from(["Williams", "Ted"])),
///     Noun::from(Cell::from(["Bonds", "Barry"])),
///     Noun::from(Cell::from(["Pujols", "Albert"])),
///     Noun::null()
/// ]));
/// let map = convert!(&noun => HashMap<&str, &str>).unwrap();
/// assert_eq!(map.len(), 4);
/// assert_eq!(map.get("Ruth"), Some(&"Babe"));
/// assert_eq!(map.get("Williams"), Some(&"Ted"));
/// assert_eq!(map.get("Bonds"), Some(&"Barry"));
/// assert_eq!(map.get("Pujols"), Some(&"Albert"));
/// ```
///
/// - Convert an iterator of the form `[e0, e1, ... eN]` where each element has type `T` into a
///   [`Noun`] of the form `[e0 e1 ... eN 0]` (a null-terminated list), returning
///   [`Result`]`<`[`Noun`]`, <err_type>>`, where `<err_type>` is the type of error returned by
///   `Noun::try_from` when attempting to convert `T` into a [`Noun`].
///
///   [`Noun`] must implement [`TryFrom`]`<T>`.
///
/// ```
/// # use noun::{atom::Atom, cell::Cell, convert, noun::Noun};
/// let strings = [];
/// let noun = convert!(strings.iter() => Noun).unwrap();
/// assert!(noun.is_null());
/// ```
///
/// ```
/// # use noun::{atom::Atom, cell::Cell, convert, noun::Noun};
/// let strings = vec![
///     String::from("1"),
///     String::from("2"),
///     String::from("3"),
///     String::from("4"),
/// ];
/// let noun = convert!(strings.into_iter() => Noun).unwrap();
/// assert_eq!(
///     noun,
///     Noun::from(Cell::from([
///         Atom::from("1"),
///         Atom::from("2"),
///         Atom::from("3"),
///         Atom::from("4"),
///         Atom::null(),
///     ]))
/// );
/// ```
///
/// [`Err(Error)`]: Error
/// [`HashMap`]: std::collections::HashMap
/// [`&Noun`]: crate::Noun
/// [`Noun`]: crate::Noun
#[macro_export]
macro_rules! convert {
    ($noun:expr => Vec<$elem_type:ty>) => {{
        use $crate::{convert::Error, noun::Noun};
        let mut noun = $noun;
        let mut elems: Vec<$elem_type> = Vec::new();
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
    }};
    ($noun:expr => HashMap<$key_type:ty, $val_type:ty>) => {{
        use std::collections::HashMap;
        use $crate::{convert::Error, noun::Noun};
        let mut noun = $noun;
        let mut map: HashMap<$key_type, $val_type> = HashMap::new();
        loop {
            match noun {
                Noun::Atom(atom) => {
                    if atom.is_null() {
                        break Ok(map);
                    } else {
                        break Err(Error::ExpectedNull);
                    }
                }
                Noun::Cell(cell) => {
                    if let Noun::Cell(head) = cell.head_ref() {
                        match (
                            <$key_type>::try_from(head.head_ref()),
                            <$val_type>::try_from(head.tail_ref()),
                        ) {
                            (Ok(key), Ok(val)) => {
                                map.insert(key, val);
                                noun = cell.tail_ref();
                            }
                            (Err(err), _) => break Err(err),
                            (_, Err(err)) => break Err(err),
                        }
                    } else {
                        break Err(Error::UnexpectedAtom);
                    }
                }
            }
        }
    }};
    ($iter:expr => Noun) => {{
        use $crate::{cell::Cell, noun::Noun, Rc};
        let mut noun = Rc::<Noun>::from(Noun::null());
        let mut iter = $iter.rev();
        loop {
            match iter.next() {
                Some(elem) => match Noun::try_from(elem) {
                    Ok(elem) => {
                        noun = Rc::<Noun>::from(Noun::from(Cell::from([
                            Rc::<Noun>::from(elem),
                            noun,
                        ])));
                    }
                    Err(err) => break Err(err),
                },
                None => break Ok(Rc::try_unwrap(noun).unwrap()),
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::{atom::Atom, cell::Cell, noun::Noun};

    #[test]
    fn convert() {
        // Noun -> Vec<String>: expect failure.
        {
            {
                let noun = Noun::from(Cell::from(["no", "null", "terminator"]));
                assert!(convert!(&noun => Vec<String>).is_err());
            }

            {
                let noun = Noun::from(Cell::from([
                    Noun::from(Cell::from(["unexpected", "cell"])),
                    Noun::null(),
                ]));
                assert!(convert!(&noun => Vec<String>).is_err());
            }
        }

        // &[&str] -> Noun: expect success.
        {
            {
                let strings = ["a", "b", "c"];
                let noun = convert!(strings.iter() => Noun).expect("&[str] to Noun");
                assert_eq!(
                    noun,
                    Noun::from(Cell::from([
                        Atom::from("a"),
                        Atom::from("b"),
                        Atom::from("c"),
                        Atom::null()
                    ]))
                );
            }
        }
    }
}
