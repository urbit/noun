//! Marker traits.

use crate::{atom::Atom, cell::Cell, noun::Noun};

macro_rules! impl_marker {
    ($trait:ty, $type:ty) => {
        impl $trait for $type {}
        impl $trait for &$type {}
        impl $trait for std::boxed::Box<$type> {}
        impl $trait for std::rc::Rc<$type> {}
        impl $trait for std::sync::Arc<$type> {}
    };
}

/// Marker trait for atom-like types.
pub trait Atomish {}

impl_marker!(Atomish, Atom);

/// Marker trait for cell-like types.
pub trait Cellish {}

impl_marker!(Cellish, Cell);

/// Marker trait for noun-like types.
pub trait Nounish {}

impl_marker!(Nounish, Atom);
impl_marker!(Nounish, Cell);
impl_marker!(Nounish, Noun);
