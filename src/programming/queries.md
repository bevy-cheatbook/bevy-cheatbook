# Queries

{{#include ../include/links.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Queries let you access [components of entities][cb::ecs-intro].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-simple-query}}
```

Get the [components][cb::component] associated with a specific
[entity][cb::entity]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-get}}
```

Get the IDs ([`Entity`][bevy::Entity]) of the entities you access with
your queries:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-entity}}
```

If you know that the query should only ever match a single entity, you can
use `single`/`single_mut`, instead of iterating:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-single}}
```

(this will panic if the query matches more than one entity)

## Bundles

Queries work with individual components. If you created an entity using a
[bundle][cb::bundle], you need to query for the specific components from
that bundle that you care about.

A common beginner mistake is to query for the bundle type!

## Query Filters

Add query filters to narrow down the entities you get from the query.

Use [`With`][bevy::With]/[`Without`][bevy::Without] to only get entities
that have specific components.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-query-filter}}
```

Multiple filters can be combined:
 - in a tuple to apply all of them (AND logic)
 - using the `Or<(…)>` wrapper to detect any of them (OR logic).
   - (note the tuple inside)
