# States

States allow you to structure the runtime "flow" of your app.

This is how you can implement things like:
 - A menu screen or a loading screen
 - Pausing / unpausing the game
 - Different game modes
 - ...

In every state, you can have different systems running. You can also add
one-shot setup and cleanup systems to run when entering or exiting a state.

To use states, define an enum type and add [system sets](./system-sets.md)
to your [app builder](./app-builder.md):

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:app-states}}
```

It is OK to have multiple system sets for the same state.

This is useful when you want to place [labels](./labels.md) and use [explicit
system ordering](./system-order.md).

This can also be useful with [Plugins](./plugins.md). Each plugin can add
its own set of systems to the same state.

States are implemented using [run criteria](./run-criteria.md) under the hood.
These special system set constructors are really just helpers to automatically
add the state management run criteria.

## Controlling States

Inside of systems, you can check and control the state using the `State<T>`
resource:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:check-state}}
```

To change to another state:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:change-state}}
```

After the systems of the current state complete, Bevy will transition to
the next state you set.

You can do arbitrarily many state transitions in a single frame update. Bevy
will handle all of them and execute all the relevant systems (before moving
on to the next [stage](./stages.md)).

(if you are curious about how this is implemented, see [Looping Run
Criteria](./run-criteria-loop.md))

## State Stack

Instead of completely transitioning from one state to another, you can also
overlay states, forming a stack.

This is how you can implement things like a "game paused" screen, or an
overlay menu, with the game world still visible / running in the background.

You can have some systems that are still running even when the state is
"inactive" (that is, in the background, with other states running on top). You
can also add one-shot systems to run when "pausing" or "resuming" the state.

In your [app builder](./app-builder.md):

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:state-stack}}
```

To manage states like this, use `push`/`pop`:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:state-push-pop}}
```

(using `.set` as shown before replaces the active state at the top of the stack)

## Known Pitfalls and Limitations

### Events

When receiving [events](./events.md) in systems that don't run all the time, such
as during a pause state, you will miss any events that are sent during the frames
when the receiving systems are not running!

To mitigate this, you could implement a [custom cleanup
strategy](../patterns/manual-event-clear.md), to manually manage the lifetime
of the relevant event types.

### Combining with Other Run Criteria

Because states are implemented using run criteria, it is tricky
to combine them with other uses of run criteria, such as [fixed
timestep](../features/fixed-timestep.md).

If you try to add another run criteria to your system set, it would replace
Bevy's state-management run criteria! This would make the system set no
longer constrained to run as part of a state!

It may still be possible to accomplish such use cases using some trickery.

(TODO) show an example of how it could be done.

### With Input

If you want to use `Input<T>` to trigger state transitions using a button/key
press, you need to clear the input manually by calling `.reset`:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:state-input-clear}}
```

(note that this requires `ResMut`)

Not doing this can cause [issues](https://github.com/bevyengine/bevy/issues/1700).

### Multiple Stages

(TODO) move this under the [Advanced Patterns](../patterns/_index.md) chapter.

If you need state-dependent systems in multiple [stages](./stages.md),
a workaround is required.

You must add the state to one stage only, and then call `.get_driver()`
and add that to the other stages before any state-dependent system sets in
those stages.

(TODO) example
