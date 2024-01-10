# Maybe Implements &emsp; ![CI][ci-badge] [![Crates.io][crates-badge]][crates-url] [![MIT licensed][mit-badge]][mit-url] 

[crates-badge]: https://img.shields.io/crates/v/maybe-impl.svg
[crates-url]: https://crates.io/crates/maybe-impl
[mit-badge]: https://img.shields.io/badge/license-MIT-blueviolet.svg
[mit-url]: https://github.com/commonsensesoftware/maybe-impl/blob/main/LICENSE
[ci-badge]: https://github.com/commonsensesoftware/maybe-impl/actions/workflows/ci.yml/badge.svg

Maybe Implements provides Rust procedural macro attributes that can be used to optionally implement traits.

## Optional Traits in Action

Technically, the provided attribute **always** implements the specified traits. It can be combined with the
existing `cfg_attr` attribute to conditionally implement one or more traits. The primary use case for such
behavior is optionally implementing `Send` and `Sync`.

First, consider that asynchronous behavior is an optional feature in your crate:

```toml
[features]
async = ["maybe-impl"]

[dependencies]
maybe-impl = { version = "0.1.0", optional = true }
```

Consider the crate has the following trait:

```rust
#[cfg_attr(feature = "async", maybe_impl::traits(Send,Sync))]
trait Foo {
    fn bar(&self);
}
```

When the crate is used with the default features, there is no change in behavior. When the `async` feature is enabled:

```bash
cargo add my-crate --features async
```

The trait is expanded to:

```rust
trait Foo: Send + Sync {
    fn bar(&self);
}
```

## Using Generics

If you define a trait with generics that requires `Send` and/or `Sync`, specifically, that will require the generic type constraint to match. [Trait aliases](https://doc.rust-lang.org/unstable-book/language-features/trait-alias.html) are currently unstable, however, you can bridge these two concepts with a _pseudo_ trait alias by defining a marker trait with a blanket implementation.

```rust
#[cfg(not(feature = "async"))]
trait Bar: Sized {}

#[cfg(not(feature = "async"))]
impl<T> Bar for T {}

#[cfg(feature = "async")]
trait Bar: Sized + Send + Sync {}

#[cfg(feature = "async")]
impl<T: Send + Sync> Bar for T {}

#[cfg_attr(feature = "async", maybe_impl::traits(Send,Sync))]
trait Foo<T: Bar> {
    fn bar(&self, bar: &T);
}
```

The default, synchronous build remains vanilla, while the asynchronous path expands to the equivalent of:

```rust
trait Foo<T: Send + Sync>: Send + Sync {
    fn bar(&self, bar: &T);
}
```

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/commonsensesoftware/more-rs-di/blob/main/LICENSE