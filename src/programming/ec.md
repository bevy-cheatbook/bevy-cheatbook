{{#include ../include/links.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

# Entities

Entities are just a [simple integer ID][bevy::Entity], that identifies a particular set of
component values.

To create ("spawn") new entities, use [`Commands`][cb::commands].

# Components

Components are the data associated with entities.

To create a new component type, simply define a Rust `struct` or `enum`, and
derive the [`Component`][bevy::Component] trait.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:struct-component}}
```

Types must be unique -- an entity can only have one component per Rust type.

Use wrapper (newtype) structs to make unique components out of simpler types:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:newtype-component}}
```

You can use empty structs to help you identify specific entities. These are
known as "marker components". Useful with [query filters][cb::query-filter].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:marker-component}}
```

Components can be accessed from [systems][cb::system], using [queries][cb::query].

You can add/remove components on existing entities, using [`Commands`][cb::commands].

# Component Bundles

Bundles are like "templates", to make it easy to create entities with a
common set of components.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:bundle}}
```

Bevy also considers arbitrary tuples of components as bundles:

```
(ComponentA, ComponentB, ComponentC)
```

Note that you cannot [query][cb::query] for a whole bundle. Bundles are just a
convenience when creating the entities. Query for the individual component types
that your [system][cb::system] needs to access.
