# Better Error Messages

By default a WASM application doesn't give informative error messages in the browser console. This can be solved by using the [console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook) crate.

Add the crate to your dependencies in `Cargo.toml`:

```toml
[dependencies]
console_error_panic_hook = "0.1"
```

And make the following function call in your main function, making sure it is
only called once.

```rust
fn main() {
    set_panic_hook();

    // The rest of your code.
}


/// Improve error messages in the browser when running as WebAssembly.
/// For more details see
/// https://github.com/rustwasm/console_error_panic_hook#readme
fn set_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}
```
