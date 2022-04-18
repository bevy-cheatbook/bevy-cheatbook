# Custom Camera Projection

{{#include ../include/links.md}}

[Click here for the full example code.][cbexample::custom-projection]

---

Camera with a custom projection (not using one of Bevy's standard perspective
or orthographic projections).

You could also use this to change the coordinate system, if you insist on
using something other than [Bevy's default coordinate system][cb::coords],
for whatever reason.

Here we implement a simple orthographic projection that maps `-1.0` to `1.0`
to the vertical axis of the window, and respects the window's aspect ratio
for the horizontal axis:

See how Bevy constructs its camera bundles, for reference:
[orthographic](https://docs.rs/bevy_render/0.7.0/src/bevy_render/camera/bundle.rs.html#73-167),
[perspective](https://docs.rs/bevy_render/0.7.0/src/bevy_render/camera/bundle.rs.html#21-71).

This example is based on the setup for a 2D camera:

```rust,no_run,noplayground
{{#include ../code/examples/custom-projection.rs:example}}
```
