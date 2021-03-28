# Entities

Conceptually, an "object" in the game.

Technically, entities are just simple unique integer IDs, that you can use to access component data.

# Components

Components are data that you can attach to entities.

Any Rust type (`struct` or `enum`) can be used as a component.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:struct-component}}
```

Types must be unique -- an entity can only have one of each type.

Use wrapper (newtype) structs to make unique components out of simpler types:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:newtype-component}}
```

Use empty structs as marker components (tags). Useful with [query filters](./queries.md#filter-by-component).

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:marker-component}}
```

Components can be accessed from systems, using [queries](./queries.md). 

# Component Bundles

Bundles are like "templates", to make it easy to spawn entities with a common set of components.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:bundle}}
```
