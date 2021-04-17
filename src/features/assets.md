# Assets

Bevy has a flexible system for loading and managing your game assets
asynchronously (in the background, without causing lag spikes in your game).

The data of your loaded assets is stored in `Assets<T>` resources.

Assets are tracked using handles. Handles are just lightweight IDs for specific assets.

## Loading using AssetServer

To load assets from files, use the `AssetServer` resource.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-server}}
```

This queues the asset loading to happen in the background. The asset will take
some time to become available. You cannot access the actual data immediately in
the same system, but you can use the handle.

You can spawn your 2D sprites, 3D models, and UI, using the handle, even before
the asset has loaded. They will just "pop in" later, when the asset becomes ready.

Note that it is OK to call `asset_server.load` as many times as you want, even
if the asset is currently loading, or already loaded. It will just provide you
with the same handle. If the asset is unavailable, it will begin loading.

## Creating your own assets

You can also add assets to `Assets<T>` manually.

This is useful if you want to create them using code (such as for procedural
generation), or if you have gotten the data in some other way.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-add}}
```

## Handles

Handles are the typical way to refer to a particular asset. When you spawn
things into your game, such as 2D sprites, 3D models, or UI, their respective
components will need handles for the assets they use.

You could store your handles somewhere that is convenient for you (such as in
[resources](../programming/res.md)).

If you don't have your handle stored anywhere, you can always generate one from
a path by calling `asset_server.load`. You could simply do that whenever you
need, and not bother storing handles.

## Accessing the Assets

To access the actual asset data from systems, use the `Assets<T>` resource.

You can identify your desired asset, using either the handle or the asset path:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-access}}
```

## AssetPath and Labels

Assets from the filesystem can be identified by an `AssetPath`, which consists of
the file path + a label. Labels are used in situations where multiple assets are
contained in the same file. An example of this are GLTF files, which can contain
meshes, scenes, textures, materials, etc.

Asset paths can be created from a string, with the label (if any) attached after a `#` symbol.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-path-labels}}
```

(For GLTF, Bevy will generate labels like `Scene0` based on the index of each
object in the file, but if your GLTF file includes names/labels exported from
your 3D modeling software, Bevy will make those available, too)

## Handles and Asset Lifetime (Garbage Collection)

Handles have built-in reference counting (similar to `Rc`/`Arc` in Rust). This
allows Bevy to track if an asset is still needed, and automatically unload it
when it no longer is.

You can use `.clone()` to create multiple handles to the same asset. The clone
is a cheap operation, but it is explicit, to ensure that you are aware of the
places in your code that create additional handles and may affect the lifetime
of assets.

### Weak Handles

Handles can be "strong" (the default) or "weak". Only strong handles are counted
and cause the asset to remain loaded. Weak handles let you refer to an asset,
while allowing it to be garbage-collected when no more strong handles remain.

You can create weak handles using `.clone_weak()` instead of `.clone()`.

## Untyped Handles

Bevy also has a `HandleUntyped` type. Use this type of handle if you need to
be able to refer to any asset, regardless of the asset type.

This allows you to store a collection (such as `Vec` or `HashMap`) containing
assets of mixed types.

You can create an untyped handle using `.clone_untyped()`.

### Untyped Loading

Conveniently, the `AssetServer` supports untyped loading, if you don't know
what asset type the files are. You can also load an entire folder:

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
