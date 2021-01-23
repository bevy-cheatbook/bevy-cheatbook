# Custom Camera Projection

[Click here for the full example code.](../code_bevy_release/examples/custom-projection.rs)

---

Camera with a custom projection (not using one of bevy's standard perspective or orthographic projections).

Here we implement a simple orthographic projection that maps `-1.0` to `1.0` to the vertical axis of the window,
and respects the window's aspect ratio for the horizontal axis:

```rust,no_run,noplayground
{{#include ../code_bevy_release/examples/custom-projection.rs:example}}
```
