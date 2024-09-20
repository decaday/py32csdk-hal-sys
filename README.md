# py32csdk-hal-sys

[![Crates.io](https://img.shields.io/crates/v/py32csdk-hal-sys.svg)](https://crates.io/crates/py32csdk-hal-sys)

This is a crate that generates bindings using the C SDK for PY32F0.





**This package is used by py32-bind-hal, you can use py32-bind-hal's rust API.**

**[py32-bind-hal - crates.io](https://crates.io/crates/py32-bind-hal)**





## Supported MCU:

### ---PY32F0xx Series---

**Puya** PY32F002A, PY32F003, PY32F030

**Xinlinggo** XL32F003*, XL32F002A*

**Luat** AIR001

## Build

This library uses **precompiled static libraries** and **pregenerated bindings** by default.

When using a debug build, this crate uses -Og -g, and when using a release build, uses -Ofast.

These two static libraries are ready for you if you don't want to bother!

### --features=recompile

#### Generate Bindings

To handle macros with type conversions, this library requires using [this Pull Request](https://github.com/rust-lang/rust-bindgen/pulls). However, to compile this PR, you need to set the dependency address of `cmacro` to https://github.com/reitermarkus/cmacro-rs. You might be able to achieve this using `[patch.crates-io]` or by directly fetching the library.

##### Compile

This crate use [Clang](https://clang.llvm.org/) and [cc](https://crates.io/crates/cc) to compile CSDK.

## New Chips:

Some content needs to be processed manually or by script

1. `(*(uint32_t *)`
2. `__HAL_RCC_xxx`
3. Interrupt