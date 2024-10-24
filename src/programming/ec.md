{{#include ../include/header015.md}}

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
{{#include ../code015/src/programming/ec.rs:spawn-despawn}}
```

Many of your entities might need to have the same common components. You can use
[Bundles][cb::bundle] to make it easier to spawn your entities.

# Components

Components are the data associated with entities.

To create a new component type, simply define a Rust `struct` or `enum`, and
derive the [`Component`] trait.

```rust,no_run,noplayground
{{#include ../code015/src/programming/ec.rs:component}}
```

Types must be unique – an entity can only have one component per Rust type.

## Newtype Components

Use wrapper (newtype) structs to make unique components out of simpler types:

```rust,no_run,noplayground
{{#include ../code015/src/programming/ec.rs:component-newtype}}
```

## Marker Components

You can use empty structs to help you identify specific entities. These are
known as "marker components". Useful with [query filters][cb::query-filter].

```rust,no_run,noplayground
{{#include ../code015/src/programming/ec.rs:component-marker}}
```

## Accessing Components

Components can be accessed from [systems][cb::system], using [queries][cb::query].

You can think of the query as the "specification" for the data you want
to access. It gives you access to specific component values from entities
that match the query's signature.

```rust,no_run,noplayground
{{#include ../code015/src/programming/ec.rs:query}}
```

## Adding/removing Components

You can add/remove components on existing entities, using [`Commands`][cb::commands] or
[exclusive `World` access][cb::world].

```rust,no_run,noplayground
{{#include ../code015/src/programming/ec.rs:insert-remove}}
```

## Required Components

Components can require other components. When you spawn an entity, Bevy will
ensure that any additional components required by the components you provide are
also added to the entity. If you do not provide a value for those components,
Bevy will automatically insert a default value.

For example, the way to create a [3D camera][cb::camera-3d] in Bevy is to spawn
an entity with the [`Camera3d`] component. Bevy will automatically make sure to
also add any other necessary components, such as [`Camera`] (where Bevy stores
general camera properties), [`Transform`] ([transforms][cb::transform]), and others.

These dependencies are recursive. For example, [`Transform`] requires
[`GlobalTransform`], so our camera entity will also get that automatically.

To learn what components are required by a
specific component type, look for the [`impl Component for
…`](https://docs.rs/bevy/0.15.0-rc.1/bevy/core_pipeline/core_3d/struct.Camera3d.html#impl-Component-for-Camera3d)
under "Trait Implementations" in the API Docs.

If you want to customize the values in any of those components, you can just add
them yourself.

```rust,no_run,noplayground
{{#include ../code015/src/programming/ec.rs:required-camera}}
```

You can add required components to your own component types as follows:

```rust,no_run,noplayground
{{#include ../code015/src/programming/ec.rs:require-components}}
```

You need to make sure that all the required components impl [`Default`]. Or
alternatively, you can use a custom constructor function for initialization.

```rust,no_run,noplayground
{{#include ../code015/src/programming/ec.rs:require-components-default}}
```

And now, with all the above, we can do:

```rust,no_run,noplayground
{{#include ../code015/src/programming/ec.rs:require-components-spawn}}
```
