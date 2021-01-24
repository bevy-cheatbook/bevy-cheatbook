# Queries

Queries let you access components of entities.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:sys-simple-query}}
```

Get the components associated with a specific entity:

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:query-get}}
```

Get the IDs of the entities you access with your queries:

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:query-entity}}
```

## Query Filters

Add query filters to narrow down the entities you get from the query.

Multiple filters can be combined:
 - in a tuple to apply all of them (AND logic)
 - using the Or<...> wrapper to detect any of them (OR logic).
 
### Filter by Component

Use `With`/`Without` to only get entities that have specific components.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:sys-query-filter}}
```

### Change Detection

You can optimize your systems by only operating on entities when specific components change.

Use query filters:
 - `Added<T>`: detect adding new components to existing entities
 - `Mutated<T>`: detect mutation of existing components
 - `Changed<T>`: detect any change (added or mutated)

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:change-detection}}
```

## Query Sets

For safety reasons, a system cannot have multiple queries with mutability conflicts on the same components.

The solution is to wrap them in a `QuerySet`:

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:sys-query-set}}
```

This ensures that only one of the conflicting queries can be used at the same time.
