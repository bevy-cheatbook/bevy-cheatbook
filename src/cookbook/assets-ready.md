# Track Assets Loading

[Click here for the full example code.](../code/examples/assets-ready.rs)

---

You might want to know when your various assets have all finished loading, and
take some action, such as exiting your loading screen and starting gameplay.

To do this, we can convert our various handles to `HandleUntyped`s, so we can
add them all into a single collection. Then we can loop over the collection
and ask the `AssetServer` about their status.

```rust,no_run,noplayground
{{#include ../code/examples/assets-ready.rs:example}}
```

Bevy also provides an API to test the loading status of a collection of
`HandleId`:

```rust,no_run,noplayground
{{#include ../code/examples/assets-ready-group.rs:example}}
```
