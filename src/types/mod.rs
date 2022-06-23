pub mod atom;
pub mod cell;
pub mod noun;

#[cfg(test)]
mod tests {
    use super::{atom::*, cell::*, noun::*};
    use crate::{Cell as _, Noun as _};
    use std::rc::Rc;

    #[test]
    fn noun_cue() {}

    #[test]
    fn noun_get() {
        /// Create a new Noun::Atom from a list of numbers.
        macro_rules! na {
            ($elem:expr , $n:expr) => {
                let vec = vec![$elem; $n];
                Noun::Atom(Atom::from(vec))
            };
            ($($x:expr),+ $(,)?) => {
                {
                    let mut vec = Vec::new();
                    $(
                        vec.push($x);

                     )*
                        Noun::Atom(Atom::from(vec))
                }
            };
        }

        /// Create a new cell from a pair of Option<Box<<>>.
        macro_rules! nc {
            ($head:expr, $tail:expr) => {
                Noun::Cell(Cell::new($head, $tail))
            };
        }

        // [[4 5] [6 14 15]]
        let tt = nc!(Rc::new(na![14]), Rc::new(na![15]));
        let t = nc!(Rc::new(na![6]), Rc::new(tt.clone()));
        let h = nc!(Rc::new(na![4]), Rc::new(na![5]));
        let n = nc!(Rc::new(h.clone()), Rc::new(t.clone()));

        assert_eq!(n.get(1), Some(&n));
        assert_eq!(n.get(2), Some(&h));
        assert_eq!(n.get(3), Some(&t));
        assert_eq!(n.get(7), Some(&tt));
        assert_eq!(n.get(12), None);
    }
}
