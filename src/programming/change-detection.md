{{#include ../include/header013.md}}

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
 - [`Added<T>`]: detect new component instances
   - if the component was added to an existing entity
   - if a new entity with the component was spawned
 - [`Changed<T>`]: detect component instances that have been changed
   - triggers when the component is mutated
   - also triggers if the component is newly-added (as per [`Added`])

(If you want to react to removals, see [removal
detection][cb::removal-detection]. It works differently.)

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:change-detection}}
```

### Checking

If you want to access all the entities, as normal, regardless of if they have
been modified, but you just want to know if a component has been changed,
you can use special [`Ref<T>`] query parameters instead of `&` for immutable access.

For mutable access, the change detection methods are always available (because
Bevy queries actually return a special [`Mut<T>`] type whenever you have `&mut`
in the query).

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:ref}}
```

## Resources

For [resources][cb::res], change detection is provided via methods on the
[`Res<T>`]/[`ResMut<T>`] system parameters.

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:changed-res}}
```

## What gets detected?

[`Changed`] detection is triggered by [`DerefMut`]. Simply accessing
[components][cb::component] via a mutable [query][cb::query], or
[resources][cb::res] via [`ResMut`], without actually performing a `&mut`
access, will *not* trigger it. This makes change detection quite accurate.

Note: if you call a Rust function that takes a `&mut T` (mutable borrow),
that counts! It will trigger change detection even if the function does
not actually do any mutation. Be careful with helper functions!

Also, when you mutate a component, Bevy does not check if the new value
is actually different from the old value. It will always trigger the change
detection. If you want to avoid that, simply check it yourself:

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:change-if-wrap}}
```

Change detection works on a per-[system][cb::system] granularity, and is
reliable. A system will detect changes only if it has not seen them before
(the changes happened since the last time it ran).

Unlike [events][cb::event], you do *not* have to worry about missing changes
If your system only runs sometimes (such as when using [states][cb::state]
or [run conditions][cb::rc]).

## Possible Pitfalls

Beware of frame delay / 1-frame-lag. This can occur if Bevy runs the detecting
system before the changing system. The detecting system will see the change
the next time it runs, typically on the next frame update.

If you need to ensure that changes are handled immediately / during the same
frame, you can use [explicit system ordering][cb::system-order].

---

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

You can check for [components][cb::component] that have been removed during the
current frame. The data is cleared at the end of every frame update. You must
make sure your detecting [system][cb::system] [is ordered after][cb::system-order]
(or is in another [schedule][cb::schedule] that runs after) the system that
does the removing.

Note: removal detection also includes despawned entities!

Use the [`RemovedComponents<T>`] special system parameter type. Internally, it
is implemented using [events][cb::event] and behaves like an [`EventReader`],
but it gives you the [`Entity`] IDs of entities whose component `T` was removed.

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:removal-detection}}
```

(To do things with these entities, you can just use the `Entity` IDs with
[`Commands::entity()`][cb::commands] or [`Query::get()`][cb::query].)

## Resources

Bevy does not provide any API for detecting when [resources][cb::res] are removed.

You can work around this using [`Option`] and a separate [`Local`][cb::local]
system parameter, effectively implementing your own detection.

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:res-removal-detection}}
```

Note that, since this detection is local to your system, it does not have
to happen during the same frame update.
