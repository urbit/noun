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
/// Alias for [`std::rc::Arc`] if the `thread-safe` feature is enabled or [`std::rc::Rc`] otherwise.
///
/// - [`std::rc::Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
/// - [`std::rc::Rc`]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
pub type Rc<T> = std::rc::Rc<T>;
