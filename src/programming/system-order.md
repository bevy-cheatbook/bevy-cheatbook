{{#include ../include/header013.md}}

# System Order of Execution

Bevy's scheduling algorithm is designed to deliver maximum performance
by running as many systems as possible in parallel across the available
CPU threads.

This is possible when the systems do not conflict over the data they need
to access. However, when a system needs to have mutable (exclusive) access
to a piece of data, other systems that need to access the same data cannot
be run at the same time. Bevy determines all of this information from the
system's function signature (the types of the parameters it takes).

In such situations, the order is *nondeterministic* by default. Bevy takes
no regard for when each system will run, and the order could even change
every frame!

## Explicit System Ordering

If a specific system must always run before or after some other systems,
you can add ordering constraints:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_order.rs:app}}
```

`.before`/`.after` may be used as many times as you need on one system.

If you need to apply the same ordering constraints to many systems,
consider using [system sets][cb::systemset].

## Does it even matter?

In many cases, you don't need to worry about this.

However, sometimes you need to rely on specific systems to run in a particular
order. For example:

  - Maybe the logic you wrote in one of your systems needs any modifications
    done to that data by another system to always happen first?
  - One system needs to receive [events][cb::event] sent by another system.
  - You are using [change detection][cb::change-detection].

In such situations, systems running in the wrong order typically causes
their behavior to be delayed until the next frame. In rare cases, depending
on your game logic, it may even result in more serious logic bugs!

It is up to you to decide if this is important.

With many things in typical games, such as juicy visual effects, it probably
doesn't matter if they get delayed by a frame. It might not be worthwhile
to bother with it. If you don't care, leaving the order ambiguous may also
result in better performance.

On the other hand, for things like handling the player input controls,
this would result in annoying lag or worse, so you should probably fix it.

## Circular Dependencies

If you have multiple systems mutually depending on each other, then it is
clearly impossible to resolve the situation completely like that.

You should try to redesign your game to avoid such situations, or just accept
the consequences. You can at least make it behave predictably, using explicit
ordering to specify the order you prefer.
