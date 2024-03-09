{{#include ../include/header013.md}}

# Local Resources

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Local resources allow you to have per-[system][cb::system] data. This data
is not stored in the ECS World, but rather together with your system.
Nothing outside of your system can access it. The value will be kept across
subsequent runs of the system.

[`Local<T>`] is a system parameter similar to [`ResMut<T>`], which gives you
full mutable access to an instance of some data type, that is independent from
entities and components.

[`Res<T>`]/[`ResMut<T>`] refer to a single global instance of the type, shared
between all systems. On the other hand, every [`Local<T>`] parameter is a
separate instance, exclusively for that system.

```rust,no_run,noplayground
{{#include ../code013/src/programming/local.rs:local-resource}}
```

The type must implement [`Default`] or [`FromWorld`]. It is automatically
initialized.  It is not possible to specify a custom initial value.

A system can have multiple [`Local`]s of the same type.

## Specify an initial value

[`Local<T>`] is always automatically initialized using the default value for the
type. If that doesn't work for you, there is an alternative way to pass data
into a system.

If you need specific data, you can use a closure instead. Rust closures that
take system parameters are valid Bevy systems, just like standalone functions.
Using a closure allows you to "move data into the function".

This example shows how to initialize some data to configure a system, without
using [`Local<T>`]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/local.rs:closure}}
```

Another way to accomplish the same thing is to "return" the system from
"constructor" helper function that creates it:

```rust,no_run,noplayground
{{#include ../code013/src/programming/local.rs:constructor}}
```
