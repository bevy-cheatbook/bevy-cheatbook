# Random Number Generation

The typical way to generate random numbers in Rust is using the `rand` crate.
However, it requires some extra configuration to work with WASM. This is
because, typically, randomness comes from the operating system. In a web app,
it needs to come from the browser.

The `rand` developers decided not to support this directly, but instead
require us to set a feature flag on the `getrandom` crate, that they depend on.

To use random numbers in your project, add this to your dependencies in
`Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
getrandom = { version = "0.2", features = ["js"] }
```

Note that you must ensure that all the versions are compatible. `getrandom`
0.2 is the version that is needed for both `rand` 0.8 and `bevy` 0.5. Using
other versions may result in you having multiple instances of the `getrandom`
crate in your dependency tree!

