{{#include ../include/header010.md}}

# Run Conditions

Run Conditions are a mechanism for controlling if Bevy should run specific
[systems][cb::system], at runtime. This is how you can make functionality
that only runs under certain conditions.

Run Conditions can be applied to individual [systems][cb::system], and [system
sets][cb::systemset].

Run Conditions are Bevy systems that return a bool. They can accept any [system
parameters][builtins::systemparam], like a normal system.

This example shows how run conditions might be used to implement different
multiplayer modes:

```rust,no_run,noplayground
{{#include ../code010/src/programming/run_conditions.rs:conditions}}
```

Run conditions can (sometimes) provide light optimization benefits, as
conditions are only evaluated once each schedule update, and failed conditions
do not schedule systems as tasks.

## Combining conditions

Run conditions can be combined using `not`, `and_then`, and `or_else`. These can
be nested however you would like to allow for fairly complex conditions.

```rust,no_run,noplayground
{{#include ../code010/src/programming/run_conditions.rs:combine}}
```

## Known Pitfalls

### Events

When receiving [events][cb::event] in systems that don't run every frame,
you will miss any events that are sent during the frames when the receiving
systems are not running!

To mitigate this, you could implement a [custom cleanup
strategy][cb::event-manual], to manually manage the lifetime of the relevant
event types.
