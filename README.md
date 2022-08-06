# [Noun](https://developers.urbit.org/reference/glossary/noun)

[![Bazel build](https://github.com/mcevoypeter/noun/actions/workflows/bazel-build.yml/badge.svg)](https://github.com/mcevoypeter/noun/actions/workflows/bazel-build.yml)
[![Cargo build](https://github.com/mcevoypeter/noun/actions/workflows/cargo-build.yml/badge.svg)](https://github.com/mcevoypeter/noun/actions/workflows/cargo-build.yml)
[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.txt)

This repo is intended to serve as a building block for Rust code that needs to
interact with an [Urbit](https://urbit.org) node by providing a native Rust
representation of the noun, Urbit's native data structure. For more information,
consult the documentation.

### Getting Started

Either `bazel` or `cargo` can be used to build the crate's libraries, tests, and
documentation. The crate can be built as a Rust-native library, a static
library, or both.

To build everything (libraries, test suite, and documentation), run:
```console
$ bazel build ...
```
or
```console
$ cargo build && cargo test && cargo doc
```

To build the libraries, run:
```console
$ bazel build :noun_rust_lib :noun_static_lib
```
or
```console
$ cargo build
```

To build and run the test suite, run:
```console
$ bazel test :noun_test :noun_doc_test
```
or
```console
$ cargo test
```

To build and view the documentation, run:
```console
$ bazel build :noun_doc && <open_command> bazel-bin/noun_doc.rustdoc/noun/index.html
```
or
```console
$ cargo doc --open
```
