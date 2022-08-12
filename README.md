# [Noun](https://developers.urbit.org/reference/glossary/noun)

[![Cargo build](https://github.com/mcevoypeter/noun/actions/workflows/cargo-build.yml/badge.svg)](https://github.com/mcevoypeter/noun/actions/workflows/cargo-build.yml)
[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.txt)

This repo is intended to serve as a building block for Rust code that needs to
interact with an [Urbit](https://urbit.org) node by providing a native Rust
representation of the noun, Urbit's native data structure. For more information,
consult the documentation.

### Getting Started

To build everything (libraries, test suite, and documentation), run:
```console
$ cargo build && cargo test && cargo doc
```

To build the libraries, run:
```console
$ cargo build --release
```

To build and run the test suite, run:
```console
$ cargo test
```

To build and view the documentation, run:
```console
$ cargo doc --open
```
