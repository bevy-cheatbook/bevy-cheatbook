# App Builder (main function)

To enter the bevy runtime, you need to configure an `App`, composed of all the
systems, plugins, event types, and resources, that you want to use.

Everything must be registered in the `App`, or it will not work.

Component types and local resources do not need to be registered.

Resources can also be created later, using [`Commands`](./commands.md).

You also need to add Bevy's built-in functionality: either `DefaultPlugins` if
you are making a full game/app, or `MinimalPlugins` for something like a
headless server.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:app-builder}}
```
