{{#include ../include/header013.md}}

# List All Resource Types

This example shows how to print a list of all types that have been added as
[resources][cb::res].

```rust,no_run,noplayground
{{#include ../code013/examples/print-resources.rs:example}}
```

```rust,no_run,noplayground
{{#include ../code013/examples/print-resources.rs:app}}
```

It lists the types of all the resources *that currently exist* in your [ECS
World][cb::ecs-intro-data] (by all registered plugins, your own, etc.).

Note that this does *not* give you a list of every type that is
useful as a resource. For that, you should consult API documentation,
looking for implementers of the [`Resource`][bevy::Resource] trait.

[See here for a summary of types provided in Bevy.][chapter::builtins]

