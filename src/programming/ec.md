{{#include ../include/header011.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

# Entities

Entities are just a [simple integer ID][bevy::Entity], that identifies a particular set of
component values.

[See here if you need help understanding how storing data in the ECS works.][cb::ecs-intro-data]

You can create ("spawn") new entities using [`Commands`][cb::commands] or
[exclusive `World` access][cb::world].

# Components

Components are the data associated with entities.

To create a new component type, simply define a Rust `struct` or `enum`, and
derive the [`Component`][bevy::Component] trait.

```rust,no_run,noplayground
{{#include ../code011/src/programming/ec.rs:component}}
```

Types must be unique -- an entity can only have one component per Rust type.

## Newtype Components

Use wrapper (newtype) structs to make unique components out of simpler types:

```rust,no_run,noplayground
{{#include ../code011/src/programming/ec.rs:component-newtype}}
```

## Marker Components

You can use empty structs to help you identify specific entities. These are
known as "marker components". Useful with [query filters][cb::query-filter].

```rust,no_run,noplayground
{{#include ../code011/src/programming/ec.rs:component-marker}}
```

## Accessing Components

Components can be accessed from [systems][cb::system], using [queries][cb::query].

## Adding/removing Components

You can add/remove components on existing entities, using [`Commands`][cb::commands] or
[exclusive `World` access][cb::world].
