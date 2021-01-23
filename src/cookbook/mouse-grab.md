# Grabbing the Mouse

[Click here for the full example code.](../code_bevy_release/examples/mouse-grab.rs)

---

You can lock/release the mouse cursor using bevy's [window settings API](https://github.com/bevyengine/bevy/blob/master/examples/window/window_settings.rs).

Here is an example that locks and hides the cursor in the primary window on mouse click and releases it when pressing Esc:

```rust,no_run,noplayground
{{#include ../code_bevy_release/examples/mouse-grab.rs:example}}
```
