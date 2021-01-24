# App Builder (main function)

To enter the bevy runtime, build an `App`, composed of all the plugins, event types, resources, and systems, that you want to use.

Everything must be registered in the `App`, or it will not work.

You also need to add bevy's builtin functionality: either `DefaultPlugins` if
you are making a full game/app, or `MinimalPlugins` for something like a
headless server.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:app-builder}}
```
