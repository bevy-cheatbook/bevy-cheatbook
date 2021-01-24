# Assets

Bevy has a flexible system for loading and managing your game assets
asynchronously (in the background, without causing lag spikes in your game).

The data of your loaded assets is stored in an `Assets<T>` resource.

To access a specific asset, you need a `Handle<T>` for it. Handles are just lightweight IDs for a specific asset.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:asset-access}}
```

To load assets from files, use the `AssetServer` resource.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:asset-server}}
```

You can also create assets using code and add them to `Assets<T>` manually.

This is useful if you want to create them using code (such as for procedural
generation), or if you have gotten the data in some other way.

## Handles

Handles have built-in reference counting (similar to `Rc`/`Arc` in Rust).

Therefore, you need to `.clone()` them to create multiple handles to the same
asset. The clone is a cheap operation, but it is explicit, to ensure that you
are aware of when your code is creating more of them.

Handles can be "strong" (default) or "weak". If no more strong handles
exist, the asset's data will be freed/unloaded automatically.

Bevy also has a `HandleUntyped` type. Use this type of handle if you need to
be able to refer to any asset, regardless of the asset type.

This allows you to have a `Vec` or `HashMap` containing assets of mixed types.

Conveniently, the `AssetServer` supports untyped loading, if you don't know
what asset type the files are. It also supports loading entire folders.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:asset-folder}}
```

It will try to detect the format of each asset based on the file extension.


