# Commands

Use `Commands` to spawn/despawn entities, add/remove components on existing entities, manage resources.

Manage Entities and Components:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:example-commands}}
```

Manage Resources:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:commands-resource}}
```

These actions do not take effect immediately; they are queued to be performed later when it is safe to do so. See: [stages](./stages.md).

(if you are not using stages, that means your systems will see them [on the next frame update](../pitfalls/frame-delay.md))
