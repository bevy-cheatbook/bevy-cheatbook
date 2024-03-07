{{#include ../include/header013.md}}

# Commands

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Use [`Commands`][bevy::Commands] to spawn/despawn entities, add/remove
components on existing entities, manage resources, from your [systems][cb::system].

```rust,no_run,noplayground
{{#include ../code013/src/programming/commands.rs:example-commands}}
```

## When do these actions get applied?

[`Commands`][bevy::Commands] do not take effect immediately, because
it wouldn't be safe to modify the data layout in memory when other
[systems][cb::system] could be running in parallel. When you do anything using
[`Commands`][bevy::Commands], it gets queued to be applied later when it is
safe to do so.

Within the same schedule, you can add `.before()`/`.after()` [ordering
constraints][cb::system-order] to your systems, and Bevy will automatically
make sure that Commands get applied in-between if necessary, so that the
second system can see the changes made by the first system.

```rust,no_run,noplayground
{{#include ../code013/src/programming/commands.rs:order}}
```

If you do not have explicit ordering dependencies, it is undefined when
Commands will be applied. It is possible that some systems will only
see the changes on the next frame update!

Otherwise, Commands are normally applied at the end of every
[schedule][cb::schedule]. [Systems][cb::system] that live in different
schedules will see the changes. For example, Bevy's engine systems (that
live in [`PostUpdate`][bevy::PostUpdate]) will see the entities you spawn in your systems (that
live in [`Update`][bevy::Update]).
