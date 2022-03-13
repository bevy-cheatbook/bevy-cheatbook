# React to Changes with Asset Events

{{#include ../include/links.md}}

If you need to perform specific actions when an asset is created,
modified, or removed, you can make a [system][cb::system] that reacts to
[`AssetEvent`][bevy::AssetEvent] [events][cb::event].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-event}}
```
