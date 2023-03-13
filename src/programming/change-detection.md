Warning: this page has not been updated for Bevy 0.10 yet!

# Change Detection

{{#include ../include/links.md}}

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
[`DerefMut`][std::DerefMut]. Simply accessing components via a mutable query,
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
