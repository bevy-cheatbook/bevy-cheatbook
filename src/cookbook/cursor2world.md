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

This code will only work on version 0.9 and higher, for previous versions, see [here](https://github.com/bevy-cheatbook/bevy-cheatbook/blob/5baa8f74860068e9d0714cd2864d4c026acccdc7/src/code/examples/cursor2world.rs
)!

(there will likely be *slight* inaccuracy from floating-point calculations)

```rust,no_run,noplayground
{{#include ../code/examples/cursor2world.rs:example}}
```
