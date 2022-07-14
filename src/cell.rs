use crate::{atom::Atom, noun::Noun, Rc};
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, Eq, Hash)]
pub struct Cell {
    head: Rc<Noun>,
    tail: Rc<Noun>,
}

impl Cell {
    pub fn head(&self) -> Rc<Noun> {
        self.head.clone()
    }

    pub fn tail(&self) -> Rc<Noun> {
        self.tail.clone()
    }

    /// Unpack a cell of the form `[a1 a2 ... aN]` into a list, returning `None` if the cell is not
    /// large enough to unpack into a list of length `N`.
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
                Noun::Atom(_) => {
                    if i < N - 1 {
                        return None;
                    }
                    nouns.push(noun.clone());
                }
                Noun::Cell(ref cell) => {
                    nouns.push(cell.head());
                    noun = cell.tail();
                }
            }
        }
        // Is copying too expensive?
        Some(nouns.try_into().unwrap())
    }

    /// Convert a cell into its head and tail, consuming the cell.
    pub fn into_parts(self) -> (Rc<Noun>, Rc<Noun>) {
        (self.head, self.tail)
    }

    /// Convert a cell into a noun, consuming the cell.
    pub fn into_noun(self) -> Noun {
        Noun::Cell(self)
    }

    /// Convert a cell into a reference-counted noun pointer, consuming the cell.
    pub fn into_noun_ptr(self) -> Rc<Noun> {
        Rc::new(Noun::Cell(self))
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[{} {}]", self.head(), self.tail())
    }
}

impl From<(Rc<Noun>, Rc<Noun>)> for Cell {
    fn from((head, tail): (Rc<Noun>, Rc<Noun>)) -> Self {
        Self { head, tail }
    }
}

macro_rules! array_to_cell {
    ($list:expr) => {{
        debug_assert!($list.len() >= 2);
        let (mut remaining, pair) = $list.split_at($list.len() - 2);
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

macro_rules! cell_from_array {
    ([Atom; $len:expr]) => {
        impl From<[Atom; $len]> for Cell {
            fn from(atoms: [Atom; $len]) -> Self {
                let atoms = atoms.map(|a| Rc::new(a.into_noun()));
                array_to_cell!(atoms)
            }
        }
    };
    ([Cell; $len:expr]) => {
        impl From<[Cell; $len]> for Cell {
            fn from(cells: [Self; $len]) -> Self {
                let cells = cells.map(|c| Rc::new(c.into_noun()));
                array_to_cell!(cells)
            }
        }
    };
    ([Noun; $len:expr]) => {
        impl From<[Noun; $len]> for Cell {
            fn from(nouns: [Noun; $len]) -> Self {
                let nouns = nouns.map(|n| Rc::new(n));
                array_to_cell!(nouns)
            }
        }
    };
    ([Rc<Noun>; $len:expr]) => {
        impl From<[Rc<Noun>; $len]> for Cell {
            fn from(nouns: [Rc<Noun>; $len]) -> Self {
                array_to_cell!(nouns)
            }
        }
    };
    ([$atom_src:ty; $len:expr]) => {
        impl From<[$atom_src; $len]> for Cell {
            fn from(atom_srcs: [$atom_src; $len]) -> Self {
                let atom_srcs = atom_srcs.map(|a| Rc::new(Atom::from(a).into_noun()));
                array_to_cell!(atom_srcs)
            }
        }
    };
}

// Create a cell of the form `[a b]`.
cell_from_array!([Atom; 2]);
cell_from_array!([Cell; 2]);
cell_from_array!([Noun; 2]);
cell_from_array!([Rc<Noun>; 2]);
cell_from_array!([&str; 2]);
cell_from_array!([String; 2]);
cell_from_array!([u8; 2]);
cell_from_array!([u16; 2]);
cell_from_array!([u32; 2]);
cell_from_array!([u64; 2]);
cell_from_array!([u128; 2]);
cell_from_array!([usize; 2]);
cell_from_array!([Vec<u8>; 2]);

// Create a cell of the form `[a b c]`.
cell_from_array!([Atom; 3]);
cell_from_array!([Cell; 3]);
cell_from_array!([Noun; 3]);
cell_from_array!([Rc<Noun>; 3]);
cell_from_array!([&str; 3]);
cell_from_array!([String; 3]);
cell_from_array!([u8; 3]);
cell_from_array!([u16; 3]);
cell_from_array!([u32; 3]);
cell_from_array!([u64; 3]);
cell_from_array!([u128; 3]);
cell_from_array!([usize; 3]);
cell_from_array!([Vec<u8>; 3]);

// Create a cell of the form `[a b c d]`.
cell_from_array!([Atom; 4]);
cell_from_array!([Cell; 4]);
cell_from_array!([Noun; 4]);
cell_from_array!([Rc<Noun>; 4]);
cell_from_array!([&str; 4]);
cell_from_array!([String; 4]);
cell_from_array!([u8; 4]);
cell_from_array!([u16; 4]);
cell_from_array!([u32; 4]);
cell_from_array!([u64; 4]);
cell_from_array!([u128; 4]);
cell_from_array!([usize; 4]);
cell_from_array!([Vec<u8>; 4]);

// Create a cell of the form `[a b c d e]`.
cell_from_array!([Atom; 5]);
cell_from_array!([Cell; 5]);
cell_from_array!([Noun; 5]);
cell_from_array!([Rc<Noun>; 5]);
cell_from_array!([&str; 5]);
cell_from_array!([String; 5]);
cell_from_array!([u8; 5]);
cell_from_array!([u16; 5]);
cell_from_array!([u32; 5]);
cell_from_array!([u64; 5]);
cell_from_array!([u128; 5]);
cell_from_array!([usize; 5]);
cell_from_array!([Vec<u8>; 5]);

// Create a cell of the form `[a b c d e f]`.
cell_from_array!([Atom; 6]);
cell_from_array!([Cell; 6]);
cell_from_array!([Noun; 6]);
cell_from_array!([Rc<Noun>; 6]);
cell_from_array!([&str; 6]);
cell_from_array!([String; 6]);
cell_from_array!([u8; 6]);
cell_from_array!([u16; 6]);
cell_from_array!([u32; 6]);
cell_from_array!([u64; 6]);
cell_from_array!([u128; 6]);
cell_from_array!([usize; 6]);
cell_from_array!([Vec<u8>; 6]);

// Create a cell of the form `[a b c d e f g]`.
cell_from_array!([Atom; 7]);
cell_from_array!([Cell; 7]);
cell_from_array!([Noun; 7]);
cell_from_array!([Rc<Noun>; 7]);
cell_from_array!([&str; 7]);
cell_from_array!([String; 7]);
cell_from_array!([u8; 7]);
cell_from_array!([u16; 7]);
cell_from_array!([u32; 7]);
cell_from_array!([u64; 7]);
cell_from_array!([u128; 7]);
cell_from_array!([usize; 7]);
cell_from_array!([Vec<u8>; 7]);

// Create a cell of the form `[a b c d e f g h]`.
cell_from_array!([Atom; 8]);
cell_from_array!([Cell; 8]);
cell_from_array!([Noun; 8]);
cell_from_array!([Rc<Noun>; 8]);
cell_from_array!([&str; 8]);
cell_from_array!([String; 8]);
cell_from_array!([u8; 8]);
cell_from_array!([u16; 8]);
cell_from_array!([u32; 8]);
cell_from_array!([u64; 8]);
cell_from_array!([u128; 8]);
cell_from_array!([usize; 8]);
cell_from_array!([Vec<u8>; 8]);

// Create a cell of the form `[a b c d e f g h i]`.
cell_from_array!([Atom; 9]);
cell_from_array!([Cell; 9]);
cell_from_array!([Noun; 9]);
cell_from_array!([Rc<Noun>; 9]);
cell_from_array!([&str; 9]);
cell_from_array!([String; 9]);
cell_from_array!([u8; 9]);
cell_from_array!([u16; 9]);
cell_from_array!([u32; 9]);
cell_from_array!([u64; 9]);
cell_from_array!([u128; 9]);
cell_from_array!([usize; 9]);
cell_from_array!([Vec<u8>; 9]);

// Create a cell of the form `[a b c d e f g h i j]`.
cell_from_array!([Atom; 10]);
cell_from_array!([Cell; 10]);
cell_from_array!([Noun; 10]);
cell_from_array!([Rc<Noun>; 10]);
cell_from_array!([&str; 10]);
cell_from_array!([String; 10]);
cell_from_array!([u8; 10]);
cell_from_array!([u16; 10]);
cell_from_array!([u32; 10]);
cell_from_array!([u64; 10]);
cell_from_array!([u128; 10]);
cell_from_array!([usize; 10]);
cell_from_array!([Vec<u8>; 10]);

// Create a cell of the form `[a b c d e f g h i j k]`.
cell_from_array!([Atom; 11]);
cell_from_array!([Cell; 11]);
cell_from_array!([Noun; 11]);
cell_from_array!([Rc<Noun>; 11]);
cell_from_array!([&str; 11]);
cell_from_array!([String; 11]);
cell_from_array!([u8; 11]);
cell_from_array!([u16; 11]);
cell_from_array!([u32; 11]);
cell_from_array!([u64; 11]);
cell_from_array!([u128; 11]);
cell_from_array!([usize; 11]);
cell_from_array!([Vec<u8>; 11]);

// Create a cell of the form `[a b c d e f g h i j k l]`.
cell_from_array!([Atom; 12]);
cell_from_array!([Cell; 12]);
cell_from_array!([Noun; 12]);
cell_from_array!([Rc<Noun>; 12]);
cell_from_array!([&str; 12]);
cell_from_array!([String; 12]);
cell_from_array!([u8; 12]);
cell_from_array!([u16; 12]);
cell_from_array!([u32; 12]);
cell_from_array!([u64; 12]);
cell_from_array!([u128; 12]);
cell_from_array!([usize; 12]);
cell_from_array!([Vec<u8>; 12]);

// Create a cell of the form `[a b c d e f g h i j k l m]`.
cell_from_array!([Atom; 13]);
cell_from_array!([Cell; 13]);
cell_from_array!([Noun; 13]);
cell_from_array!([Rc<Noun>; 13]);
cell_from_array!([&str; 13]);
cell_from_array!([String; 13]);
cell_from_array!([u8; 13]);
cell_from_array!([u16; 13]);
cell_from_array!([u32; 13]);
cell_from_array!([u64; 13]);
cell_from_array!([u128; 13]);
cell_from_array!([usize; 13]);
cell_from_array!([Vec<u8>; 13]);

// Create a cell of the form `[a b c d e f g h i j k l m n]`.
cell_from_array!([Atom; 14]);
cell_from_array!([Cell; 14]);
cell_from_array!([Noun; 14]);
cell_from_array!([Rc<Noun>; 14]);
cell_from_array!([&str; 14]);
cell_from_array!([String; 14]);
cell_from_array!([u8; 14]);
cell_from_array!([u16; 14]);
cell_from_array!([u32; 14]);
cell_from_array!([u64; 14]);
cell_from_array!([u128; 14]);
cell_from_array!([usize; 14]);
cell_from_array!([Vec<u8>; 14]);

// Create a cell of the form `[a b c d e f g h i j k l m n o]`.
cell_from_array!([Atom; 15]);
cell_from_array!([Cell; 15]);
cell_from_array!([Noun; 15]);
cell_from_array!([Rc<Noun>; 15]);
cell_from_array!([&str; 15]);
cell_from_array!([String; 15]);
cell_from_array!([u8; 15]);
cell_from_array!([u16; 15]);
cell_from_array!([u32; 15]);
cell_from_array!([u64; 15]);
cell_from_array!([u128; 15]);
cell_from_array!([usize; 15]);
cell_from_array!([Vec<u8>; 15]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p]`.
cell_from_array!([Atom; 16]);
cell_from_array!([Cell; 16]);
cell_from_array!([Noun; 16]);
cell_from_array!([Rc<Noun>; 16]);
cell_from_array!([&str; 16]);
cell_from_array!([String; 16]);
cell_from_array!([u8; 16]);
cell_from_array!([u16; 16]);
cell_from_array!([u32; 16]);
cell_from_array!([u64; 16]);
cell_from_array!([u128; 16]);
cell_from_array!([usize; 16]);
cell_from_array!([Vec<u8>; 16]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q]`.
cell_from_array!([Atom; 17]);
cell_from_array!([Cell; 17]);
cell_from_array!([Noun; 17]);
cell_from_array!([Rc<Noun>; 17]);
cell_from_array!([&str; 17]);
cell_from_array!([String; 17]);
cell_from_array!([u8; 17]);
cell_from_array!([u16; 17]);
cell_from_array!([u32; 17]);
cell_from_array!([u64; 17]);
cell_from_array!([u128; 17]);
cell_from_array!([usize; 17]);
cell_from_array!([Vec<u8>; 17]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q r]`.
cell_from_array!([Atom; 18]);
cell_from_array!([Cell; 18]);
cell_from_array!([Noun; 18]);
cell_from_array!([Rc<Noun>; 18]);
cell_from_array!([&str; 18]);
cell_from_array!([String; 18]);
cell_from_array!([u8; 18]);
cell_from_array!([u16; 18]);
cell_from_array!([u32; 18]);
cell_from_array!([u64; 18]);
cell_from_array!([u128; 18]);
cell_from_array!([usize; 18]);
cell_from_array!([Vec<u8>; 18]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q r s]`.
cell_from_array!([Atom; 19]);
cell_from_array!([Cell; 19]);
cell_from_array!([Noun; 19]);
cell_from_array!([Rc<Noun>; 19]);
cell_from_array!([&str; 19]);
cell_from_array!([String; 19]);
cell_from_array!([u8; 19]);
cell_from_array!([u16; 19]);
cell_from_array!([u32; 19]);
cell_from_array!([u64; 19]);
cell_from_array!([u128; 19]);
cell_from_array!([usize; 19]);
cell_from_array!([Vec<u8>; 19]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q r s t]`.
cell_from_array!([Atom; 20]);
cell_from_array!([Cell; 20]);
cell_from_array!([Noun; 20]);
cell_from_array!([Rc<Noun>; 20]);
cell_from_array!([&str; 20]);
cell_from_array!([String; 20]);
cell_from_array!([u8; 20]);
cell_from_array!([u16; 20]);
cell_from_array!([u32; 20]);
cell_from_array!([u64; 20]);
cell_from_array!([u128; 20]);
cell_from_array!([usize; 20]);
cell_from_array!([Vec<u8>; 20]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q r s t u]`.
cell_from_array!([Atom; 21]);
cell_from_array!([Cell; 21]);
cell_from_array!([Noun; 21]);
cell_from_array!([Rc<Noun>; 21]);
cell_from_array!([&str; 21]);
cell_from_array!([String; 21]);
cell_from_array!([u8; 21]);
cell_from_array!([u16; 21]);
cell_from_array!([u32; 21]);
cell_from_array!([u64; 21]);
cell_from_array!([u128; 21]);
cell_from_array!([usize; 21]);
cell_from_array!([Vec<u8>; 21]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q r s t u v]`.
cell_from_array!([Atom; 22]);
cell_from_array!([Cell; 22]);
cell_from_array!([Noun; 22]);
cell_from_array!([Rc<Noun>; 22]);
cell_from_array!([&str; 22]);
cell_from_array!([String; 22]);
cell_from_array!([u8; 22]);
cell_from_array!([u16; 22]);
cell_from_array!([u32; 22]);
cell_from_array!([u64; 22]);
cell_from_array!([u128; 22]);
cell_from_array!([usize; 22]);
cell_from_array!([Vec<u8>; 22]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q r s t u v]`.
cell_from_array!([Atom; 23]);
cell_from_array!([Cell; 23]);
cell_from_array!([Noun; 23]);
cell_from_array!([Rc<Noun>; 23]);
cell_from_array!([&str; 23]);
cell_from_array!([String; 23]);
cell_from_array!([u8; 23]);
cell_from_array!([u16; 23]);
cell_from_array!([u32; 23]);
cell_from_array!([u64; 23]);
cell_from_array!([u128; 23]);
cell_from_array!([usize; 23]);
cell_from_array!([Vec<u8>; 23]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q r s t u v w]`.
cell_from_array!([Atom; 24]);
cell_from_array!([Cell; 24]);
cell_from_array!([Noun; 24]);
cell_from_array!([Rc<Noun>; 24]);
cell_from_array!([&str; 24]);
cell_from_array!([String; 24]);
cell_from_array!([u8; 24]);
cell_from_array!([u16; 24]);
cell_from_array!([u32; 24]);
cell_from_array!([u64; 24]);
cell_from_array!([u128; 24]);
cell_from_array!([usize; 24]);
cell_from_array!([Vec<u8>; 24]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q r s t u v w x]`.
cell_from_array!([Atom; 25]);
cell_from_array!([Cell; 25]);
cell_from_array!([Noun; 25]);
cell_from_array!([Rc<Noun>; 25]);
cell_from_array!([&str; 25]);
cell_from_array!([String; 25]);
cell_from_array!([u8; 25]);
cell_from_array!([u16; 25]);
cell_from_array!([u32; 25]);
cell_from_array!([u64; 25]);
cell_from_array!([u128; 25]);
cell_from_array!([usize; 25]);
cell_from_array!([Vec<u8>; 25]);

// Create a cell of the form `[a b c d e f g h i j k l m n o p q r s t u v w x y z]`.
cell_from_array!([Atom; 26]);
cell_from_array!([Cell; 26]);
cell_from_array!([Noun; 26]);
cell_from_array!([Rc<Noun>; 26]);
cell_from_array!([&str; 26]);
cell_from_array!([String; 26]);
cell_from_array!([u8; 26]);
cell_from_array!([u16; 26]);
cell_from_array!([u32; 26]);
cell_from_array!([u64; 26]);
cell_from_array!([u128; 26]);
cell_from_array!([usize; 26]);
cell_from_array!([Vec<u8>; 26]);

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}
