{{#include ../include/header011.md}}

# Convert cursor to world coordinates

[Click here for the full example code.][cbexample::cursor2world]

---

## 3D games

There is a good (unofficial) plugin:
[`bevy_mod_picking`][project::bevy_mod_picking].

## 2D games

If you only have one window (the primary window), as is the case for most apps
and games, you can do this:

<details>
  <summary>
  <code>Code (simple version):</code>
  </summary>

```rust,no_run,noplayground
{{#include ../code011/examples/cursor2world.rs:simple}}
```

</details>

If you have a more complex application with multiple windows, here is a more
complex version of the code that can handle that:

<details>
  <summary>
  <code>Code (multi-window version):</code>
  </summary>

```rust,no_run,noplayground
{{#include ../code011/examples/cursor2world.rs:multiple-windows}}
```

</details>
