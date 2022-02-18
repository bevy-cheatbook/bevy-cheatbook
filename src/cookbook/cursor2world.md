# Convert cursor to world coordinates

{{#include ../include/links.md}}

[Click here for the full example code.][cbexample::cursor2world]

---

Bevy does not yet provide built-in functions to help with finding out what
the cursor is pointing at.

## 3D games

There is a good (unofficial) plugin:
[`bevy_mod_picking`][project::bevy_mod_picking].

## 2D games

This code should work for 2D games with orthographic projections. It is
"undoing" the projection and camera transformations.

(there will likely be *slight* inaccuracy from floating-point calculations)

This should work regardless of the scaling settings of your projection, and
camera [transform][cb::transform].

```rust,no_run,noplayground
{{#include ../code/examples/cursor2world.rs:example}}
```
