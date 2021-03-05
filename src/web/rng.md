# Random number generation

You may have noticed when using the `rand` crate in a WASM application that it
results in a panic. This is because most crates handling random number
generation by default look for an operating system call to retrieve entropy,
which is necessary to create random numbers. In a WASM context, which is
essentially a sandbox environment in your browser, there is no such call and
thus this sort of thing fails.

The solution here is to use a WASM-compatible random number generator, and/or
to enable the required feature. The `rand` crate version 0.7 and earlier had a
`wasm-bindgen` feature, but since version 0.8 the official solution is to
toggle the feature flag on `rand` its dependency, `getrandom`' yourself.

In your `Cargo.toml`:

```toml
rand = "0.8"
getrandom = { version = "0.1", features = ["wasm-bindgen"] }
```

Note: do not use a later version of `getrandom` than 0.1. This will result in
namespace conflicts (and thus runtime panics) due to Bevy also indirectly depending on `getrandom` version 0.1. This is a known problem and a solution is currently being discussed.
