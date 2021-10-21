Relevant official examples:
[`ecs_guide`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/ecs_guide.rs).

---

# Entities

Entities are just a simple integer ID, that identifies a particular set of
component values.

# Components

Components are the data associated with entities.

Any Rust type (`struct` or `enum`) can be used as a component.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:struct-component}}
```

Types must be unique -- an entity can only have one component per Rust type.

Use wrapper (newtype) structs to make unique components out of simpler types:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:newtype-component}}
```

You can use empty structs to mark specific entities. These are known as "marker
components". Useful with [query filters](./queries.md#filter-by-component).

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:marker-component}}
```

Components can be accessed from [systems](./systems.md), using [queries](./queries.md). 

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

## Common Pitfalls

Because both bundles and individual components are regular Rust structs,
Bevy / the Rust compiler often has no way to distinguish them.

If you accidentally use a bundle struct somewhere where Bevy expects a
component, you will not get an error. Bevy will just treat it as a component
of that struct type!

For example, this is why we needed the `#[bundle]` annotation to include a
sub-bundle in the example above.

Other places where this issue may come up are: when using
[`Commands`](./commands.md), and when [querying](./queries.md) (queries *only*
work with components!).
