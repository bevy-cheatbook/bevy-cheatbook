{{#include ../include/header014.md}}

# Queries

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Queries let you access [components of entities][cb::ecs-intro]. Use the
[`Query`] [system parameter][cb::system], where you can specify the data
you want to access, and optionally additional [filters][cb::query-filter].

Think of the types you put in your [`Query`] as a "specification" for
selecting what entities you want to be able to access. Queries will match
only those entities in the ECS World that fit your specification. You are
then able to use the query in various different ways, to access the relevant
data from such entities.

## Query Data

The first type parameter for a query is the data you want to access. Use `&` for
shared/readonly access and `&mut` for exclusive/mutable access. Use [`Option`] if
the component is not required (you want to access entities with or without that
component). If you want multiple components, put them in a tuple.

### Iterating

The most common operation is to simply iterate the query, to access the
component values of every entity that matches:

```rust,no_run,noplayground
{{#include ../code014/src/programming/queries.rs:sys-simple-query}}
```

Instead of iterating in a `for` loop, you can also call `.for_each(…)` with a
closure to run for each entity. This syntax often tends to be optimized better
by the compiler and may lead to better performance. However, it may be less
flexible (you cannot use control flow like `break`/`continue`/`return`/`?`)
and more cumbersome to write. Your choice.

```rust,no_run,noplayground
{{#include ../code014/src/programming/queries.rs:iter-for-each}}
```

If you want to know the entity IDs of the entities you are accessing, you can
put the special [`Entity`] type in your query. This is useful if you need
to later perform specific operations on those entities.

```rust,no_run,noplayground
{{#include ../code014/src/programming/queries.rs:query-entity}}
```

### Accessing Specific Entities

To access the [components][cb::component] from one specific [entity][cb::entity]
only, you need to know the [`Entity`] ID:

```rust,no_run,noplayground
{{#include ../code014/src/programming/queries.rs:query-get}}
```

If you want to access the data from several entities all at once, you can use
`many`/`many_mut` (panic on error) or `get_many`/`get_many_mut` (return
[`Result`]). These methods ensure that all the requested entities exist and
match the query, and will produce an error otherwise.

```rust,no_run,noplayground
{{#include ../code014/src/programming/queries.rs:query-many}}
```

### Unique Entities

If you know that only one matching entity is supposed to exist (the query is
expected to only ever match a single entity), you can use `single`/`single_mut`
(panic on error) or `get_single`/`get_single_mut` (return [`Result`]). These
methods ensure that there exists exactly one candidate entity that can match
your query, and will produce an error otherwise.

You do not need to know the [`Entity`] ID.

```rust,no_run,noplayground
{{#include ../code014/src/programming/queries.rs:query-single}}
```

### Combinations

If you want to iterate over all possible combinations of N entities, Bevy
provides a method for that too. Be careful: with a lot of entities, this
can easily become very slow!

```rust,no_run,noplayground
{{#include ../code014/src/programming/queries.rs:query-combinations}}
```

### Bundles

You **cannot** query for a [bundle][cb::bundle]!

Bundles are only a convenience to help you set up your entities with the
correct set of components you'd like to have on them. They are only used
during spawning / insert / remove.

Queries work with individual components. You need to query for the specific
components from that bundle that you care about.

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
{{#include ../code014/src/programming/queries.rs:sys-query-filter}}
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
{{#include ../code014/src/programming/queries.rs:query-transmute}}
```

Note: when we call `debug_positions` from each function, it will access
different entities! Even though the `Query<&Transform>` parameter type does not
have any additional [filters][cb::query-filter], it was created by transmuting
via [`QueryLens`], and therefore it can only access the entities and components
of the original [`Query`] that it was derived from. If we were to add
`debug_positions` to Bevy as a regular system, it would access the transforms of
all entities.

Also note: this has some performance overhead; the transmute operation is not
free. Bevy normally caches some query metadata across multiple runs of a
system. When you create the new query, it has to make a copy of it.

## Query Joining

You can combine two queries to get a new query to access only those entities
that would match both queries, yielding the combined set of components.

This works via [`QueryLens`], just like [transmutation](#query-transmutation).

```rust,no_run,noplayground
{{#include ../code014/src/programming/queries.rs:query-join}}
```

Note: the resulting query cannot access any data that the original queries
were not able to access. If you try to add [`With`]/[`Without`] filters,
they will not have their usual effect.
