# Change Detection

You can optimize your systems by only operating on entities when specific components change.

Use query filters:
 - `Added<T>`: detect adding new components to existing entities
 - `Mutated<T>`: detect mutation of existing components
 - `Changed<T>`: detect any change (added or mutated)

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:change-detection}}
```

## WARNING!

Change information only persists until the end of the current frame!

If your detecting system runs before the changing system, it will *not* detect
the changes!

`Added` is especially tricky to use. You need to add components using
[`Commands`](./commands.md), which are only applied at the end of each
[stage](./stages.md). The detecting system needs to be in a *later stage* that
runs during the same frame!
