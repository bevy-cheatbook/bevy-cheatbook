# Track Loading Progress

{{#include ../include/links.md}}

There are good community plugins that can help with this. See [my
recommendations for helper crates][cb::3rdparty::code-helpers]. Otherwise,
this page shows you how to do it manually.

---

If you want to check the status of various [asset files][cb::assetserver],
you can poll it from the [`AssetServer`][bevy::AssetServer]. It will tell you
whether the asset(s) are loaded, still loading, not loaded, or encountered
an error.

To check an individual asset, you can use `asset_server.get_load_state(…)` with
a handle or path to refer to the asset.

To check a group of many assets, you can add them to a single collection
(such as a `Vec<HandleUntyped>`; [untyped handles][cb::handleuntyped] are very
useful for this) and use `asset_server.get_group_load_state(…)`.

---

Here is a more complete code example:

```rust,no_run,noplayground
{{#include ../code/examples/assets-ready.rs:example}}
```
