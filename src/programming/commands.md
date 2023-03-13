Warning: this page has not been updated for Bevy 0.10 yet!

# Commands

{{#include ../include/links.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Use [`Commands`][bevy::Commands] to spawn/despawn entities, add/remove
components on existing entities, manage resources.

These actions do not take effect immediately; they are queued to be performed
later when it is safe to do so. See: [stages][cb::stage].

(if you are not using stages, that means your other [systems][cb::system]
will see them on the next frame update)

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:example-commands}}
```
