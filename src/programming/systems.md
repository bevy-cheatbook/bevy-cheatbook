# Systems

Systems are functions you write, which are run by Bevy.

This is where you implement all your game logic.

These functions can only take special parameter types, to specify what game data you want to access. [If you use unsupported types in your function, you will get confusing compiler errors!](../pitfalls/into-system.md)

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-debug-res}}
```

System parameters can be grouped into tuples. This is useful for organization.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-param-tuple}}
```

## Performance Advice

Running systems is very fast. Do not be afraid to split your logic into many
small functions. It often actually helps performance, by potentially allowing
for more parallelism.

Having smaller data types (splitting your data into multiple components or
resources, rather than one big `struct`) helps by allowing each system to access
only the data that is relevant to it. The fewer access conflicts, the faster
your game will run.

The general rule of thumb with Bevy performance is: more granular is better.
