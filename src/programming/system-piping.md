{{#include ../include/header09.md}}

# System Piping

Relevant official examples:
[`system_piping`][example::system_piping].

---

You can compose a single Bevy [system][cb::system] from multiple Rust functions.

You can make functions that can take an input and produce an output, and be
connected together to run as a single larger system. This is called "system piping".

You can think of it as creating "modular" systems made up of multiple building
blocks. This way, you can reuse some common code/logic in multiple systems.

Note that system piping is *not* a way of communicating between systems.
If you want to pass data between systems, you should use [Events][cb::event]
instead.

## Example: Handling [`Result`][std::Result]s

One useful application of system piping is to be able to return errors (allowing
the use of Rust's `?` operator) and then have a separate function for handling
them:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-io}}
```

Such functions cannot be [registered][cb::app] individually as systems (Bevy
doesn't know what to do with the input/output). By "piping" them together, we
create a valid system that we can add:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-pipe}}
```

In a piped system, you can also include additional parameters like commands, resources, or queries. These parameters are passed automatically just as they would be in regular systems. Keep in mind that the piped input has to remain as the first argument.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-io-with-params}}
```

## Performance Warning

Beware that Bevy treats the whole chain as if it was a single big system, with
all the combined system parameters and their respective data access
requirements. This implies that parallelism could be limited, affecting
performance.

If you create multiple "piped systems" that all contain a common function which
contains any mutable access, that prevents all of them from running in parallel!
