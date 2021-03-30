# Change Detection

Bevy allows you to easily detect when components are changed.

Use query filters:
 - `Added<T>`: detect adding new components to existing entities
 - `Changed<T>`: detect when a component is modified

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:change-detection}}
```

Change detection is triggered by `DerefMut`. Simply accessing components via a
mutable query, without it actually being a `&mut` access, will *not* trigger it.

This makes change detection quite accurate. You can rely on it to optimize
your game's performance, or to otherwise trigger things to happen.

Also note that when you mutate a component, Bevy does not track if the new value
is actually different from the old value. It will always trigger the change
detection. If you want to avoid that, simply check it yourself:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:change-if-wrap}}
```

## Possible Pitfalls

Beware of [frame delay / 1-frame-lag](../pitfalls/frame-delay.md). This can
occur if Bevy runs the detecting system before the changing system. The
detecting system will see the change the next time it runs, typically on the
next frame update.

If you need to ensure that changes are handled immediately / during the same frame,
you can use [explicit system ordering](./system-order.md).

However, when detecting component additions with `Added<T>` (which are typically
done using [`Commands`](./commands.md)), this is not enough; you need [stages](./stages.md).
