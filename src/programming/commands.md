{{#include ../include/header014.md}}

# Commands

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Use [`Commands`] to spawn/despawn entities, add/remove components on existing
entities, manage resources, from your [systems][cb::system].

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:example-commands}}
```

## When do these actions get applied?

[`Commands`] do not take effect immediately, because it wouldn't be safe to
modify the data layout in memory when other [systems][cb::system] could be
running in parallel. When you do anything using [`Commands`], it gets queued to
be applied later when it is safe to do so.

Within the same [schedule][cb::schedule], you can add `.before()`/`.after()`
[ordering constraints][cb::system-order] to your systems, and Bevy will
automatically make sure that Commands get applied in-between if necessary, so
that the second system can see the changes made by the first system.

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:order}}
```

If you do not have explicit ordering dependencies, it is undefined when Commands
will be applied. It is possible that some systems will only see the changes on
the next frame update!

Otherwise, Commands are normally applied at the end of every
[schedule][cb::schedule]. [Systems][cb::system] that live in different schedules
will see the changes. For example, Bevy's engine systems (that live in
[`PostUpdate`]) will see the entities you spawn in your systems (that live in
[`Update`]).

## Custom Commands

Commands can also serve as a convenient way to do any custom manipulations
that require [full access][cb::world] to the ECS [`World`]. You can queue up
any custom code to run in a deferred fashion, the same way as the standard
commands work.

For a one-off thing, you can just pass a closure:

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:command-closure}}
```

If you want something reusable, consider [one-shot systems][cb::oneshot].
They are a way to write regular Bevy systems and run them on-demand.

### Extending the Commands API

If you want something more integrated, that feels like as if it was
part of Bevy's Commands API, here is how to do it.

Create a custom type and implement the [`Command`] trait:

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:command-impl}}
```

And if you want to make it extra nice to use, you can create
an extension trait to add extra methods to [`Commands`]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:command-ext}}
```

Note: if you want to use your custom extension method from other Rust
files, you will have to import your trait, or it will not be available:

```rust,no_run,noplayground
use crate::thing::MyCustomCommandsExt;
```
