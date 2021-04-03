# Quitting the App

[Click here for the full example code.](../code/examples/quit.rs)

---

To cleanly shut down bevy, send an `AppExit` event from any system:

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:system}}
```

For prototyping, bevy provides a system you can add, to exit on pressing the Esc key:

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:main}}
```

