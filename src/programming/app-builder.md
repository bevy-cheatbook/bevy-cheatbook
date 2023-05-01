{{#include ../include/header09.md}}

# The App

Relevant official examples: All of them ;)

In particular, check out the complete game examples:
[`alien_cake_addict`][example::alien_cake_addict],
[`breakout`][example::breakout].

---

To enter the bevy runtime, you need to configure an [`App`][bevy::App]. The app
is how you define the structure of all the things that make up your project:
[plugins][cb::plugin], [systems][cb::system], [event][cb::event] types,
[states][cb::state], [stages][cb::stage]…

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:app-builder}}
```

Technically, the [`App`][bevy::App] contains the ECS World(s) (where all
the data is stored) and Schedule(s) (where all the [systems][cb::system]
to run are stored). For advanced use-cases, [Sub-apps][cb::subapp] are a
way to have more than one ECS World and Schedule.

[Local resources][cb::local] do not need to be registered. They are part of
their respective [systems][cb::system].

[Component][cb::component] types do not need to be registered.

---

Schedules cannot (yet) be modified at runtime; all [systems][cb::system] you
want to run must be added/configured in the [`App`][bevy::App] ahead of time.

The data in the ECS World can be modified at any time; create/destroy your
[entities][cb::ec] and [resources][cb::res], from [systems][cb::system]
using [Commands][cb::commands], or [exclusive systems][cb::exclusive] using
[direct World access][cb::world].

[Resources][cb::res] can also be initialized ahead of time, here in the
[`App`][bevy::App] builder.

## Builtin Bevy Functionality

The Bevy game engine's own functionality is represented as a [plugin group][cb::plugingroup].
Every typical Bevy app must first add it, using either:
 - [`DefaultPlugins`][bevy::DefaultPlugins] if you are making a full game/app
 - [`MinimalPlugins`][bevy::MinimalPlugins] for something like a headless server.

## Quitting the App

To cleanly shut down bevy, send an [`AppExit`][bevy::AppExit]
[event][cb::event] from any [system][cb::system]:

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:system}}
```

For prototyping, Bevy provides a convenient system you can add, to close the
focused window on pressing the `Esc` key. When all windows are closed, Bevy will
quit automatically.

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:main}}
```
