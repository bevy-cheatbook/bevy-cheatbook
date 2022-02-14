# Touchscreen

{{#include ../include/links.md}}

Relevant official examples:
[`touch_input`][example::touch_input],
[`touch_input_events`][example::touch_input_events].

---

Multi-touch touchscreens are supported. You can track multiple fingers on
the screen, with position and pressure/force information. Bevy does not
offer gesture recognition.

The [`Touches`][bevy::Touches] [resource][cb::res] allows you to track any
fingers currently on the screen:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:touches}}
```

Alternatively, you can use [`TouchInput`][bevy::TouchInput] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:touch-events}}
```
