# wrapper-lite

[![](https://img.shields.io/crates/v/wrapper-lite)](https://crates.io/crates/wrapper-lite)
[![](https://img.shields.io/docsrs/wrapper-lite)](https://docs.rs/wrapper-lite)
![GitHub Tag](https://img.shields.io/github/v/tag/hanyu-dev/wrapper-lite)

The new type idiom gives compile time guarantees that the right type of value is supplied to a program.

This crate provides a simple macro for you to create a wrapper over _any_ type.

## Migrate from v0.1.X

To make `cargo-fmt` happy, starting from v0.2.0, the following usage which is not a valid Rust struct syntax is no longer accepted.

```rust,compile_fail
wrapper_lite::wrapper!(
    pub ExampleWrapper(u8)
);
```

Instead:

```rust
wrapper_lite::wrapper!(
    pub struct ExampleWrapper(u8);
);
```

Now we can format the macro content with `cargo fmt`!

## MSRV

1.56.0

## LICENSE

MIT
