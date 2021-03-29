# System Chaining

Systems can take an input and produce an output, and be connected together to run as a single larger system (chain).

Think of this as "glue", for constructing a larger system out of multiple Rust functions.

One useful application of this is error handling (allowing the use of `?`):

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-io}}
```

Such systems cannot be registered individually (Bevy doesn't know what to do
with the input/output). You have to connect them in a chain:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-chain}}
```

Chains are a specialized tool; only use them if you really want to construct large
systems out of modular parts. Avoid using them as a general-purpose data passing
mechanism. For most use cases, you probably want to use [Events](./events.md) instead.

## Performance Warning

Beware that Bevy treats the whole chain as if it was a single big system, with
all the combined resources and queries. This implies that parallelism could be
limited, affecting performance.

Avoid adding a system that requires mutable access to anything, as part of
multiple chains. It would block all affected chains from running in parallel.


