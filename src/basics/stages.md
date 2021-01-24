# Stages

Bevy runs multiple systems in parallel, as much as possible, using multiple CPU
cores to give you good performance.

You can use stages to introduce synchronization points: ensure all systems of
the previous stage have completed before any systems of the next stage begin.

`Commands` are applied at the end of each stage.

Bevy provides at least these stages: `FIRST`, `PRE_UPDATE`, `UPDATE`, `POST_UPDATE`, `LAST`.

By default, when you add systems, they are added to `UPDATE`.

Bevy's internal systems are in the other stages, to ensure they are ordered
correctly relative to your game logic (for example, rendering needs to happen
after your game logic completes).

We can add more stages. For example, if we want our debug systems to run after our game logic:

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:custom-stage}}
```
