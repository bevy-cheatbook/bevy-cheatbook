# App Builder (main function)

Relevant official examples:
[`ecs_guide`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/ecs_guide.rs),
[`hello_world`](https://github.com/bevyengine/bevy/blob/latest/examples/hello_world.rs),
[`empty`](https://github.com/bevyengine/bevy/blob/latest/examples/app/empty.rs),
[`empty_defaults`](https://github.com/bevyengine/bevy/blob/latest/examples/app/empty_defaults.rs),
[`headless`](https://github.com/bevyengine/bevy/blob/latest/examples/app/headless.rs).

---

To enter the bevy runtime, you need to configure an `App`, composed of all
the [systems](./systems.md), [plugins](./plugins.md), [event](./events.md)
types, and [resources](./res.md), that you want to use.

Everything must be registered in the `App`, or it will not work.

Component types and [local resources](./local.md) do not need to be registered.

Resources can also be created later, using [`Commands`](./commands.md).

You also need to add Bevy's built-in functionality: either `DefaultPlugins`
if you are making a full game/app, or `MinimalPlugins` for something like
a headless server.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:app-builder}}
```
