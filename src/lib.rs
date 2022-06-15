pub mod atom;
pub mod cell;
pub mod serdes;

use crate::{
    atom::Atom,
    cell::Cell,
    serdes::{Cue, Jam},
};
use std::hash::Hash;

pub trait Noun<A, C>
where
    A: Atom<C, Self>,
    C: Cell<A, Self>,
    Self: Cue + Jam + Eq + Hash + Sized,
{
    fn into_atom(self) -> Result<A, Self>;

    fn into_cell(self) -> Result<C, Self>;
}
