# Commands

Use `Commands` to spawn/despawn entities, add/remove components on existing entities, manage resources.

These actions only take effect at the end of the stage, not immediately.

(if you are not using stages, that means your systems will see them [on the next frame update](../pitfalls/frame-delay.md))

Manage Entities and Components:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:example-commands}}
```

Manage Resources:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:commands-resource}}
```
