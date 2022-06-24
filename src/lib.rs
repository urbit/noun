//! [Urbit]-native data structures: [atoms], [cells], and [nouns].
//!
//! [Urbit]: https://urbit.org
//! [atoms]: https://urbit.org/docs/glossary/atom
//! [cells]: https://urbit.org/docs/glossary/cell
//! [nouns]: https://urbit.org/docs/glossary/noun

pub mod atom;
pub mod cell;
pub mod noun;
pub mod ops;
pub mod serdes;

pub use crate::noun::*;

#[macro_export]
macro_rules! create_test {
    ($success:expr, $return:ty, $($test:block)+) => {
        fn run_test<A, C, N>() -> $return
        where
            A: crate::atom::Atom<C, N>,
            C: crate::cell::Cell<A, N>,
            N: crate::noun::Noun<A, C>,
        {
            $(
                $test
            )*

            $success
        }
    };
}
