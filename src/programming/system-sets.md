{{#include ../include/header010.md}}

# System Sets

System sets allow you to easily apply [ordering][cb::system-order] to a
collection of systems (even systems that have not been added yet). Ordering
systems relative to a system set applies that ordering to all systems in the
set.

First, define system sets:

```rust,no_run,noplayground
{{#include ../code010/src/programming/system_sets.rs:define}}
```

Then, configure the system sets with ordering:

```rust,no_run,noplayground
{{#include ../code010/src/programming/system_sets.rs:configure}}
```

And finally, assign systems to the system set:

```rust,no_run,noplayground
{{#include ../code010/src/programming/system_sets.rs:add-systems}}
```

System sets can also be "nested":

```rust,no_run,noplayground
{{#include ../code010/src/programming/system_sets.rs:nested}}
```
