//! Cells.

use crate::{atom::Atom, noun::Noun, Rc};
use std::fmt::{Display, Error, Formatter};

/// A pair of reference-counted nouns.
#[derive(Clone, Debug, Eq, Hash)]
pub struct Cell {
    head: Rc<Noun>,
    tail: Rc<Noun>,
}

impl Cell {
    /// Returns the head of this noun.
    pub fn head(&self) -> Rc<Noun> {
        self.head.clone()
    }

    /// Returns the tail of this noun.
    pub fn tail(&self) -> Rc<Noun> {
        self.tail.clone()
    }

    /// Unpacks this cell into an array of length `N`, returning `None` if the cell is not of the
    /// form `[a1 a2 ... aN]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use noun::{atom::Atom, cell::Cell};
    /// let cell = Cell::from([0u8, 1u8, 2u8, 3u8, 4u8, 5u8]);
    ///
    /// let nouns = cell.as_list::<6>().unwrap();
    /// assert_eq!(*nouns[0], Atom::from(0u8).into_noun());
    /// assert_eq!(*nouns[1], Atom::from(1u8).into_noun());
    /// assert_eq!(*nouns[2], Atom::from(2u8).into_noun());
    /// assert_eq!(*nouns[3], Atom::from(3u8).into_noun());
    /// assert_eq!(*nouns[4], Atom::from(4u8).into_noun());
    /// assert_eq!(*nouns[5], Atom::from(5u8).into_noun());
    /// ```
    ///
    /// ```
    /// # use noun::{atom::Atom, cell::Cell};
    /// let cell = Cell::from([0u8, 1u8, 2u8, 3u8]);
    ///
    /// assert_eq!(cell.as_list::<6>(), None);
    /// ```
    pub fn as_list<const N: usize>(&self) -> Option<[Rc<Noun>; N]> {
        debug_assert!(N >= 2);
        let mut nouns = Vec::with_capacity(N);
        nouns.push(self.head());
        let mut noun = self.tail();
        for i in 1..N {
            match *noun {
                Noun::Atom(_) if i < N - 1 => return None,
                Noun::Cell(ref cell) if i < N - 1 => {
                    nouns.push(cell.head());
                    noun = cell.tail();
                }
                _ => nouns.push(noun.clone()),
            }
        }
        // Is copying too expensive?
        Some(nouns.try_into().unwrap())
    }

    /// Converts this cell into its head and tail, consuming the cell.
    pub fn into_parts(self) -> (Rc<Noun>, Rc<Noun>) {
        (self.head, self.tail)
    }

    /// Converts this cell into a noun, consuming the cell.
    pub fn into_noun(self) -> Noun {
        Noun::Cell(self)
    }

    /// Converts this cell into a reference-counted noun, consuming the cell.
    pub fn into_rc_noun(self) -> Rc<Noun> {
        Rc::new(Noun::Cell(self))
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
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

impl From<(Rc<Noun>, Rc<Noun>)> for Cell {
    fn from((head, tail): (Rc<Noun>, Rc<Noun>)) -> Self {
        Self { head, tail }
    }
}

/// Create a cell of the form `[a1 a2 ... aN]` from an `N`-element array.
macro_rules! cell_from_array {
    (n=$n:expr) => {
        cell_from_array!([Atom; $n]);
        cell_from_array!([Cell; $n]);
        cell_from_array!([Noun; $n]);
        cell_from_array!([Rc<Noun>; $n]);
        cell_from_array!([&str; $n]);
        cell_from_array!([String; $n]);
        cell_from_array!([u8; $n]);
        cell_from_array!([u16; $n]);
        cell_from_array!([u32; $n]);
        cell_from_array!([u64; $n]);
        cell_from_array!([u128; $n]);
        cell_from_array!([usize; $n]);
        cell_from_array!([Vec<u8>; $n]);
    };
    ([Atom; $len:expr]) => {
        impl From<[Atom; $len]> for Cell {
            fn from(atoms: [Atom; $len]) -> Self {
                let atoms = atoms.map(|a| Rc::new(a.into_noun()));
                cell_from_array!(atoms)
            }
        }
    };
    ([Cell; $len:expr]) => {
        impl From<[Cell; $len]> for Cell {
            fn from(cells: [Self; $len]) -> Self {
                let cells = cells.map(|c| Rc::new(c.into_noun()));
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
                let atom_srcs = atom_srcs.map(|a| Rc::new(Atom::from(a).into_noun()));
                cell_from_array!(atom_srcs)
            }
        }
    };
    ($array:expr) => {{
        debug_assert!($array.len() >= 2);
        let (mut remaining, pair) = $array.split_at($array.len() - 2);
        let mut cell = Cell::from((pair[0].clone(), pair[1].clone()));
        while !remaining.is_empty() {
            let split = remaining.split_at(remaining.len() - 1);
            remaining = split.0;
            let single = split.1;
            cell = Cell::from([single[0].clone(), Rc::new(cell.into_noun())]);
        }
        cell
    }};
}

cell_from_array!(n=2);
cell_from_array!(n=3);
cell_from_array!(n=4);
cell_from_array!(n=5);
cell_from_array!(n=6);
cell_from_array!(n=7);
cell_from_array!(n=8);
cell_from_array!(n=9);
cell_from_array!(n=10);
cell_from_array!(n=11);
cell_from_array!(n=12);
cell_from_array!(n=13);
cell_from_array!(n=14);
cell_from_array!(n=15);
cell_from_array!(n=16);
cell_from_array!(n=17);
cell_from_array!(n=18);
cell_from_array!(n=19);
cell_from_array!(n=20);
cell_from_array!(n=21);
cell_from_array!(n=22);
cell_from_array!(n=23);
cell_from_array!(n=24);
cell_from_array!(n=25);
cell_from_array!(n=26);
cell_from_array!(n=27);
cell_from_array!(n=28);
cell_from_array!(n=29);
cell_from_array!(n=30);

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_list() {
        {
            let cell = Cell::from([
                Atom::from("request").into_noun(),
                Atom::from(0u8).into_noun(),
                Atom::from("POST").into_noun(),
                Atom::from("http://eth-mainnet.urbit.org:8545").into_noun(),
                Cell::from([
                    Cell::from([Atom::from("Content-Type"), Atom::from("application/json")]).into_noun(),
                    Atom::from(0u8).into_noun(),
                ]).into_noun(),
                Atom::from(0u8).into_noun(),
                Atom::from(78u8).into_noun(),
                Atom::from(r#"[{"params":[],"id":"block number","jsonrpc":"2.0","method":"eth_blockNumber"}]"#).into_noun(),
            ]);
            let [tag, req_num, method, uri, headers, body] = cell.as_list::<6>().expect("as list");
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
}
