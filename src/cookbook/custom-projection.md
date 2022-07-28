# Custom Camera Projection

{{#include ../include/links.md}}

[Click here for the full example code.][cbexample::custom-projection]

---

**Note**: this example is showing you how to do something not officially
supported/endorsed by Bevy. Do at your own risk.

Camera with a custom projection (not using one of Bevy's standard perspective
or orthographic projections).

You could also use this to change the coordinate system, if you insist on
using something other than [Bevy's default coordinate system][cb::coords],
for whatever reason.

Here we implement a simple orthographic projection that maps `-1.0` to `1.0`
to the vertical axis of the window, and respects the window's aspect ratio
for the horizontal axis:

See how Bevy constructs its camera bundles, for reference:
 - [2d](https://github.com/bevyengine/bevy/blob/v0.8.0/crates/bevy_core_pipeline/src/core_2d/camera_2d.rs#L47)
 - [3d](https://github.com/bevyengine/bevy/blob/v0.8.0/crates/bevy_core_pipeline/src/core_3d/camera_3d.rs#L72)

This example is based on the setup for a 2D camera:

```rust,no_run,noplayground
{{#include ../code/examples/custom-projection.rs:example}}
```
