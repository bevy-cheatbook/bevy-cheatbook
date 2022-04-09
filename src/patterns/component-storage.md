# Component Storage (Table/Sparse-Set)

{{#include ../include/links.md}}

Bevy ECS provides two different ways of storing data: tables and sparse sets.
The two storage kinds offer different performance characteristics.

The kind of storage to be used can be chosen per [component][cb::component] type.
When you derive the [`Component`][bevy::Component] trait, you can specify it.
The default, if unspecified, is table storage.

You can have components with a mixture of different storage kinds on the same entity.

The rest of this page is dedicated to explaining the performance trade-offs and why
you might want to choose one storage kind vs. the other.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:component-storage}}
```

## Table Storage

Table storage is optimized for fast [query][cb::query] iteration. If the way you usually use a
specific component type is to access its data across many entities, this will offer the best
performance.

However, adding/removing table components to existing entities is a relatively slow operation. It
requires copying the data of all the other table components for the entity, to a different
location in memory.

It's OK if you have to do this sometimes, but if you are likely to add/remove a component very
frequently, you might want to switch that component type to sparse-set storage.

You can see why table storage was chosen as Bevy's default. Most component types are rarely
added/removed in practice. You typically spawn entities with all the components they should have,
and then access the data via queries, usually every frame. Sometimes you might add or remove
a component to change an entity's behavior, but probably not nearly as often, or every frame.

## Sparse-Set Storage

Sparse-Set storage is optimized for fast adding/removing of a component to existing entities, at
the cost of slower querying. It can be more efficient for components that you would like to add/remove very frequently.

An example of this might be a [marker component][cb::component-marker] indicating whether an enemy can
currently see the player. You might want to have such a component type, so that you can easily
use a [query filter][cb::query-filter] to find all the enemies that are currently tracking the
player. However, this is something that can change every frame, as enemies or the player move
around the game level. If you add/remove this component every time the visibility status changed,
that's a lot of additions and removals.

You can see that situations like these are more niche and do not apply to most component types. Treat
sparse-set storage as a potential optimization you could try in specific circumstances.

## Table Fragmentation

Furthermore, the actual memory layout of the "tables" depends on the set of all table components
that each of your entities has.

ECS queries perform best when many of the entities they match have the same overall set of components.

Having a large number of entities, that all have the same component types, is very efficient
in terms of data access performance. Having diverse entities with a varied mixture of different
component types, means that their data will be fragmented in memory and be less efficient to access.

Sparse-Set components do not affect the memory layout of tables. Hence, components that are only
used on a few entities or as a "temporary effect", might also be good candidates for sparse-set
storage. That way they don't fragment the memory of the other (table) components.

## Overall Advice

While this page describes the general performance characteristics and gives some guidelines, you
often cannot know if something improves performance without benchmarking.

You could use sparse-set storage just in obvious situations like the "visibility marker" example
given earlier, and otherwise leave the default table storage.

When your game grows complex enough and you have something to benchmark, you could try to apply
it in more places and see how it affects your results.
