# App Builder (main function)

{{#include ../include/links.md}}

Relevant official examples: All of them ;)

In particular, check out the complete game examples:
[`alien_cake_addict`][example::alien_cake_addict],
[`breakout`][example::breakout].

---

To enter the bevy runtime, you need to configure an [`App`][bevy::App],
composed of all the [systems][cb::system], [plugins][cb::plugin],
[event][cb::event] types, and [resources][cb::res], that you want to use.

Everything must be registered in the [`App`][bevy::App], or it will not work.

[Component types][cb::component] and [local resources][cb::local] do not
need to be registered.

Resources can also be created later, using [Commands][cb::commands].

You also need to add Bevy's built-in functionality: either
[`DefaultPlugins`][bevy::DefaultPlugins] if you are making a full game/app, or
[`MinimalPlugins`][bevy::MinimalPlugins] for something like a headless server.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:app-builder}}
```
