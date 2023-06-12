{{#include ../include/header010.md}}

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
can also add one-shot setup and cleanup systems to run when entering or
exiting a state.

To use states, define an enum type and add [systems][cb::systems]
with the [run condition][cb::run-criteria] `in_state`:

```rust,no_run,noplayground
{{#include ../code010/src/programming/states.rs:define}}
```

Adding states also creates several schedules, which execute when transitioning
between states:
 - `OnEnter(<state_enum>)`
 - `OnExit(<state_enum>)`

```rust,no_run,noplayground
{{#include ../code010/src/programming/states.rs:transitions}}
```

## Controlling States

Inside of systems, you can check and control the state using the
[`State<T>`][bevy::State] and [`NextState<T>`][bevy::NextState] resources:

```rust,no_run,noplayground
{{#include ../code010/src/programming/states.rs:check-state}}
```

To change to another state:

```rust,no_run,noplayground
{{#include ../code010/src/programming/states.rs:change-state}}
```

Bevy changes states whenever the `apply_state_transition` system runs, which (by
default) occurs once per frame.

## Known Pitfalls and Limitations

### Events

When receiving [events][cb::event] in systems that don't run all the time, such
as during a pause state, you will miss any events that are sent during the
frames when the receiving systems are not running!

To mitigate this, you could implement a [custom cleanup
strategy][cb::event-manual], to manually manage the lifetime of the relevant
event types.
