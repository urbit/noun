//! Pairs of nouns.
//!
//! A cell is a pair of reference-counted nouns. A cell can be:
//! - created from an array of atoms, cells, nouns, or types that can easily be converted into
//!   atoms;
//! - compared to other cells;
//! - unpacked into an array of nouns;
//! - pretty-printed;
//! - converted into a noun.

use crate::{atom::Atom, noun::Noun, Rc};
use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Display, Error, Formatter},
    hash::{Hash, Hasher},
    mem::MaybeUninit,
};

/// A pair of reference-counted nouns.
///
/// To create a new cell, use one of the `From<[T; N]>` implementations. For example:
/// ```
/// # use noun::{atom::Atom, cell::Cell, Noun, cell};
/// let cell = Cell::from(["hello", "world"]);
/// assert_eq!(*cell.head(), Noun::from(Atom::from("hello")));
/// assert_eq!(*cell.tail(), Noun::from(Atom::from("world")));
/// ```
///
/// ```
/// # use noun::{atom::Atom, cell::Cell, Noun, cell};
/// let cell = Cell::from([0u8, 2u8, 4u8, 8u8]);
/// assert_eq!(*cell.head(), Noun::from(Atom::from(0u8)));
/// assert_eq!(*cell.tail(), Noun::from(Cell::from([2u8, 4u8, 8u8])));
/// ```
#[derive(Clone, Debug, Eq, Hash)]
pub struct Cell {
    head: Rc<Noun>,
    tail: Rc<Noun>,
}

impl Cell {
    /// Constructs a new cell.
    fn new(head: Rc<Noun>, tail: Rc<Noun>) -> Self {
        Self { head, tail }
    }

    /// Returns the head of this cell.
    pub fn head(&self) -> Rc<Noun> {
        self.head.clone()
    }

    /// Returns the head of this cell as a reference.
    pub fn head_ref(&self) -> &Noun {
        &self.head
    }

    /// Returns the tail of this cell.
    pub fn tail(&self) -> Rc<Noun> {
        self.tail.clone()
    }

    /// Returns the tail of this cell as a reference.
    pub fn tail_ref(&self) -> &Noun {
        &self.tail
    }

    /// Computes the hash of this cell.
    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64((&*self.head()).hash());
        hasher.write_u64((&*self.tail()).hash());
        hasher.finish()
    }

    /// Unpacks this cell into an array of length `N`, returning `None` if the cell is not of the
    /// form `[a1 a2 ... aN]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use noun::{atom::Atom, cell::Cell, Noun, cell};
    /// let cell = Cell::from([0u8, 1u8, 2u8, 3u8, 4u8, 5u8]);
    ///
    /// let nouns = cell.to_array::<6>().unwrap();
    /// assert_eq!(*nouns[0], Noun::from(Atom::from(0u8)));
    /// assert_eq!(*nouns[1], Noun::from(Atom::from(1u8)));
    /// assert_eq!(*nouns[2], Noun::from(Atom::from(2u8)));
    /// assert_eq!(*nouns[3], Noun::from(Atom::from(3u8)));
    /// assert_eq!(*nouns[4], Noun::from(Atom::from(4u8)));
    /// assert_eq!(*nouns[5], Noun::from(Atom::from(5u8)));
    /// ```
    ///
    /// ```
    /// # use noun::{atom::Atom, cell::Cell, cell};
    /// let cell = Cell::from([0u8, 1u8, 2u8, 3u8]);
    ///
    /// assert_eq!(cell.to_array::<6>(), None);
    /// ```
    pub fn to_array<const N: usize>(&self) -> Option<[Rc<Noun>; N]> {
        debug_assert!(N >= 2);
        // See https://doc.rust-lang.org/nomicon/unchecked-uninit.html.
        let mut nouns: [MaybeUninit<Rc<Noun>>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        nouns[0] = MaybeUninit::new(self.head());
        let mut noun = self.tail();
        for i in 1..N {
            match *noun {
                Noun::Atom(_) if i < N - 1 => return None,
                Noun::Cell(ref cell) if i < N - 1 => {
                    nouns[i] = MaybeUninit::new(cell.head());
                    noun = cell.tail();
                }
                _ => nouns[i] = MaybeUninit::new(noun.clone()),
            }
        }
        // Using `mem::transmute()` here as suggested in the Rustnomicon example linked above results in
        // compiler error E0512.
        let nouns = unsafe { nouns.as_ptr().cast::<[Rc<Noun>; N]>().read() };
        Some(nouns)
    }

    /// Unpacks this cell into a vector.
    ///
    /// If the length of the cell is known at compile-time, use [`Self::to_array()`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use noun::{atom::Atom, cell::Cell, Noun, cell};
    /// let cell = Cell::from([0u8, 1u8, 2u8, 4u8, 8u8, 16u8, 32u8, 64u8, 128u8]);
    ///
    /// let nouns = cell.to_vec();
    /// assert_eq!(nouns.len(), 9);
    /// assert_eq!(*nouns[0], Noun::from(Atom::from(0u8)));
    /// assert_eq!(*nouns[1], Noun::from(Atom::from(1u8)));
    /// assert_eq!(*nouns[2], Noun::from(Atom::from(2u8)));
    /// assert_eq!(*nouns[3], Noun::from(Atom::from(4u8)));
    /// assert_eq!(*nouns[4], Noun::from(Atom::from(8u8)));
    /// assert_eq!(*nouns[5], Noun::from(Atom::from(16u8)));
    /// assert_eq!(*nouns[6], Noun::from(Atom::from(32u8)));
    /// assert_eq!(*nouns[7], Noun::from(Atom::from(64u8)));
    /// assert_eq!(*nouns[8], Noun::from(Atom::from(128u8)));
    ///
    /// ```
    pub fn to_vec(&self) -> Vec<Rc<Noun>> {
        let mut nouns = Vec::new();
        nouns.push(self.head());
        let mut noun = self.tail();
        while let Noun::Cell(cell) = &*noun {
            nouns.push(cell.head());
            noun = cell.tail();
        }
        nouns.push(noun);
        nouns
    }

    /// Converts this cell into its head and tail, consuming the cell.
    pub fn into_parts(self) -> (Rc<Noun>, Rc<Noun>) {
        (self.head, self.tail)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // This is unfortunately more complicated than
        // `write!(f, "[{} {}]", self.head(), self.tail())` to handle the fact that brackets are
        // left-associative and therefore need not always be printed.
        write!(f, "[")?;
        match (&*self.head(), &*self.tail()) {
            (head, Noun::Atom(tail)) => write!(f, "{} {}", head, tail)?,
            (head, _) => {
                write!(f, "{} ", head)?;
                let mut tail = self.tail();
                while let Noun::Cell(cell) = &*tail {
                    write!(f, "{} ", cell.head())?;
                    tail = cell.tail();
                }
                write!(f, "{}", tail)?;
            }
        }
        write!(f, "]")
    }
}

/// Create a cell of the form `[a1 a2 ... aN]` from an `N`-element array of [`Rc<Noun>`].
macro_rules! cell_from_array {
    ($array:expr) => {{
        debug_assert!($array.len() >= 2);
        let (mut remaining, pair) = $array.split_at($array.len() - 2);
        let mut cell = {
            let head: &Rc<Noun> = &pair[0];
            let tail: &Rc<Noun> = &pair[1];
            Cell::new(head.clone(), tail.clone())
        };
        while !remaining.is_empty() {
            let split = remaining.split_at(remaining.len() - 1);
            remaining = split.0;
            let single = split.1;
            cell = Cell::new(single[0].clone(), Rc::new(Noun::from(cell)));
        }
        cell
    }};
}

macro_rules! impl_from_array_for_cell {
    (n = $n:expr) => {
        impl_from_array_for_cell!([Atom; $n]);
        impl_from_array_for_cell!([Cell; $n]);
        impl_from_array_for_cell!([Noun; $n]);
        impl_from_array_for_cell!([Rc<Noun>; $n]);
        impl_from_array_for_cell!([&str; $n]);
        impl_from_array_for_cell!([String; $n]);
        impl_from_array_for_cell!([u8; $n]);
        impl_from_array_for_cell!([u16; $n]);
        impl_from_array_for_cell!([u32; $n]);
        impl_from_array_for_cell!([u64; $n]);
        impl_from_array_for_cell!([u128; $n]);
        impl_from_array_for_cell!([usize; $n]);
        impl_from_array_for_cell!([Vec<u8>; $n]);
    };
    ([Atom; $len:expr]) => {
        impl From<[Atom; $len]> for Cell {
            fn from(atoms: [Atom; $len]) -> Self {
                let atoms = atoms.map(|a| Rc::new(Noun::from(a)));
                cell_from_array!(atoms)
            }
        }
    };
    ([Cell; $len:expr]) => {
        impl From<[Cell; $len]> for Cell {
            fn from(cells: [Self; $len]) -> Self {
                let cells = cells.map(|c| Rc::new(Noun::from(c)));
                cell_from_array!(cells)
            }
        }
    };
    ([Noun; $len:expr]) => {
        impl From<[Noun; $len]> for Cell {
            fn from(nouns: [Noun; $len]) -> Self {
                let nouns = nouns.map(|n| Rc::new(n));
                cell_from_array!(nouns)
            }
        }
    };
    ([Rc<Noun>; $len:expr]) => {
        impl From<[Rc<Noun>; $len]> for Cell {
            fn from(nouns: [Rc<Noun>; $len]) -> Self {
                cell_from_array!(nouns)
            }
        }
    };
    ([$atom_src:ty; $len:expr]) => {
        impl From<[$atom_src; $len]> for Cell {
            fn from(atom_srcs: [$atom_src; $len]) -> Self {
                let atom_srcs = atom_srcs.map(|a| Rc::new(Noun::from(Atom::from(a))));
                cell_from_array!(atom_srcs)
            }
        }
    };
}

impl_from_array_for_cell!(n = 2);
impl_from_array_for_cell!(n = 3);
impl_from_array_for_cell!(n = 4);
impl_from_array_for_cell!(n = 5);
impl_from_array_for_cell!(n = 6);
impl_from_array_for_cell!(n = 7);
impl_from_array_for_cell!(n = 8);
impl_from_array_for_cell!(n = 9);
impl_from_array_for_cell!(n = 10);
impl_from_array_for_cell!(n = 11);
impl_from_array_for_cell!(n = 12);
impl_from_array_for_cell!(n = 13);
impl_from_array_for_cell!(n = 14);
impl_from_array_for_cell!(n = 15);
impl_from_array_for_cell!(n = 16);
impl_from_array_for_cell!(n = 17);
impl_from_array_for_cell!(n = 18);
impl_from_array_for_cell!(n = 19);
impl_from_array_for_cell!(n = 20);
impl_from_array_for_cell!(n = 21);
impl_from_array_for_cell!(n = 22);
impl_from_array_for_cell!(n = 23);
impl_from_array_for_cell!(n = 24);
impl_from_array_for_cell!(n = 25);
impl_from_array_for_cell!(n = 26);
impl_from_array_for_cell!(n = 27);
impl_from_array_for_cell!(n = 28);
impl_from_array_for_cell!(n = 29);
impl_from_array_for_cell!(n = 30);

impl From<Vec<Rc<Noun>>> for Cell {
    fn from(nouns: Vec<Rc<Noun>>) -> Self {
        cell_from_array!(nouns)
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}

/// Creates a new [`Cell`].
///
/// This is syntactic sugar for `Cell::from()`.
///
/// - Create a [`Cell`] from a single expression of type `T`. [`Cell`] must implement [`From<T>`].
/// ```
/// # use noun::{atom::Atom, cell::Cell, Noun, Rc};
/// let cell = Cell::from(vec![
///     Rc::<Noun>::from(Atom::from(0u8)),
///     Rc::<Noun>::from(Atom::from(1u8)),
///     Rc::<Noun>::from(Atom::from(2u8)),
///     Rc::<Noun>::from(Atom::from(3u8)),
/// ]);
/// ```
///
/// - Create a [`Cell`] from a sequence of expressions. Each expression must be of the same type
///   `T`, and [`Cell`] must implement [`From<[T; N]>`], where `N` is the number of expressions.
/// ```
/// # use noun::cell::Cell;
/// let cell = Cell::from([0u8, 1u8, 2u8, 3u8]);
/// ```
#[macro_export]
macro_rules! cell {
    ($x:expr) => {
        $crate::cell::Cell::from($x)
    };
    ($($x:expr),+ $(,)?) => {
        $crate::cell::Cell::from([$($x,)+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_array() {
        {
            let cell = Cell::from([
                Noun::from(Atom::from("request")),
                Noun::from(Atom::from(0u8)),
                Noun::from(Atom::from("POST")),
                Noun::from(Atom::from("http://eth-mainnet.urbit.org:8545")),
                Noun::from(Cell::from([
                    Noun::from(Cell::from([
                        Atom::from("Content-Type"),
                        Atom::from("application/json"),
                    ])),
                    Noun::from(Atom::from(0u8)),
                ])),
                Noun::from(Atom::from(0u8)),
                Noun::from(Atom::from(78u8)),
                Noun::from(Atom::from(
                    r#"[{"params":[],"id":"block number","jsonrpc":"2.0","method":"eth_blockNumber"}]"#,
                )),
            ]);
            let [tag, req_num, method, uri, headers, body] = cell.to_array::<6>().expect("as list");
            if let (Noun::Atom(tag), Noun::Atom(req_num), Noun::Atom(method), Noun::Atom(uri)) =
                (&*tag, &*req_num, &*method, &*uri)
            {
                assert_eq!(tag, "request");
                assert_eq!(*req_num, 0u8);
                assert_eq!(method, "POST");
                assert_eq!(uri, "http://eth-mainnet.urbit.org:8545");
            } else {
                panic!("unexpected cell");
            }
            if let Noun::Cell(headers) = &*headers {
                if let Noun::Cell(header) = &*headers.head() {
                    if let (Noun::Atom(key), Noun::Atom(val)) = (&*header.head(), &*header.tail()) {
                        assert_eq!(key, "Content-Type");
                        assert_eq!(val, "application/json");
                    } else {
                        panic!("unexpected cell");
                    }
                } else {
                    panic!("unexpected atom");
                }
                if let Noun::Atom(null) = &*headers.tail() {
                    assert_eq!(*null, 0u8);
                } else {
                    panic!("unexpected cell");
                }
            } else {
                panic!("unexpected atom");
            }
            if let Noun::Cell(body) = &*body {
                if let Noun::Atom(null) = &*body.head() {
                    assert_eq!(*null, 0u8);
                } else {
                    panic!("unexpected cell");
                }
                if let Noun::Cell(body) = &*body.tail() {
                    if let (Noun::Atom(body_len), Noun::Atom(body)) = (&*body.head(), &*body.tail())
                    {
                        assert_eq!(*body_len, 78u8);
                        assert_eq!(
                            body,
                            r#"[{"params":[],"id":"block number","jsonrpc":"2.0","method":"eth_blockNumber"}]"#
                        );
                    } else {
                        panic!("unexpected cell");
                    }
                } else {
                    panic!("unexpected atom");
                }
            } else {
                panic!("unexpected atom");
            }
        }
    }

    #[test]
    fn from_vec() {
        {
            let _0 = Rc::<Noun>::from(Atom::from(0u8));
            let _2 = Rc::<Noun>::from(Atom::from(2u8));
            let _8 = Rc::<Noun>::from(Atom::from(8u8));
            let _32 = Rc::<Noun>::from(Atom::from(32u8));
            let _128 = Rc::<Noun>::from(Atom::from(128u8));
            let cell = Cell::from(vec![
                _0.clone(),
                _2.clone(),
                _8.clone(),
                _32.clone(),
                _128.clone(),
            ]);

            let [a, b, c, d, e] = cell.to_array::<5>().expect("cell to array");
            assert_eq!(a, _0);
            assert_eq!(b, _2);
            assert_eq!(c, _8);
            assert_eq!(d, _32);
            assert_eq!(e, _128);
        }
    }
}
