Warning: this page has not been updated for Bevy 0.10 yet!

# React to Changes with Asset Events

{{#include ../include/links.md}}

If you need to perform specific actions when an asset is created,
modified, or removed, you can make a [system][cb::system] that reacts to
[`AssetEvent`][bevy::AssetEvent] [events][cb::event].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-event}}
```

**Note:** If you are handling `Modified` events and doing a mutable access to
the data, the `.get_mut` will trigger another `Modified` event for the same
asset. If you are not careful, this could result in an infinite loop! (from
events caused by your own system)
