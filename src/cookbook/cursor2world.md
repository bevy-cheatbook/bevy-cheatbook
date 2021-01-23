# Convert cursor to world coordinates

[Click here for the full example code.](../code_bevy_release/examples/cursor2world.rs)

---

Bevy does not yet provide built-in functions to help with finding out what the cursor is pointing at.

## 3D games

There is a good (unofficial) plugin: [`bevy_mod_picking`](https://github.com/aevyrie/bevy_mod_picking).

## 2D games

For a game using the default bevy 2d orthographic camera:

```rust,no_run,noplayground
{{#include ../code_bevy_release/examples/cursor2world.rs:example}}
```
