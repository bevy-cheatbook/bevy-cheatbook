# I can't see my UI!

If you are trying to build a UI, but it is not showing on the screen, you
probably forgot to spawn a UI Camera. The UI Camera is required for Bevy to
render UI.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:ui-camera}}
```

