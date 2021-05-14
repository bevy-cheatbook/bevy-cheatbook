# Custom Camera Projection

[Click here for the full example code.](../code/examples/custom-projection.rs)

---

Camera with a custom projection (not using one of Bevy's standard perspective
or orthographic projections).

You could also use this to change the coordinate system, if you
insist on using something other than [Bevy's default coordinate
system](../features/transforms.md), for whatever reason.

Here we implement a simple orthographic projection that maps `-1.0` to `1.0`
to the vertical axis of the window, and respects the window's aspect ratio
for the horizontal axis:

```rust,no_run,noplayground
{{#include ../code/examples/custom-projection.rs:example}}
```
