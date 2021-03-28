# States

States allow you to structure the runtime "flow" of your app.

You can have different systems running, depending on the state.

You can add systems that only run when transitioning between states.

States are built using stages.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:app-states}}
```

Inside of systems, you can check and control the state using the `State<T>` resource:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:check-state}}
```

To change to another state:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:change-state}}
```

After the systems of the current state complete, bevy will transition to the next state you set.

Note: you can do arbitrarily many state transitions in a single frame. Bevy will
handle all of them and execute all the relevant systems before moving on to the
next stage in the app schedule.
