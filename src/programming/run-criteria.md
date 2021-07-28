# Run Criteria

Run Criteria are a mechanism for controlling if Bevy should run specific
systems, at runtime. This is how you can make functionality that only runs
under certain conditions.

Run Criteria are a lower-level primitive. Bevy provides higher-level
abstractions on top, such as [States](./states.md). You can use Run Criteria
without such abstractions, if you really need more direct control.

Run Criteria can be applied to individual [systems](./systems.md), [system
sets](./system-sets.md), and [stages](./stages.md).

Run Criteria are Bevy systems that return a value of type [`enum
ShouldRun`](https://docs.rs/bevy/0.5.0/bevy/ecs/schedule/enum.ShouldRun.html).
They can accept any system parameters, like a normal system.

This example shows how run criteria might be used to implement different
multiplayer modes:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:run-criteria}}
```

## Run Criteria Labels

If you have multiple systems or system sets that you want to share the same
run criteria, you can give it a [label](./labels.md).

When you use a label, Bevy will only execute the run criteria once, remember
its output, and apply it to everything with the label.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:run-criteria-label}}
```

The run-once property is especially important if you have a complex run
criteria system that performs mutations or is otherwise non-idempotent.

## Known Pitfalls

When receiving [events](./events.md) in systems that don't run every frame,
you will miss any events that are sent during the frames when the receiving
systems are not running!

To mitigate this, you could implement a [custom cleanup
strategy](../patterns/manual-event-clear.md), to manually manage the lifetime
of the relevant event types.

---

Bevy's [fixed timestep](../features/fixed-timestep.md) is also implemented
using run criteria under the hood.
