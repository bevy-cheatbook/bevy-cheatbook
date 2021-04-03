# System Order of Execution

Bevy's scheduling algorithm is designed to deliver maximum performance by
running as many systems as possible in parallel across the available CPU
threads.

This is possible when the systems do not conflict over the data they need
to access. However, when a system needs to have mutable (exclusive) access to a
piece of data, other systems that need to access the same data cannot be run at
the same time. Bevy determines all of this information from the system's
function signature (the types of the parameters it takes).

In such situations, the order is *nondeterministic* by default. Bevy takes no
regard for when each system will run, and the order could even change every frame!

## Does it even matter?

In many cases, you don't need to worry about this.

However, sometimes you need to rely on specific systems to run in a particular
order. For example:

  - Maybe the logic you wrote in one of your systems needs any modifications
    done to that data by another system to always happen first?
  - One system needs to receive [events](./events.md) sent by another system.
  - You are using [change detection](./change-detection.md).

In such situations, systems running in the wrong order typically causes their
behavior [to be delayed until the next frame](../pitfalls/frame-delay.md). In
rare cases, depending on your game logic, it may even result in more serious
logic bugs!

It is up to you to decide if this is important.

With many things in typical games, such as juicy visual effects, it probably
doesn't matter if they get delayed by a frame. It might not be worthwhile to
bother with it. If you don't care, leaving the order ambiguous may also result
in better performance.

On the other hand, for things like handling the player input controls, this
would result in annoying lag, so you should probably fix it.

## Explicit System Ordering

The solution is to use system [labels](./labels.md) to explicitly specify the
order you want:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:system-labels}}
```

Each label is a reference point that other systems can be ordered around.

`.label`/`.before`/`.after` may be used as many times as you need on one system.

You can place multiple labels on one system.

You can also use the same label on multiple systems.

When you have multiple systems with common labels or ordering, it may be
convenient to use [system sets](./system-sets.md).

## Circular Dependencies

If you have multiple systems mutually depending on each other, then it is
clearly impossible to resolve the situation completely like that.

You should try to redesign your game to avoid such situations, or just accept
the consequences. You can at least make it behave predictably, using explicit
ordering to specify the order you prefer.

In really advanced scenarios, if you insist on all affected systems becoming
settled during the same frame, you might be able to do it using [Looping Run
Criteria](./run-criteria-loop.md) or [states](./states.md).
