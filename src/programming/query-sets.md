# Query Sets

{{#include ../include/links.md}}

For safety reasons, a [system][cb::system] cannot have multiple
[queries][cb::query] with mutability conflicts on the same
[components][cb::component].

Bevy provides a solution: wrap them in a [`QuerySet`][bevy::QuerySet]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-query-set}}
```

(Note: you have to use [`QueryState`][bevy::QueryState] instead of [`Query`][bevy::Query])

This ensures only one of the conflicting queries can be used at the same time.

The maximum number of queries in a query set is 4.
