{{#include ../include/header09.md}}

# Systems

Relevant official examples:
[`ecs_guide`][example::ecs_guide],
[`startup_system`][example::startup_system],
[`system_param`][example::system_param].

---

Systems are functions you write, which are run by Bevy.

This is where you implement all your game logic.

These functions can only take [special parameter types][builtins::systemparam],
to specify what you need access to. [If you use unsupported parameter types
in your function, you will get confusing compiler errors!][pitfall::intosystem]

Some of the options are:
 - accessing [resources][cb::res] using [`Res`][bevy::Res]/[`ResMut`][bevy::ResMut]
 - accessing [components of entities][cb::component] using [queries][cb::query] ([`Query`][bevy::Query])
 - creating/destroying entities, components, and resources using [Commands][cb::commands] ([`Commands`][bevy::Commands])
 - sending/receiving [events][cb::event] using [`EventWriter`][bevy::EventWriter]/[`EventReader`][bevy::EventReader]

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-debug-res}}
```

System parameters can be grouped into tuples (which can be nested). This is
useful for organization.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-param-tuple}}
```

{{#include ../include/builtins.md:systemparam-limits}}

## Runtime

To run your systems, you need to add them to Bevy via the [app builder][cb::app]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:systems-appbuilder}}
```

The above is enough for simple projects.

As your project grows more complex, you might want to enhance your app builder
with some of the powerful tools that Bevy offers for managing when/how
your systems run, such as: [explicit ordering][cb::system-order] with
[labels][cb::label], [system sets][cb::systemset], [states][cb::state],
[run criteria][cb::runcriteria], and [stages][cb::stage].
