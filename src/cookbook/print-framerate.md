# Show Framerate in Console

[Click here for the full example code.](../code_bevy_release/examples/print-framerate.rs)

---

You can use bevy's builtin diagnostics system to print framerate (FPS) to the console, for monitoring performance.

*Note (git master)*: `PrintDiagnosticsPlugin` was renamed to `LogDiagnosticsPlugin`.

```rust,no_run,noplayground
{{#include ../code_bevy_release/examples/print-framerate.rs:main}}
```
