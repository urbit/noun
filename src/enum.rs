use crate::{
    Atom as _Atom, Cell as _Cell, Cue as _Cue, IntoNoun as _IntoNoun, Jam as _Jam, Noun as _Noun,
};
use bitstream_io::{BitRead, BitWrite};
use std::hash::{Hash, Hasher};

#[derive(Eq, Clone, Debug, Hash)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl _Cue for Noun {
    type Error = ();

    fn cue(_src: impl BitRead) -> Result<Self, <Self as _Cue>::Error> {
        todo!()
    }
}

impl _Jam for Noun {
    type Error = ();

    fn jam(self, _sink: &mut impl BitWrite) -> Result<(), <Self as _Jam>::Error> {
        todo!()
    }
}

impl _Noun for Noun {
    type Atom = Atom;
    type Cell = Cell;
    type Error = ();

    fn get(&self, idx: usize) -> Option<&Noun> {
        if let Self::Cell(cell) = self {
            match idx {
                0 | 1 => Some(self),
                2 => Some(&*cell.head.as_ref()?),
                3 => Some(&*cell.tail.as_ref()?),
                n if n % 2 == 0 => (&*cell.head.as_ref()?).get(idx / 2),
                _ => (&*cell.tail.as_ref()?).get(idx / 2),
            }
        } else {
            None
        }
    }

    fn into_atom(self) -> Result<<Self as _Noun>::Atom, <Self as _Noun>::Error> {
        match self {
            Self::Atom(atom) => Ok(atom),
            _ => Err(()),
        }
    }

    fn into_cell(self) -> Result<<Self as _Noun>::Cell, <Self as _Noun>::Error> {
        match self {
            Self::Cell(cell) => Ok(cell),
            _ => Err(()),
        }
    }
}

impl PartialEq for Noun {
    fn eq(&self, other: &Self) -> bool {
        if let (Self::Atom(this), Self::Atom(that)) = (self, other) {
            this == that
        } else if let (Self::Cell(this), Self::Cell(that)) = (self, other) {
            this == that
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Atom(Vec<u8>);

impl _Atom for Atom {
    type Error = ();

    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl _IntoNoun for Atom {
    type Error = ();
    type Noun = Noun;

    fn into_noun(self) -> Result<Self::Noun, Self::Error> {
        Ok(Noun::Atom(self))
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Cell {
    head: Option<Box<Noun>>,
    tail: Option<Box<Noun>>,
}

impl _Cell for Cell {
    type Noun = Noun;

    fn into_parts(self) -> (Option<<Self as _Cell>::Noun>, Option<<Self as _Cell>::Noun>) {
        (self.head.map(|n| *n), self.tail.map(|n| *n))
    }
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

impl _IntoNoun for Cell {
    type Error = ();
    type Noun = Noun;

    fn into_noun(self) -> Result<Self::Noun, Self::Error> {
        Ok(Noun::Cell(self))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Wrap a value in an Option<Box<>>.
    macro_rules! b {
        ($inner:expr) => {
            Some(Box::new($inner))
        };
    }

    /// Create a new Noun::Atom from a list of numbers.
    macro_rules! na {
        ($elem:expr , $n:expr) => {
            let vec = vec![$elem; $n];
            Noun::Atom(Atom(vec))
        };
        ($($x:expr),+ $(,)?) => {
            {
                let mut vec = Vec::new();
                $(
                    vec.push($x);

                 )*
                Noun::Atom(Atom(vec))
            }
        };
    }

    /// Create a new cell from a pair of Option<Box<<>>.
    macro_rules! nc {
        ($head:expr, $tail:expr) => {
            Noun::Cell(Cell {
                head: $head,
                tail: $tail,
            })
        };
    }

    #[test]
    fn noun_get() {
        // [[4 5] [6 14 15]]
        let tt = nc!(b!(na![14]), b!(na![15]));
        let t = nc!(b!(na![6]), b!(tt.clone()));
        let h = nc!(b!(na![4]), b!(na![5]));
        let n = nc!(b!(h.clone()), b!(t.clone()));

        assert_eq!(n.get(1), Some(&n));
        assert_eq!(n.get(2), Some(&h));
        assert_eq!(n.get(3), Some(&t));
        assert_eq!(n.get(7), Some(&tt));
        assert_eq!(n.get(12), None);
    }
}
