**This project is in progress.**

This is a crate that generates bindings using the C SDK for PY32F0. Currently, installing `arm-none-eabi-gcc` is required to use this library. Use `arm-none-eabi-gcc -v` in cmd to verify the installation.

**TODO:**

--

To handle macros with type conversions, this library requires using [this Pull Request](https://github.com/rust-lang/rust-bindgen/pulls). However, to compile this PR, you need to set the dependency address of `cmacro` to https://github.com/reitermarkus/cmacro-rs. You might be able to achieve this using `[patch.crates-io]` or by directly fetching the library.

A more convenient way is to use the precompiled static library.

New Chips:
1. `(*(uint32_t *)`
2. 