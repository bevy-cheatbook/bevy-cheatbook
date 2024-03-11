{{#include ../include/header013.md}}

# Queries

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Queries let you access [components of entities][cb::ecs-intro].

Use the [`Query`] [system parameter][cb::system], where you can specify the data
you want to access, and optionally additional [filters][cb::query-filter].

Think of the types you put in your [`Query`] as a "specification" for selecting
what entities you want to access. Queries will match only those entities in the
ECS World that fit your specification. You are then able to access the relevant
data from any such entities.

The first type parameter for a query is the data you want to access. Use `&` for
shared/readonly access and `&mut` for exclusive/mutable access. Use [`Option`] if
the component is not required (you want to find entities with or without that
component. If you want multiple components, put them in a tuple.

### Iterating

The most common operation is to simply iterate to access the component values of
every entity that matches the query:

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:sys-simple-query}}
```

If you want to know the entity IDs of the entities you are accessing, you can
put the special [`Entity`] type in your query. This is useful if you need
to later perform specific operations on those entities.

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-entity}}
```

### Accessing Specific Entities

To access the [components][cb::component] from one specific [entity][cb::entity]
only, you need to know the [`Entity`] ID:

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-get}}
```

If you want to access the data from several entities all at once, you can use
`many`/`many_mut` (panic on error) or `get_many`/`get_many_mut` (return
[`Result`]).  These methods ensure that all the requested entities exist and
match the query, and will produce an error otherwise.

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-many}}
```

### Unique Entities

If you know that only one matching entity is supposed to exist (the query is
expected to only ever match a single entity), you can use `single`/`single_mut`
(panic on error) or `get_single`/`get_single_mut` (return [`Result`]). These
methods ensure that there exists exactly one candidate entity that can match
your query, and will produce an error otherwise.

You do not need to know the [`Entity`] ID.

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-single}}
```

### Combinations

If you want to iterate over all possible combinations of N entities, Bevy
provides a method for that too. Be careful: with a lot of entities, this
can easily become very slow!

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-combinations}}
```

## Bundles

Queries work with individual components. If you created an entity using a
[bundle][cb::bundle], you need to query for the specific components from
that bundle that you care about.

A common beginner mistake is to query for the bundle type!

## Query Filters

Add query filters to narrow down the entities you get from the query.

This is done using the second (optional) generic type parameter of the
[`Query`] type.

Note the syntax of the query: first you specify the data you want to access
(using a tuple to access multiple things), and then you add any additional
filters (can also be a tuple, to add multiple).

Use [`With`]/[`Without`] to only get entities that have specific components.

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:sys-query-filter}}
```

This is useful if you don't actually care about the data stored inside these
components, but you want to make sure that your query only looks for entities
that have (or not have) them. If you want the data, then put the component in
the first part of the query (as shown previously), instead of using a filter.

Multiple filters can be combined:
 - in a tuple to apply all of them (AND logic)
 - using the `Or<(…)>` wrapper to detect any of them (OR logic).
   - (note the tuple inside)

## Query Transmutation

If you want one function with a [`Query`] parameter to call another function
with a different (but compatible) [`Query`] parameter, you can create the
needed [`Query`] from the one you have using something called [`QueryLens`].

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-transmute}}
```

Note: when we call `debug_positions` from each function, it will access
different entities! Even though the `Query<&Transform>` parameter type does not
have any additional [filters][cb::query-filter], it was created by transmuting
via [`QueryLens`], and therefore it can only access the entities and components
of the original [`Query`] that it was derived from. If we were to add
`debug_positions` to Bevy as a regular system, it would access the transforms of
all entities.
