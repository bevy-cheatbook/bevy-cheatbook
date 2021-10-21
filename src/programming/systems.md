# Systems

Relevant official examples:
[`ecs_guide`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/ecs_guide.rs),
[`startup_system`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/startup_system.rs),
[`system_param`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/system_param.rs).

---

Systems are functions you write, which are run by Bevy.

This is where you implement all your game logic.

These functions can only take special parameter types, to specify what you
need access to. [If you use unsupported parameter types in your function,
you will get confusing compiler errors!](../pitfalls/into-system.md)

Some of the options are:
 - accessing [resources](./res.md) using `Res`/`ResMut`
 - accessing [components of entities](./ec.md) using [queries](./queries.md)
 - creating/destroying entities, components, and resources using [`Commands`](./commands.md)
 - sending/receiving [events](./events.md) using `EventWriter`/`EventReader`.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-debug-res}}
```

System parameters can be grouped into tuples (which can be nested). This is
useful for organization.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-param-tuple}}
```

The maximum number of top-level system parameters, or tuple members, is 16. You
can group/nest them into tuples, if you need to work around that limitation.

## Runtime

To run your systems, you need to add them to Bevy via the [app builder](./app-builder.md):

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:systems-appbuilder}}
```

The above is enough for simple projects.

As your project grows more complex, you might want to enhance your app builder
with some of the powerful tools that Bevy offers for managing when/how
your systems run, such as: [explicit ordering](./system-order.md) with
[labels](./labels.md), [system sets](./system-sets.md), [states](./states.md),
[run criteria](./run-criteria.md), and [stages](./stages.md).
