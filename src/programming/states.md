{{#include ../include/header013.md}}

# States

Relevant official examples:
[`state`][example::state].

---

States allow you to structure the runtime "flow" of your app.

This is how you can implement things like:
 - A menu screen or a loading screen
 - Pausing / unpausing the game
 - Different game modes
 - …

In every state, you can have different [systems][cb::system] running. You
can also add setup and cleanup systems to run when entering or exiting a state.

---

To use states, first define an `enum` type. You need to derive
[`States`][bevy::States] + an assortment of required standard Rust traits:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:definition}}
```

Note: you can have multiple orthogonal states! Create multiple types
if you want to track multiple things independently!

You then need to register the state type(s) in the [app builder][cb::app]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:app-register}}
```

## Running Different Systems for Different States

If you want some [systems][cb::system] to only run in specific states,
Bevy offers an [`in_state`][bevy::in_state] [run condition][cb::rc]. Add it
to your systems. You probably want to create [system sets][cb::systemset]
to help you group many systems and control them at once.

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:app-example}}
```

Bevy also creates special [`OnEnter`][bevy::OnEnter], [`OnExit`][bevy::OnExit],
and [`OnTransition`][bevy::OnTransition] [schedules][cb::schedule] for each
possible value of your state type. Use them to perform setup and cleanup for
specific states. Any systems you add to them will run once every time the state
is changed to/from the respective values.

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:app-example-transitions}}
```

### With Plugins

This can also be useful with [Plugins][cb::plugin]. You can set up all the state
types for your project in one place, and then your different plugins can just add
their systems to the relevant states.

You can also make plugins that are configurable, so that it is possible to specify
what state they should add their systems to:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:plugin-config}}
```

Now you can configure the plugin when adding it to the app:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:plugin-config-app}}
```

When you are just using [plugins][cb::plugin] to help with internal
organization of your project, and you know what systems should go into each
state, you probably don't need to bother with making the plugin configurable
as shown above. Just hardcode the states / add things to the correct states
directly.

## Controlling States

Inside of systems, you can check the current state using the
[`State<T>`][bevy::State] [resource][cb::res]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:check-state}}
```

To change to another state, you can use the [`NextState<T>`][bevy::NextState]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:change-state}}
```

This will queue up state transitions to be performed during the next frame
update cycle.

## State Transitions

Every frame update, a [schedule][cb::schedule] called
[`StateTransition`][bevy::StateTransition] runs. There, Bevy will check if
any new state is queued up in [`NextState<T>`][bevy::NextState] and perform
the transition for you.

The transition involves several steps:
 - A [`StateTransitionEvent`][bevy::StateTransitionEvent] [event][cb::event] is sent.
 - The [`OnExit(old_state)`][bevy::OnExit] [schedule][cb::schedule] is run.
 - The [`OnTransition { from: old_state, to: new_state }`][bevy::OnTransition] [schedule][cb::schedule] is run.
 - The [`OnEnter(new_state)`][bevy::OnEnter] [schedule][cb::schedule] is run.

[`StateTransitionEvent`][bevy::StateTransitionEvent] is useful in any
[systems][cb::system] that run regardless of state, but want to know if a
transition has occurred. You can use it to detect state transitions.

The [`StateTransition`][bevy::StateTransition] [schedule][cb::schedule] runs
after [`PreUpdate`][bevy::PreUpdate] (which contains Bevy engine internals),
but before [`FixedMain`][bevy::FixedMain] ([fixed timestep][cb::fixedtimestep])
and [`Update`][bevy::Update], where your game's [systems][cb::system]
usually live.

Therefore, state transitions happen before your game logic for the current frame.

If doing state transitions once per frame is not enough for
you, you can add additional transition points, by adding Bevy's
[`apply_state_transition`][bevy::apply_state_transition] [system][cb::system]
wherever you like.

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:app-custom-transition}}
```

## Known Pitfalls

### System set configuration is per-schedule!

This is the same general caveat that applies any time you configure [system sets][cb::set].

Note that `app.configure_sets()` is *per-[schedule][cb::schedule]!* If you configure some sets
in one [schedule][cb::schedule], that configuration does not carry over to other schedules.

Because states are so schedule-heavy, you have to be especially careful. Don't assume
that just because you configured a set, you can use it anywhere.

For example, your sets from [`Update`][bevy::Update]
and [`FixedUpdate`][bevy::FixedUpdate] will not work in
[`OnEnter`][bevy::OnEnter]/[`OnExit`][bevy::OnExit] for your various state
transitions.

### Events

This is the same general caveat that applies to any [systems][cb::system] with
[run conditions][cb::rc] that want to receive [events][cb::event].

When receiving [events][cb::event] in systems that don't run all the time, such
as during a pause state, you will miss any events that are sent while when
the receiving systems are not running!

To mitigate this, you could implement a [custom cleanup
strategy][cb::event-manual], to manually manage the lifetime of the relevant
event types.
