{{#include ../include/header09.md}}

# Load Assets from Files with AssetServer

Relevant official examples:
[`asset_loading`][example::asset_loading].

---

To load assets from files, use the [`AssetServer`][bevy::AssetServer]
[resource][cb::res].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-server}}
```

This queues the asset loading to happen in the background, and return a
[handle][cb::handle]. The asset will take some time to become available. You
cannot access the actual data immediately in the same [system][cb::system],
but you can use the handle.

You can spawn entities like your 2D sprites, 3D models, and UI, using the
handle, even before the asset has loaded. They will just "pop in" later,
when the asset becomes ready.

Note that it is OK to call `asset_server.load(…)` as many times as you want,
even if the asset is currently loading, or already loaded. It will just
provide you with the same handle. Every time you call it, it will just check
the status of the asset, begin loading it if needed, and give you a handle.

Bevy supports loading [a variety of asset file formats][builtins::file-formats],
and can be extended to support more. The asset loader implementation to use
is selected based on the file extension.

## Untyped Loading

If you want an [untyped handle][cb::handleuntyped], you can use
`asset_server.load_untyped(…)` instead.

Untyped loading is possible, because Bevy always detects the file type from
the file extension anyway.

### Loading Folders

You can also load an entire folder of assets, regardless of how many
files are inside, using `asset_server.load_folder(…)`. This gives you a
`Vec<HandleUntyped>` with all the [untyped handles][cb::handleuntyped].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-folder}}
```

Loading folders is not supported by all I/O backends. Notably, it does not
work on WASM/Web.

## AssetPath and Labels

The asset path you use to identify an asset from the filesystem is actually
a special [`AssetPath`][bevy::AssetPath], which consists of the file path +
a label. Labels are used in situations where multiple assets are contained
in the same file. An example of this are [GLTF files][cb::gltf], which can
contain meshes, scenes, textures, materials, etc.

Asset paths can be created from a string, with the label (if any) attached
after a `#` symbol.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-path-labels}}
```

See the [GLTF page][cb::gltf] for more info about working with 3D models.

## Where are assets loaded from?

The asset server internally relies on an implementation of the
[`AssetIo`][bevy::AssetIo] Rust trait, which is Bevy's way of providing
"backends" for fetching data from different types of storage.

Bevy provides its own default built-in I/O backends for each [supported
platform][chapter::platforms].

On desktop platforms, it treats asset paths as relative to a folder called
`assets`, that must be placed at one of the following locations:
 - Alongside the game's executable file, for distribution
 - In your Cargo project folder, when running your game using `cargo` during development
   - This is identified by the `CARGO_MANIFEST_DIR` environment variable

On the web, it fetches assets using HTTP URLs pointing within an `assets`
folder located alongside the game's `.wasm` file.

There are [unofficial plugins][cb::3rdparty] available that provide alternative
I/O backend implementations, such as for loading assets from inside archive
files (`.zip`), embedded inside the game executable, using a network protocol,
… many other possibilities.
