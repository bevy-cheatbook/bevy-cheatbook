# Removal Detection

Relevant official examples:
[`removal_detection`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/removal_detection.rs).

---

Removal detection is special. This is because, unlike with [change
detection](./change-detection.md), the data does not exist in the ECS anymore
(obviously), so Bevy cannot keep tracking metadata for it.

Nevertheless, being able to respond to removals is important for some
applications, so Bevy offers a limited form of it.

## Components

You can check for components that have been removed during the current
frame. The data is cleared at the end of every frame update. Note that
this makes this feature tricky to use, and requires you to use multiple
[stages](./stages.md).

When you remove a component (using [`Commands`](./commands.md)), the operation
is applied at the end of the stage. The system that checks for the removal
must run in a later stage during the same frame update. Otherwise, it will
not detect the removal.

Use the `RemovedComponents<T>` special system parameter type, to get an
iterator for the `Entity` IDs of all the entities that had a component of type
`T` that was removed earlier this frame.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:removal-detection}}
```

(To do things with these entities, you can just use the `Entity` IDs with
[`Commands::entity()`](./commands.md) or [`Query::get()`](./queries.md).)

## Resources

Bevy does not provide any API for detecting when resources are removed.

You can work around this using `Option` and a separate [`Local`](./local.md)
system parameter, effectively implementing your own detection.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:res-removal-detection}}
```

Note that, since this detection is local to your system, it does not have
to happen during the same frame update.
