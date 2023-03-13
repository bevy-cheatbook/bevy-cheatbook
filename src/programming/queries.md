Warning: this page has not been updated for Bevy 0.10 yet!

# Queries

{{#include ../include/links.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Queries let you access [components of entities][cb::ecs-intro].

Use the [`Query`][bevy::Query] [system parameter][cb::system], where you can
specify the data you want to access, and optionally additional
[filters][cb::query-filter] for selecting entities.

Think of the types you put in your `Query` as a "specification" for selecting
what entities you want to access. Queries will match only those entities in the
ECS World that fit your specification. You are then able to access the relevant
data from individual such entities (using an [`Entity`][bevy::Entity] ID), or
iterate to access all entities that qualify.

The first type parameter for a query is the data you want to access. Use `&` for
shared/readonly access and `&mut` for exclusive/mutable access. Use `Option` if
the component is not required (you want to find entities with or without that
component. If you want multiple components, put them in a tuple.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-simple-query}}
```

The above example used iteration to access all entities that the query could find.

To access the [components][cb::component] from specific [entity][cb::entity]
only:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-get}}
```

If you want to know the entity IDs of the entities you are accessing, you can
put the special [`Entity`][bevy::Entity] type in your query. This is useful
together with iteration, so you can identify the entities that the query found:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-entity}}
```

If you know that the query is expected to only ever match a single entity, you
can use `single`/`single_mut` (panic on error) or `get_single`/`get_single_mut`
(return [`Result`][std::Result]). These methods ensure that there exists exactly
one candidate entity that can match your query, and will produce an error
otherwise.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-single}}
```

## Bundles

Queries work with individual components. If you created an entity using a
[bundle][cb::bundle], you need to query for the specific components from
that bundle that you care about.

A common beginner mistake is to query for the bundle type!

## Query Filters

Add query filters to narrow down the entities you get from the query.

This is done using the second (optional) generic type parameter of the
[`Query`][bevy::Query] type.

Note the syntax of the query: first you specify the data you want to access
(using a tuple to access multiple things), and then you add any additional
filters (can also be a tuple, to add multiple).

Use [`With`][bevy::With]/[`Without`][bevy::Without] to only get entities
that have specific components.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-query-filter}}
```

This is useful if you don't actually care about the data stored inside these
components, but you want to make sure that your query only looks for entities
that have (or not have) them. If you want the data, then put the component in
the first part of the query (as shown previously), instead of using a filter.

Multiple filters can be combined:
 - in a tuple to apply all of them (AND logic)
 - using the `Or<(…)>` wrapper to detect any of them (OR logic).
   - (note the tuple inside)
