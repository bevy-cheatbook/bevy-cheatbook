# Stages

All systems to be run by Bevy are contained in stages. Every frame update,
Bevy executes each stage, in order. Within each stage, Bevy's scheduling
algorithm can run many systems in parallel, using multiple CPU cores for
good performance.

The boundaries between stages are effectively hard synchronization points.
They ensure that all systems of the previous stage have completed before any
systems of the next stage begin, and that there is a moment in time when no
systems are in-progress.

This makes it possible/safe to apply [`Commands`](./commands.md). Any
operations performed by systems using `Commands` are applied at the end of
each stage.

Internally, Bevy has at least these built-in stages: `First`, `PreUpdate`,
`Update`, `PostUpdate`, `Last`.

([labeled](./labels.md) with `enum CoreStage`)

By default, when you add your systems, they are added to `CoreStage::Update`.

Bevy's internal systems are in the other stages, to ensure they are ordered
correctly relative to your game logic.

If you want to add your own systems to any of Bevy's internal stages, you
need to beware of potential unexpected interactions with Bevy's own internal
systems.  Remember: Bevy's internals are implemented using ordinary systems
and ECS, just like your own stuff!

You can add your own additional stages. For example, if we want our debug
systems to run after our game logic:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:custom-stage}}
```

If you need to manage when your systems run, relative to one another, it
is generally preferable to avoid using stages, and to use [explicit system
ordering](./system-order.md) instead. Stages limit parallel execution and
the performance of your game.

However, stages can make it easier to organize things, when you really want
to be sure that all previous systems have completed. Stages are also the
only way to apply `Commands`.

If you have systems that need to rely on the actions that other systems
have performed by using `Commands`, and need to do so during the same frame,
placing those systems into separate stages is the only way to accomplish that.

[More information on avoiding frame delays / 1-frame-lag, when using `Commands`.](../pitfalls/frame-delay.md#commands)
