# Commands

Use `Commands` to spawn/despawn entities, add/remove components on existing entities, manage resources.

These actions do not take effect immediately; they are queued to be performed later when it is safe to do so. See: [stages](./stages.md).

(if you are not using stages, that means your systems will see them [on the next frame update](../pitfalls/frame-delay.md))

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:example-commands}}
```

Be careful not to confuse components and bundles. For example: `.insert_bundle`
is correct: it will add all the components from the bundle; if you instead use
`.insert` with a bundle type, the bundle struct will be added as a single
component!
