# Track Assets Loading

[Click here for the full example code.](../code/examples/assets-ready.rs)

---

You might want to know when your various assets have all finished loading, and
take some action, such as exiting your loading screen and starting gameplay.

To do this, we can convert our various handles into `HandleUntyped`s, so we can
add them all into a single collection.

We can then ask the `AssetServer` about the loading state of the collection.

```rust,no_run,noplayground
{{#include ../code/examples/assets-ready.rs:example}}
```

It is also possible to query the load state of a single asset, by passing an
individual handle to `AssetServer.get_load_state()`.
