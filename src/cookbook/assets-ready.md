# Track Assets Loading

{{#include ../include/links.md}}

[Click here for the full example code.][cbexample::assets-ready]

---

You might want to use a 3rd-party library for this. See [my recommendations for
helper crates][cb::3rdparty::code-helpers] that can do this for you. Otherwise,
this page shows you how to roll your own.

---

You might want to know when your various assets have all finished loading, and
take some action, such as exiting your loading screen and starting gameplay.

To do this, we can convert our various handles into
[`HandleUntyped`][bevy::HandleUntyped]s, so we can add them all into a
single collection.

We can then ask the [`AssetServer`][bevy::AssetServer] about the loading
state of the collection.

```rust,no_run,noplayground
{{#include ../code/examples/assets-ready.rs:example}}
```

It is also possible to query the load state of a single asset, by passing an
individual handle to `AssetServer.get_load_state()`.
