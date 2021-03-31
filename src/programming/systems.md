# Systems

Systems are functions you write, which are run by Bevy.

This is where you implement all your game logic.

These functions can only take special parameter types, to specify what game data
you want to access. [If you use unsupported types in your function, you will get
confusing compiler errors!](../pitfalls/into-system.md)

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-debug-res}}
```

System parameters can be grouped into tuples (which can be nested). This is
useful for organization.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-param-tuple}}
```

The maximum number of top-level system parameters, or tuple members, is 11. You
can group/nest them into tuples, if you need to work around that limitation.
