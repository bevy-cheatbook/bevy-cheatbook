# Change Detection

Bevy allows you to easily detect when data is changed. You can use this to
perform actions in response to changes.

## Components

Use [query filters](./queries.md#query-filters):
 - `Added<T>`: detect new component instances
   - if the component was added to an existing entity
   - if a new entity with the component was spawned
 - `Changed<T>`: detect component instances that have been changed
   - triggers when the component is accessed mutably
   - also triggers if the component is newly-added (as per `Added`)

(If you want to react to removals, see the page on [removal
detection](./removal-detection.md). It works differently and is much
trickier to use.)

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:change-detection}}
```

`Changed` detection is triggered by `DerefMut`. Simply accessing components
via a mutable query, without actually performing a `&mut` access, will *not*
trigger it.

This makes change detection quite accurate. You can rely on it to optimize
your game's performance, or to otherwise trigger things to happen.

Also note that when you mutate a component, Bevy does not track if the new
value is actually different from the old value. It will always trigger the
change detection. If you want to avoid that, simply check it yourself:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:change-if-wrap}}
```

Change detection is reliable -- it will detect any changes that
have occured since the last time your detecting system ran. If your
system only runs sometimes (such as with [states](./states.md) or [run
criteria](./run-criteria.md)), you *do not* have to worry about missing
changes.

## Resources

For resources, change detection is provided via methods on the
`Res`/`ResMut` system parameters.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:changed-res}}
```

This works the same way as change detection for components.

## Possible Pitfalls

Beware of [frame delay / 1-frame-lag](../pitfalls/frame-delay.md). This can
occur if Bevy runs the detecting system before the changing system. The
detecting system will see the change the next time it runs, typically on
the next frame update.

If you need to ensure that changes are handled immediately / during the same
frame, you can use [explicit system ordering](./system-order.md).

However, when detecting component additions with `Added<T>` (which are
typically done using [`Commands`](./commands.md)), this is not enough;
you need [stages](./stages.md).
