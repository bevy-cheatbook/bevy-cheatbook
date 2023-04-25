{{#include ../include/header09.md}}

# Removal Detection

Relevant official examples:
[`removal_detection`][example::removal_detection].

---

Removal detection is special. This is because, unlike with [change
detection][cb::change-detection], the data does not exist in the ECS anymore
(obviously), so Bevy cannot keep tracking metadata for it.

Nevertheless, being able to respond to removals is important for some
applications, so Bevy offers a limited form of it.

## Components

You can check for [components][cb::component] that have been removed during
the current frame. The data is cleared at the end of every frame update. Note
that this makes this feature tricky to use, and requires you to use multiple
[stages][cb::stage].

When you remove a component (using [Commands][cb::commands]
([`Commands`][bevy::Commands])), the operation is applied at the end of the
[stage][cb::stage]. The [system][cb::system] that checks for the removal
must run in a later stage during the same frame update. Otherwise, it will
not detect the removal.

Use the [`RemovedComponents<T>`][bevy::RemovedComponents] special system
parameter type, to get an iterator for the [`Entity`][bevy::Entity] IDs of
all the entities that had a component of type `T` that was removed earlier
this frame.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:removal-detection}}
```

(To do things with these entities, you can just use the `Entity` IDs with
[`Commands::entity()`][cb::commands] or [`Query::get()`][cb::query].)

## Resources

Bevy does not provide any API for detecting when [resources][cb::res] are removed.

You can work around this using [`Option`][std::Option] and a separate
[`Local`][cb::local] system parameter, effectively implementing your own
detection.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:res-removal-detection}}
```

Note that, since this detection is local to your system, it does not have
to happen during the same frame update.
