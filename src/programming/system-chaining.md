# System Chaining

Relevant official examples:
[`system_chaining`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/system_chaining.rs).

---

Systems can take an input and produce an output, and be connected together
to run as a single larger system (chain).

Think of this as "glue", for constructing a larger system out of multiple
Rust functions.

One useful application is to be able to return errors from systems (allowing
the use of Rust's `?` operator) and handle them elsewhere:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-io}}
```

Such systems cannot be registered individually (Bevy doesn't know what to
do with the input/output). You have to connect them in a chain:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-chain}}
```

Chaining effectively constructs a single large system out of modular parts. It
is *not* a channel for passing data around. If you want to pass data between
systems, you probably want to use [Events](./events.md) instead.

## Performance Warning

Beware that Bevy treats the whole chain as if it was a single big system,
with all the combined resources and queries. This implies that parallelism
could be limited, affecting performance.

Avoid adding a system that requires mutable access to anything, as part
of multiple chains. It would block all affected chains (and other systems
accessing the same data) from running in parallel.
