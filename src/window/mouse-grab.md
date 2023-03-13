Warning: this page has not been updated for Bevy 0.10 yet!

# Grabbing the Mouse

{{#include ../include/links.md}}

[Click here for the full example code.][cbexample::mouse-grab]

Relevant official examples:
[`mouse_grab`][example::mouse_grab].

---

You can lock/release the mouse cursor using bevy's [window settings
API][example::window_settings].

Here is an example that locks and hides the cursor in the primary window
on [mouse click][input::mouse-button] and releases it when [pressing
`Esc`][input::keyboard]:

```rust,no_run,noplayground
{{#include ../code/examples/mouse-grab.rs:example}}
```
