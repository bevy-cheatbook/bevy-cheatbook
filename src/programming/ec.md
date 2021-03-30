# ECS as a Data Structure

Bevy stores and manages all your data for you, using the Bevy ECS (Entity-Component System).

Conceptually, you can think of it by analogy with tables, like in a database or
spreadsheet. Your different data types (Components) are like the "columns" of a
table, and there can be arbitrarily many "rows" (Entities) containing values /
instances of each component.

For example, you could create a `Health` component for your game. You could then
have many entities representing different things in your game, such as the
player, NPCs, or monsters, all of which can have a `Health` value (as well as
other relevant components).

This makes it easy to write game logic ([Systems](./systems.md)) that can operate on any
entity with the necessary components (such as a health/damage system for
anything that has `Health`), regardless of whether that's the player, an NPC, or
a monster (or anything else). This makes your game logic very flexible and
reusable.

The set / combination of components that a given entity has, is called the
entity's Archetype.

Note that entities aren't limited to just "objects in the game world". The ECS
is a general-purpose data structure. You can create entities and components to
store any data.

## Entities

Entities are just a simple integer ID, that identifies a particular set of
component values.

## Components

Components are the data associated with entities.

Any Rust type (`struct` or `enum`) can be used as a component.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:struct-component}}
```

Types must be unique -- an entity can only have one component of each type.

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

## Component Bundles

Bundles are like "templates", to make it easy to create entities with a common set of components.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:bundle}}
```

Bevy also considers arbitrary tuples of components as bundles:

```
(ComponentA, ComponentB, ComponentC)
```

## Common Pitfalls

Because both bundles and individual components are regular Rust structs, Bevy /
the Rust compiler often has no way to distinguish them.

If you accidentally use a bundle struct somewhere where Bevy expects a
component, you will not get an error. Bevy will just treat it as a component of
that struct type!

For example, this is why we needed the `#[bundle]` annotation to include a
sub-bundle in the example above.

Other places where this issue may come up are: when using
[`Commands`](./commands.md), and when [querying](./queries.md) (queries *only*
work with components!).
