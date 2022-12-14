# Local Resources

{{#include ../include/links.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Local resources allow you to have per-[system][cb::system] data. This data
is not stored in the ECS World, but rather together with your system.

[`Local<T>`][bevy::Local] is a system parameter similar to
[`ResMut<T>`][bevy::ResMut], which gives you full mutable access to an
instance of some data type, that is independent from entities and components.

[`Res<T>`][bevy::Res]/[`ResMut<T>`][bevy::ResMut] refer to a single global
instance of the type, shared between all systems. On the other hand, every
[`Local<T>`][bevy::Local] parameter is a separate instance, exclusively for
that system.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:local-resource}}
```

The type must implement [`Default`][std::Default] or
[`FromWorld`][bevy::FromWorld]. It is automatically initialized.

A system can have multiple [`Local`][bevy::Local]s of the same type.

## Specify an initial value

[`Local<T>`][bevy::Local] is always automatically initialized using the
default value for the type.

If you need specific data, you can use a closure instead. Rust closures
that take system parameters are valid Bevy systems, just like standalone
functions. Using a closure allows you to "move data into the function".

This example shows how to initialize some data to configure a system,
without using [`Local<T>`][bevy::Local]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:local-config}}
```

Another way to accomplish the same thing is to "return" the system
from "constructor" helper, that creates it:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:local-config-return}}
```

