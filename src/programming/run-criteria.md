{{#include ../include/header012.md}}

# Run Criteria

Run Criteria are a mechanism for controlling if Bevy should run specific
[systems][cb::system], at runtime. This is how you can make functionality
that only runs under certain conditions.

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

## Known Pitfalls

### Combining Multiple Run Criteria

It is not possible to make a system that is conditional on multiple run
criteria. Bevy has a `.pipe` method that allows you to "chain" run criteria,
which could let you modify the output of a run criteria, but this is very
limiting in practice.

Consider using [`iyes_loopless`][project::iyes_loopless]. It allows you to
use any number of run conditions to control your systems, and does not prevent
you from using [states][cb::state] or [fixed timestep][cb::fixedtimestep].

### Events

When receiving [events][cb::event] in systems that don't run every frame,
you will miss any events that are sent during the frames when the receiving
systems are not running!

To mitigate this, you could implement a [custom cleanup
strategy][cb::event-manual], to manually manage the lifetime of the relevant
event types.
