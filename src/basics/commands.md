# Commands

Use `Commands` to spawn/despawn entities, add/remove components on existing entities, insert resources.

These actions only take effect at the end of the stage, not immediately.

(if you are not using stages, that means your systems will see them the next frame, not the current)

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:example-commands}}
```
