# Run Criteria

{{#include ../include/links.md}}

Run Criteria are a mechanism for controlling if Bevy should run specific
[systems][cb::system], at runtime. This is how you can make functionality
that only runs under certain conditions.

Run Criteria are a lower-level primitive. Bevy provides higher-level
abstractions on top, such as [States][cb::state]. You can use Run Criteria
without such abstractions, if you really need more direct control.

Run Criteria can be applied to individual [systems][cb::system], [system
sets][cb::systemset], and [stages][cb::stage].

Run Criteria are Bevy systems that return a value of type [`enum
ShouldRun`][bevy::ShouldRun]. They can accept any [system
parameters][builtins::systemparam], like a normal system.

This example shows how run criteria might be used to implement different
multiplayer modes:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:run-criteria}}
```

## Run Criteria Labels

If you have multiple systems or system sets that you want to share the same
run criteria, you can give it a [label][cb::label].

When you use a label, Bevy will only execute the run criteria once, remember
its output, and apply it to everything with the label.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:run-criteria-label}}
```

The run-once property is especially important if you have a complex run
criteria system that performs mutations or is otherwise non-idempotent.

## Known Pitfalls

When receiving [events][cb::event] in systems that don't run every frame,
you will miss any events that are sent during the frames when the receiving
systems are not running!

To mitigate this, you could implement a [custom cleanup
strategy][cb::event-manual], to manually manage the lifetime of the relevant
event types.

---

Bevy's [fixed timestep][cb::fixedtimestep] is also implemented
using run criteria under the hood.
