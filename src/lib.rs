//! [Urbit]-native data structures: [atoms], [cells], and [nouns].
//!
//! [Urbit]: https://urbit.org
//! [atoms]: https://urbit.org/docs/glossary/atom
//! [cells]: https://urbit.org/docs/glossary/cell
//! [nouns]: https://urbit.org/docs/glossary/noun

pub mod atom;
pub mod cell;
pub mod convert;
pub mod noun;
pub mod serdes;

/// Reference-counting pointer.
///
/// Alias for [`std::rc::Rc`] when `thread-safe` feature is disabled.
///
/// [`std::rc::Rc`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
#[cfg(not(feature = "thread-safe"))]
pub type Rc<T> = std::rc::Rc<T>;

/// Reference-counting pointer.
///
/// Alias for [`std::sync::Arc`] when `thread-safe` feature is enabled.
///
/// [`std::sync::Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
#[cfg(feature = "thread-safe")]
pub type Rc<T> = std::sync::Arc<T>;
