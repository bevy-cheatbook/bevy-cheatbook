{{#include ../include/header013.md}}

# Changing the Background Color

Relevant official examples:
[`clear_color`][example::clear_color].

---

Use the [`ClearColor`] [resource][cb::res] to choose the default background
color. This color will be used as the default for all [cameras][cb::camera],
unless overridden.

Note that the window will be black if no cameras exist. You must spawn at
least one camera.

```rust,no_run,noplayground
{{#include ../code013/src/window/clear_color.rs:main}}
```

To override the default and use a different color for a specific camera, you can
set it using the [`Camera`] [component][cb::component].

```rust,no_run,noplayground
{{#include ../code013/src/window/clear_color.rs:camera}}
```

All of these locations (the components on specific cameras, the global default
resource) can be mutated at runtime, and bevy will use your new color. Changing
the default color using the resource will apply the new color to all existing
cameras that do not specify a custom color, not just newly-spawned cameras.
