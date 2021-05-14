# Query Sets

For safety reasons, a system cannot have multiple queries with mutability
conflicts on the same components.

Bevy provides a solution: wrap them in a `QuerySet`:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sys-query-set}}
```

This ensures only one of the conflicting queries can be used at the same time.

The maximum number of queries in a query set is 4.
