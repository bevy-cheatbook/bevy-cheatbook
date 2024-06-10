{{#include ../include/header014.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

# Entities

[See here for more explanation on how storing data in the ECS works.][cb::ecs-intro-data]

Conceptually, an entity represents a set of values for different components.
Each component is a Rust type (`struct` or `enum`) and an entity can be used to
store a value of that type.

Technically, an entity is just a simple integer ID (imagine the "row number" in
a table/spreadsheet) that can be used to find related data values (in different
"columns" of that table).

In Bevy, [`Entity`] is this value. It consists of two integers:
the ID and the "generation" (allowing IDs to be reused, after you despawn old
entities).

You can create ("spawn") new entities and destroy ("despawn") entities using
[`Commands`][cb::commands] or [exclusive `World` access][cb::world].

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:spawn-despawn}}
```

Many of your entities might need to have the same common components. You can use
[Bundles][cb::bundle] to make it easier to spawn your entities.

# Components

Components are the data associated with entities.

To create a new component type, simply define a Rust `struct` or `enum`, and
derive the [`Component`] trait.

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:component}}
```

Types must be unique – an entity can only have one component per Rust type.

## Newtype Components

Use wrapper (newtype) structs to make unique components out of simpler types:

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:component-newtype}}
```

## Marker Components

You can use empty structs to help you identify specific entities. These are
known as "marker components". Useful with [query filters][cb::query-filter].

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:component-marker}}
```

## Accessing Components

Components can be accessed from [systems][cb::system], using [queries][cb::query].

You can think of the query as the "specification" for the data you want
to access. It gives you access to specific component values from entities
that match the query's signature.

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:query}}
```

## Adding/removing Components

You can add/remove components on existing entities, using [`Commands`][cb::commands] or
[exclusive `World` access][cb::world].

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:insert-remove}}
```
