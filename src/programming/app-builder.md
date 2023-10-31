{{#include ../include/header012.md}}

# The App

Relevant official examples: All of them ;)

In particular, check out the complete game examples:
[`alien_cake_addict`][example::alien_cake_addict],
[`breakout`][example::breakout].

---

To enter the Bevy runtime, you need to configure an [`App`][bevy::App]. The app
is how you define the structure of all the things that make up your project:
[plugins][cb::plugin], [systems][cb::system] (and their configuration/metadata:
[run conditions][cb::rc], [ordering][cb::system-order], [sets][cb::systemset]),
[event][cb::event] types, [states][cb::state], [schedules][cb::schedule]…

You typically create your [`App`][bevy::App] in your project's `main` function.
However, you don't have to add everything from there. If you want to add things
to your app from multiple places (like other Rust files or crates), use
[plugins][cb::plugin]. As your project grows, you will need to do that to keep
everything organized.

```rust,no_run,noplayground
{{#include ../code012/src/programming/app_builder.rs:main}}
```

Note: use tuples with `add_systems`/`add_plugins`/`configure_sets` to add
multiple things at once.

[Component][cb::component] types do not need to be registered.

Schedules cannot [(yet)][bevy::279] be modified at runtime; all
[systems][cb::system] you want to run must be added/configured in the
[`App`][bevy::App] ahead of time. You can control individual systems using [run
conditions][cb::rc]. You can also [dynamically enable/disable entire
schedules][cb::schedule-dynamic] using the
[`MainScheduleOrder`][bevy::MainScheduleOrder] [resource][cb::res].

## Builtin Bevy Functionality

The Bevy game engine's own functionality is represented as a [plugin group][cb::plugingroup].
Every typical Bevy app must first add it, using either:
 - [`DefaultPlugins`][bevy::DefaultPlugins] if you are making a full game/app
 - [`MinimalPlugins`][bevy::MinimalPlugins] for something like a headless server.

## Setting up data

Normally, you can set up [your data][cb::ecs-intro-data] from
[systems][cb::system]. Use [Commands][cb::commands] from regular systems, or
use [exclusive systems][cb::exclusive] to get [full World access][cb::world].

Add your setup systems as startup systems for things you want to initialize
at launch, or use [state][cb::state] enter/exit systems to do things when
transitioning between menus, game modes, levels, etc.

However, you can also initialize data directly from the app builder. This
is common for [resources][cb::res], if they need to be present at all
times. You can also get [direct World access][cb::world].

```rust,no_run,noplayground
{{#include ../code012/src/programming/app_builder.rs:world}}
```

## Quitting the App

To cleanly shut down bevy, send an [`AppExit`][bevy::AppExit]
[event][cb::event] from any [system][cb::system]:

```rust,no_run,noplayground
{{#include ../code012/src/programming/app_builder.rs:appexit}}
```

For prototyping, Bevy provides a convenient system you can add, to close the
focused window on pressing the `Esc` key. When all windows are closed, Bevy will
quit automatically.

```rust,no_run,noplayground
{{#include ../code012/src/programming/app_builder.rs:close_on_esc}}
```
