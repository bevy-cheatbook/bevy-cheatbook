# System Chaining

{{#include ../include/links.md}}

Relevant official examples:
[`system_chaining`][example::system_chaining].

---

You can compose a single Bevy [system][cb::system] from multiple Rust functions.

You can make functions that can take an input and produce an output, and be
connected together to run as a single larger system.

This is called "system chaining", but beware that the term is somewhat
misleading – you are *not* creating a chain of multiple systems to run in
order; you are creating a single large Bevy system consisting of multiple
Rust functions.

Note that system chaining is *not* a way of communicating between systems.
If you want to pass data between systems, you should use [Events][cb::event]
instead.

---

One useful application is to be able to return errors from your system code
(allowing the use of Rust's `?` operator) and then have a separate function
for handling them:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-io}}
```

Such functions cannot be [registered][cb::app] individually as systems
(Bevy doesn't know what to do with the input/output). You have to connect
them in a chain:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-chain}}
```

## Performance Warning

Beware that Bevy treats the whole chain as if it was a single big system,
with all the combined resources and queries. This implies that parallelism
could be limited, affecting performance.

Avoid adding a system that requires mutable access to anything, as part
of multiple chains. It would block all affected chains (and other systems
accessing the same data) from running in parallel.
