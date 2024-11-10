{{#include ../include/header014.md}}

# The App

Relevant official examples: All of them ;)

In particular, check out the complete game examples:
[`alien_cake_addict`][example::alien_cake_addict],
[`breakout`][example::breakout].

---

To enter the Bevy runtime, you need to configure an [`App`]. The app is how you
define the structure of all the things that make up your project:
[plugins][cb::plugin], [systems][cb::system] (and their configuration/metadata:
[run conditions][cb::rc], [ordering][cb::system-order], [sets][cb::systemset]),
[event][cb::event] types, [states][cb::state], [schedules][cb::schedule]…

You typically create your [`App`] in your project's `main` function.  However,
you don't have to add everything from there. If you want to add things to your
app from multiple places (like other Rust files or crates), use
[plugins][cb::plugin]. As your project grows, you will need to do that to keep
everything organized.

```rust,no_run,noplayground
{{#include ../code014/src/programming/app_builder.rs:main}}
```

Note: use tuples with `add_systems`/`add_plugins`/`configure_sets` to add
multiple things at once.

[Component][cb::component] types do not need to be registered.

Schedules cannot (yet) be modified at runtime; all [systems][cb::system] you
want to run must be added/configured in the [`App`] ahead of time. You can
control individual systems using [run conditions][cb::rc]. You can also
dynamically enable/disable entire schedules using the [`MainScheduleOrder`]
[resource][cb::res].

## Builtin Bevy Functionality

The Bevy game engine's own functionality is represented as a [plugin group][cb::plugingroup].
Every typical Bevy app must first add it, using either:
 - [`DefaultPlugins`] if you are making a full game/app.
 - [`MinimalPlugins`] for something like a headless server.

## Setting up data

Normally, you can set up [your data][cb::ecs-intro-data] from
[systems][cb::system]. Use [Commands][cb::commands] from regular systems, or
use [exclusive systems][cb::exclusive] to get [full World access][cb::world].

Add your setup systems to the [`Startup`] [schedule][cb::schedule] for
things you want to initialize at launch, or use [state][cb::state] enter/exit
systems to do things when transitioning between menus, game modes, levels, etc.

However, you can also initialize data directly from the app builder. This
is common for [resources][cb::res], if they need to be present at all
times. You can also get [direct World access][cb::world].

```rust,no_run,noplayground
{{#include ../code014/src/programming/app_builder.rs:world}}
```

## Quitting the App

To cleanly shut down bevy, send an [`AppExit`] [event][cb::event] from any
[system][cb::system]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/app_builder.rs:appexit}}
```

In a real app, you could do this from various places, such as
a handler for an "Exit" button in your main menu, etc.

You can specify the exit code to return to the OS. If Bevy receives
multiple [`AppExit`] events, success will only be returned if all
of them report success. If some report an error, the last event will
determine the actual exit code of the process.
