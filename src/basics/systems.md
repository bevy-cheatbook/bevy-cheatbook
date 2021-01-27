# Systems

Systems are functions you write, which are run by Bevy.

This is where you implement all your game logic.

These functions can only take special parameter types, to specify what game data you want to access.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:sys-debug-res}}
```

System parameters can be grouped into tuples. This is useful for organization,
or to overcome the limits on the maximum number of parameters (if you get a
compiler error after adding more parameters to your system):

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:sys-param-tuple}}
```
