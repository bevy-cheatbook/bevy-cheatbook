# Fixed Timestep

[official example]: https://github.com/bevyengine/bevy/blob/latest/examples/ecs/fixed_timestep.rs

(consult the [official example] for more details)

---

If you need something to happen at fixed time intervals (a common
use case is Physics updates), you can use Bevy's `FixedTimestep`
[Run Criteria](../programming/run-criteria.md).

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:fixed-timestep}}
```

(thanks **@billyb2** for [contributing this example](https://github.com/bevy-cheatbook/bevy-cheatbook/pull/64))

## State

You can check the current state of the fixed timestep trackers, by accessing
the `FixedTimesteps` [resource](../programming/res.md). This lets you know
how much time remains until the next time it triggers, or how much it has
overstepped. You need to label your fixed timesteps.

See the [official example], which illustrates this.

## Caveats

Note that, as this feature is implemented using Run Criteria, the systems
are still called as part of the regular frame-update cycle, along with all
of the normal systems. So, the timing is not exact.

The `FixedTimestep` run criteria simply checks how much time passed since the
last time your systems were ran, and decides whether to run them during the
current frame, or not, or run them multiple times, as needed.

### **Danger!** Lost events!

By default, Bevy's [events](../programming/events.md) are *not reliable!* They
only persist for 2 frames, after which they are lost. If your fixed-timestep
systems receive events, beware that you may miss some events if the framerate
is higher than 2x the fixed timestep.

One way around that is to use [events with manual
clearing](../patterns/manual-event-clear.md). This gives you control over
how long events persist, but can also leak / waste memory if you forget to
clear them.
