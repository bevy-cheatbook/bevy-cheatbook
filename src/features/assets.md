# Assets

Bevy has a flexible system for loading and managing your game assets
asynchronously (in the background, without causing lag spikes in your game).

The data of your loaded assets is stored in `Assets<T>` resources.

Assets are tracked using handles. Handles are just lightweight IDs for a specific asset.

## Loading using AssetServer

To load assets from files, use the `AssetServer` resource.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-server}}
```

This queues the asset loading to happen in the background. The asset will take
some time to become available. You cannot access it immediately after loading in
the same system.

Note that it is OK to call `.load` as many times as you want, regardless of if
the asset is currently loading, or already loaded. It will just provide you with
a handle. If the asset is unavailable, it will begin loading.

## Creating your own assets

You can also create assets using code and add them to `Assets<T>` manually.

This is useful if you want to create them using code (such as for procedural
generation), or if you have gotten the data in some other way.

## Accessing the Assets

To access your assets from systems, use the `Handle<T>` and `Assets<T>` resource:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-access}}
```

## AssetPath

Assets from the filesystem are identified by an `AssetPath`, which consists of
the file path + a label. Labels are used in situations where multiple assets can
be loaded from the same file. An example of this are GLTF files, which can
contain meshes, scenes, textures, materials, etc.

Asset paths can be created from a string, with the label (if any) attached after a `#` symbol.

```rust,no_run,noplayground
```

## Working with Handles

Handles are based on asset paths (for loaded assets) or internal UUIDs (for
manually-added assets). This means that they can easily and deterministically be
"created out of thin air" at any time. You can always convert an asset path into
a handle, regardless of if that asset has ever been loaded.

## Handles and Asset Lifetime (Garbage Collection)

Handles have built-in reference counting (similar to `Rc`/`Arc` in Rust). This
allows Bevy to track if an asset is still needed, and automatically unload it
if it no longer is.

Handles can be "strong" or "weak". Only strong handles are counted and cause the
asset to remain loaded.

You can use `.clone()` or `.clone_weak()` to create multiple handles to the same
asset. The clone is a cheap operation, but it is explicit, to ensure that you
are aware of the places in your code that create additional handles and may
affect the lifetime of assets.

## Working with Handles

Bevy also has a `HandleUntyped` type. Use this type of handle if you need to
be able to refer to any asset, regardless of the asset type.

This allows you to have a `Vec` or `HashMap` containing assets of mixed types.

Conveniently, the `AssetServer` supports untyped loading, if you don't know
what asset type the files are. It also supports loading entire folders.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-folder}}
```

It will try to detect the format of each asset based on the file extension.


## AssetEvent

If you need to perform specific actions when the asset has finished loading, is
modified, or removed, you can react to `AssetEvent`s.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-event}}
```
