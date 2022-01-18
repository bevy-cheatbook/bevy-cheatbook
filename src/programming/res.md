# Resources

{{#include ../include/links.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Resources allow you to store a single global instance of some data type,
independently of entities.

Use them for data that is truly global for your app, such as configuration
/ settings.

Any Rust type (`struct` or `enum`) can be used as a resource. Currently,
no special trait or derive is required, but that may change in future Bevy
versions (similar to how [Components][cb::component] require it).

Types must be unique; there can only be one instance of a given type.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:resource}}
```

Resources can be accessed from [systems][cb::system], using `Res`/`ResMut`.

## Resource Initialization

Implement `Default` for simple resources:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:resource-default}}
```

For resources that need complex initialization, implement `FromWorld`:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:fromworld}}
```

You can initialize your resources at [`App` creation][cb::app]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:appinit-resource}}
```

[`Commands`][cb::commands] can be used to create/remove resources from
inside a system:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:commands-resource}}
```

If you insert a resource of a type that already exists, it will be overwritten.

## Usage Advice

The choice of when to use entities/components vs. resources is typically
about how you want to access the data: globally from anywhere (resources),
or using ECS patterns (entities/components).

Even if there is only one of a certain thing in your game (such as the
player in a single-player game), it can be a good fit to use an entity
instead of resources, because entities are composed of multiple components,
some of which can be common with other entities. This can make your game
logic more flexible. For example, you could have a "health/damage system"
that works with both the player and enemies.
