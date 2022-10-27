# [Noun](https://developers.urbit.org/reference/glossary/noun)

[![Cargo build](https://github.com/urbit/noun/actions/workflows/cargo-build.yml/badge.svg)](https://github.com/urbit/noun/actions/workflows/cargo-build.yml)
[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.txt)

This repository is intended to serve as a building block for Rust code that
needs to interact with an [Urbit][urbit] node by providing a native Rust
representation of the [noun][noun], Urbit's native data structure. For more
information, consult the documentation (see below).

### Getting Started

Ensure you have an up-to-date Rust toolchain installed on your machine. If you
need Rust installation instructions, head to [rust-lang.org][rust].

To build, run:
```console
$ cargo build --release
```

If you want a debug build, run:
```console
$ cargo build
```

To build and run the test suite, run:
```console
$ cargo test
```

To build and view the documentation, run:
```console
$ cargo doc --open
```


[noun]: https://developers.urbit.org/reference/glossary/noun
[rust]: https://www.rust-lang.org/tools/install
[urbit]: https://urbit.org
