{{#include ../include/header012.md}}

# Convert cursor to world coordinates

## 2D games

If you only have one window (the primary window), as is the case for most apps
and games, you can do this:

<details>
  <summary>
  <code>Code (simple version):</code>
  </summary>

```rust,no_run,noplayground
{{#include ../code012/src/cookbook/cursor2world.rs:simple}}
```

```rust,no_run,noplayground
{{#include ../code012/src/cookbook/cursor2world.rs:simple-app}}
```

</details>

If you have a more complex application with multiple windows, here is a more
complex version of the code that can handle that:

<details>
  <summary>
  <code>Code (multi-window version):</code>
  </summary>

```rust,no_run,noplayground
{{#include ../code012/src/cookbook/cursor2world.rs:multiple-windows}}
```

```rust,no_run,noplayground
{{#include ../code012/src/cookbook/cursor2world.rs:multiple-windows-app}}
```

</details>

## 3D games

If you'd like to be able to detect what 3D object the cursor is pointing at, select
objects, etc., there is a good (unofficial) plugin:
[`bevy_mod_picking`][project::bevy_mod_picking].

For a simple top-down camera view game with a flat ground plane, it might be
sufficient to just compute the coordinates on the ground under the cursor.

<button class="button_wasm_cbexample" id="button_cursor_3d_ground_plane">Load Interactive Example</button>

In the interactive example, there is a ground plane with a non-default position
and rotation. There is a red cube, which is positioned using the global
coordinates, and a blue cube, which is a [child entity][cb::hierarchy] of the
ground plane and positioned using local coordinates. They should both follow the
cursor.

<details>
  <summary>
  <code>Code and explanation:</code>
  </summary>

```rust,no_run,noplayground
{{#include ../code012/src/cookbook/cursor2world.rs:3d-ground-plane}}
```

```rust,no_run,noplayground
{{#include ../code012/src/cookbook/cursor2world.rs:3d-ground-plane-app}}
```

If the ground is tilted/rotated or moved, the global and local coordinates
will differ, and may be useful for different use cases, so we compute both.

For some examples:
 - if you want to spawn a [child][cb::hierarchy] entity, or to quantize
   the coordinates to a grid (for a tile-based game, to detect the grid tile under the cursor),
   the local coordinates will be more useful
 - if you want to spawn some overlays, particle effects, other independent game entities,
   at the position of the cursor, the global coordinates will be more useful

</details>

<script type="module" src="/loadwasm.js"/>
