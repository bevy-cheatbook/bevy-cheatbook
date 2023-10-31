{{#include ../include/header012.md}}

# Param Sets

For safety reasons, a [system][cb::system] cannot have multiple parameters
whose data access might have a chance of mutability conflicts over the
same data.

Some examples:
 - Multiple incompatible [queries][cb::query].
 - Using [`&World`][bevy::World] while also having other system parameters to access specific data.
 - …

Such code will compile (Rust cannot know about Bevy ECS semantics), but will
result in a runtime panic. When Bevy tries to run the system, it will panic with
a message about conflicting system parameters:

Bevy provides a solution: wrap them in a [`ParamSet`][bevy::ParamSet]:

```rust,no_run,noplayground
{{#include ../code012/src/programming/paramset.rs:paramset}}
```

This ensures only one of the conflicting parameters can be used at the same time.

The maximum number of parameters in a param set is 8.
