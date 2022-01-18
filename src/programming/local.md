# Local Resources

{{#include ../include/links.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Local resources allow you to have per-[system][cb::system] data.

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

You can initialize a [`Local`][bevy::Local] to a value other than the
type's default, using `.config`, when [adding your system][cb::app] to your
[`App`][bevy::App].

`.config` is Bevy's API for "configuring" specific system parameters. Most
other types of system parameters do not support configuration, but
[`Local`][bevy::Local]s let you specify the initial value.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:local-config}}
```
