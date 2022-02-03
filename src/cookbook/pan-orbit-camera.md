# Pan + Orbit Camera

{{#include ../include/links.md}}

[Click here for the full example code.][cbexample::pan-orbit-camera.rs]

---

This code is a community contribution.

Current version developed by [**@mirenbharta**][cb::1].
Initial work by [**@skairunner**][cb::cookbook::2].

---

This is a camera controller similar to the ones in 3D editors like Blender.

Use the right mouse button to rotate, middle button to pan, scroll wheel to
move inwards/outwards.

This is largely shown for illustrative purposes, as an example
to learn from. In your projects, you may want to try the
[`bevy_config_cam`][project::bevy_config_cam] plugin.

```rust,no_run,noplayground
{{#include ../code/examples/pan-orbit-camera.rs:example}}
```
