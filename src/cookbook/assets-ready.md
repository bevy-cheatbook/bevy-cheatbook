# Track Assets Loading

[Click here for the full example code.](../code/examples/assets-ready.rs)

---

You might want to know when your various assets have all finished loading, and
take some action, such as exiting your loading screen and starting gameplay.

To do this, we can initiate the assets loading and collect the handle ids,
which can then be passed to the `AssetServer` in order to query the load
state of the collection.

```rust,no_run,noplayground
{{#include ../code/examples/assets-ready.rs:example}}
```

It's possible to query the load state of a single asset, by passing an
individual HandleId to the API `AssetServer.get_load_state()`.