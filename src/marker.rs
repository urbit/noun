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

/// Atom-like types.
pub trait Atomish {}

impl_marker!(Atomish, Atom);

/// Cell-like types.
pub trait Cellish {}

impl_marker!(Cellish, Cell);

/// Noun-like types.
pub trait Nounish {}

impl_marker!(Nounish, Atom);
impl_marker!(Nounish, Cell);
impl_marker!(Nounish, Noun);
