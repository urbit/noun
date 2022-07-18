//! This library provides a batteries-included [noun] implementation, which is [Urbit]'s
//! native data structure.
//!
//! # Thread Safety
//!
//! By default, this library uses [`std::rc::Rc`] as its reference-counting pointer, which is not
//! thread-safe. To use this library in a multi-threaded context, enable the `thread-safe` feature,
//! which will use [`std::sync::Arc`], a thread-safe reference-counting pointer, instead of
//! [`std::rc::Rc`].
//!
//! [Urbit]: https://urbit.org
//! [noun]: https://urbit.org/docs/glossary/noun
//! [`std::rc::Rc`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
//! [`std::sync::Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html

pub mod atom;
pub mod cell;
pub mod convert;
pub mod noun;
pub mod serdes;

#[doc(inline)]
pub use crate::noun::*;

/// A reference-counting pointer.
///
/// Alias for [`std::rc::Rc`] when `thread-safe` feature is disabled.
#[cfg(not(feature = "thread-safe"))]
pub type Rc<T> = std::rc::Rc<T>;

/// A reference-counting pointer.
///
/// Alias for [`std::sync::Arc`] when `thread-safe` feature is enabled.
#[cfg(feature = "thread-safe")]
pub type Rc<T> = std::sync::Arc<T>;
