Warning: this page has not been updated for Bevy 0.10 yet!

# Hot-Reloading Assets

{{#include ../include/links.md}}

Relevant official examples:
[`hot_asset_reloading`][example::hot_asset_reloading].

---

At runtime, if you modify the file of an [asset][cb::asset]
that is [loaded][cb::assetserver] into the game (via the
[`AssetServer`][bevy::AssetServer]), Bevy can detect that and reload the
asset automatically. This is very useful for quick iteration. You can edit
your assets while the game is running and see the changes instantly in-game.

Not all [file formats][builtins::file-formats] and use cases are supported
equally well. Typical asset types like textures / images should work without
issues, but complex GLTF or scene files, or assets involving custom logic,
might not.

If you need to run custom logic as part of your hot-reloading
workflow, you could implement it in a [system][cb::system], using
[`AssetEvent`][bevy::AssetEvent] ([learn more][cb::assetevent]).

Hot reloading is opt-in and has to be enabled in order to work:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-watch}}
```

Note that this requires the `filesystem_watcher` [Bevy cargo
feature][cb::features]. It is enabled by default, but if you have disabled
default features to customize Bevy, be sure to include it if you need it.

## Shaders

Bevy also supports hot-reloading for shaders. You can edit your custom shader
code and see the changes immediately.

This works for any shader loaded from a file path, such as shaders specified
in your Materials definitions, or shaders [loaded][cb::assetserver] via the
[`AssetServer`][bevy::AssetServer].

Shader code that does not come from asset files, such as if you include it
as a static string in your source code, cannot be hot-reloaded (for obvious
reasons).
