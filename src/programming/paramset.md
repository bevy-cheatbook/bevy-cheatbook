# Param Sets

{{#include ../include/links.md}}

For safety reasons, a [system][cb::system] cannot have multiple parameters
whose data access might have a chance of mutability conflicts over the
same data.

Some examples:
 - Multiple incompatible [queries][cb::query].
 - Using [`&World`][bevy::World] while also having other system parameters to access specific data.
 - …

Bevy provides a solution: wrap them in a [`ParamSet`][bevy::ParamSet]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-query-set}}
```

This ensures only one of the conflicting parameters can be used at the same time.

The maximum number of parameters in a param set is 8.
