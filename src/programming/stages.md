# Stages

Bevy runs multiple systems in parallel, as much as possible, using multiple CPU
cores to give you good performance.

You can use stages to introduce synchronization points: ensure all systems of
the previous stage have completed before any systems of the next stage begin.

`Commands` are applied at the end of each stage.

Bevy provides at least these built-in stages: `First`, `PreUpdate`, `Update`, `PostUpdate`, `Last`.

By default, when you add systems, they are added to `Update`.

Bevy's internal systems are in the other stages, to ensure they are ordered
correctly relative to your game logic (for example, rendering needs to happen
after your game logic completes).

If you want to add your own systems to any of Bevy's internal stages, you need
to beware of potential unexpected interactions with Bevy's own internal systems.
Remember: Bevy's internals are implemented using ordinary systems and ECS, just
like your own stuff!

You can add your own additional stages. For example, if we want our debug systems to run after our game logic:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:custom-stage}}
```
