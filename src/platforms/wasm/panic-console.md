Warning: this page has not been updated for Bevy 0.10 yet!

# Panic Messages

{{#include ../../include/links.md}}

Unless we do something about it, you will not be able to see Rust panic
messages when running in a web browser. This means that, if your game crashes,
you will not know why.

To fix this, we can set up a panic hook that will cause
the messages to appear in the browser console, using the
[console_error_panic_hook][project::console_error_panic_hook] crate.

Add the crate to your dependencies in `Cargo.toml`:

```toml
[dependencies]
console_error_panic_hook = "0.1"
```

At the start of your main function, before doing anything else, add this:

```rust,no_run,noplayground
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
```
