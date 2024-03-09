{{#include ../include/header09.md}}

# Change Detection

Relevant official examples:
[`component_change_detection`][example::component_change_detection].

---

Bevy allows you to easily detect when data is changed. You can use this to
perform actions in response to changes.

One of the main use cases is optimization – avoiding unnecessary work by
only doing it if the relevant data has changed. Another use case is triggering
special actions to occur on changes, like configuring something or sending
the data somewhere.

## Components

### Filtering

You can make a [query][cb::query] that only yields entities if specific
[components][cb::component] on them have been modified.

Use [query filters][cb::query-filter]:
 - [`Added<T>`][bevy::Added]: detect new component instances
   - if the component was added to an existing entity
   - if a new entity with the component was spawned
 - [`Changed<T>`][bevy::Changed]: detect component instances that have been changed
   - triggers when the component is accessed mutably
   - also triggers if the component is newly-added (as per [`Added`][bevy::Added])

(If you want to react to removals, see the page on [removal
detection][cb::removal-detection]. It works differently and is much
trickier to use.)

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:change-detection}}
```

### Checking

If you want to access all the entities, as normal, regardless of if they have
been modified, but you just want to check the status, you can use the special
[`ChangeTrackers<T>`][bevy::ChangeTrackers] query parameter.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:changetrackers}}
```

This is useful for processing all entities, but doing different things
depending on if they have been modified.

## Resources

For [resources][cb::res], change detection is provided via methods on the
[`Res`][bevy::Res]/[`ResMut`][bevy::ResMut] system parameters.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:changed-res}}
```

Note that change detection cannot currently be used to detect
[states][cb::state] changes (via the [`State`][bevy::State]
[resource][cb::res]) ([bug][bevy::2343]).

## What gets detected?

[`Changed`][bevy::Changed] detection is triggered by
[`DerefMut`]. Simply accessing components via a mutable query,
without actually performing a `&mut` access, will *not* trigger it.

This makes change detection quite accurate. You can rely on it to optimize
your game's performance, or to otherwise trigger things to happen.

Also note that when you mutate a component, Bevy does not track if the new
value is actually different from the old value. It will always trigger the
change detection. If you want to avoid that, simply check it yourself:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:change-if-wrap}}
```

Change detection works on a per-[system][cb::system] granularity, and is
reliable. A system will not detect changes that it made itself, only those
done by other systems, and only if it has not seen them before (the changes
happened since the last time it ran). If your system only runs sometimes
(such as with [states][cb::state] or [run criteria][cb::runcriteria]),
you do *not* have to worry about missing changes.

## Possible Pitfalls

Beware of frame delay / 1-frame-lag. This can occur if Bevy runs the detecting
system before the changing system. The detecting system will see the change
the next time it runs, typically on the next frame update.

If you need to ensure that changes are handled immediately / during the same
frame, you can use [explicit system ordering][cb::system-order].

However, when detecting component additions with [`Added<T>`][bevy::Added]
(which are typically done using [`Commands`][cb::commands]), this is not
enough; you need [stages][cb::stage].

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

You can work around this using [`Option`] and a separate [`Local`][cb::local]
system parameter, effectively implementing your own detection.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:res-removal-detection}}
```

Note that, since this detection is local to your system, it does not have
to happen during the same frame update.
