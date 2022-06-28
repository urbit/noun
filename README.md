# noun

[Urbit](https://urbit.org) uses the [noun](https://urbit.org/docs/glossary/noun)
as its only data structure. The [Urbit runtime](https://urbit.org/docs/vere) does not *need* to rely exclusively on nouns
to represent data, but nouns are naturally the best choice in a lot of
circumstances. However, that doesn't mean that a single noun representation is
the best choice in all scenarios in which nouns are used in the runtime. The
noun representation used by the [Nock](https://urbit.org/docs/nock) interpreter,
for instance,
has different requirements than the noun representation used by the event loop
and the IO drivers. 

This library provides traits for nouns, atoms, and cells
that define the required functionality that any type claiming to be a noun,
atom, or cell, respectively, must implement. It also includes traits for
converting non-noun types to and from nouns, serializing (jamming) nouns,
deserializing (cueing) nouns, and a handful of other noun-specific operations.
Finally, the library provides concrete noun, atom, and cell types that can be
used out of the box.

### Getting Started

##### Build

To build, run:
```console
$ cargo build
```

##### Test

To build and run the test suite, run:
```console
$ cargo test
```

##### Read

To build and view the documentation, run:
```console
$ cargo doc --open
```
