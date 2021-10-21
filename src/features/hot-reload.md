# Hot-Reloading Assets

Relevant official examples:
[`hot_asset_reloading`](https://github.com/bevyengine/bevy/blob/latest/examples/asset/hot_asset_reloading.rs),
[`hot_shader_reloading`][`hot_shader_reloading`].

---

At runtime, if you modify the file of an [asset](./assets.md) that is loaded
into the game (via the `AssetServer`), Bevy will detect that and reload the
asset automatically. This is very useful for quick iteration. You can edit
your assets while the game is running and see the changes instantly in-game.

Not all file formats and use cases are supported equally well. Typical asset
types like textures / images should work without issues, but complex GLTF or
scene files, or assets involving custom logic, might not.

If you need to run custom logic as part of your hot-reloading workflow, you
could implement it in a system, using [`AssetEvent`](./assets.md#assetevent).

Hot reloading is opt-in and has to be enabled in order to work. You can do
this in a [startup system](../programming/app-builder.md):

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-watch}}
```

## Shaders

Bevy also supports hot-reloading for shaders. You can edit your custom shader
code and see the changes immediately. This only works if you are loading
your shaders through the bevy asset system (via the `AssetServer`). See the
[official example][`hot_shader_reloading`].


[`hot_shader_reloading`]: https://github.com/bevyengine/bevy/blob/latest/examples/shader/hot_shader_reloading.rs
