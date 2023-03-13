Warning: this page has not been updated for Bevy 0.10 yet!

# Convert cursor to world coordinates

{{#include ../include/links.md}}

[Click here for the full example code.][cbexample::cursor2world]

---

## 3D games

There is a good (unofficial) plugin:
[`bevy_mod_picking`][project::bevy_mod_picking].

## 2D games

Starting from Bevy 0.9, there are camera methods to help you convert between
screen-space (viewport) and world-space coordinates. We can use this to easily
find the position of the mouse cursor.

This code will work regardless of the [camera's projection][cb::camera-projection]
settings and [transform][cb::transform].

(there will likely be *slight* inaccuracy from floating-point calculations)

```rust,no_run,noplayground
{{#include ../code/examples/cursor2world.rs:example}}
```

In older versions of Bevy, there were no such coordinate conversion methods, and
the math had to be done manually. If you are interested in how that works, see
the example from an old version of the book,
[here](https://github.com/bevy-cheatbook/bevy-cheatbook/blob/5baa8f74860068e9d0714cd2864d4c026acccdc7/src/code/examples/cursor2world.rs).
