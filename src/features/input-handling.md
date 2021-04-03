# Input Handling

[Click here to download the example code.](../code/examples/input.rs)

---

Since bevy resources and events are global and accessible from any system, you
don't need to centralize your input handling code in one place. You can consume
any relevant inputs wherever you need them.

If you simply need to check the current state of specific keyboard keys or mouse buttons:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:res-input}}
```

For more powerful input handling, to detect all activity, use input events:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:event-input}}
```
