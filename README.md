# wrapper-lite

[![](https://img.shields.io/crates/v/wrapper-lite)](https://crates.io/crates/wrapper-lite)
[![](https://img.shields.io/docsrs/wrapper-lite)](https://docs.rs/wrapper-lite)
![GitHub Tag](https://img.shields.io/github/v/tag/hanyu-dev/wrapper-lite)

The new type idiom gives compile time guarantees that the right type of value is supplied to a program.

This crate provides a simple macro for you to create a wrapper over _any_ type.

## Migrate from v0.2.X

1. The macro now only accepts valid Rust struct syntax.

   ```rust
   wrapper_lite::wrapper!(
       pub struct TestWrapper1(u8); // <-- note the semicolon, now it's required
   );
   ```

   ```rust
   wrapper_lite::wrapper!(
       pub struct TestWrapper2 {
           inner: u8,
       }
   );
   ```
1. When there's no default value specified, we cannot implement the `From` trait for the wrapper type, and now it's a hard error.

   ```rust,compile_fail
   wrapper_lite::wrapper!(
       #[wrapper_impl(From)]
       pub struct TestWrapperComplex<'a, 'b> {
           inner: String,
           _a: ::core::marker::PhantomData<&'a ()>,
           _b: ::core::marker::PhantomData<&'b ()>,
       }
   );
   ```

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

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
