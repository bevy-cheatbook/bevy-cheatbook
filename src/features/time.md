Warning: this page has not been updated for Bevy 0.10 yet!

# Time and Timers

{{#include ../include/links.md}}

Relevant official examples:
[`timers`][example::timers],
[`move_sprite`][example::move_sprite].

---

## Time

The [`Time`][bevy::Time] [resource][cb::res] is your main global source
of timing information, that you can access from any [system][cb::system]
that does anything that needs time. [You should derive all timings from
it][pitfall::time].

Bevy updates these values at the beginning of every frame.

### Delta Time

The most common use case is "delta time" – how much time passed between
the previous frame update and the current one. This tells you how fast the
game is running, so you can scale things like movement and animations. This
way everything can happen smoothly and run at the same speed, regardless of
the game's frame rate.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:time-delta}}
```

### Ongoing Time

[`Time`][bevy::Time] can also give you the total running time since startup.
Use this if you need a cumulative, increasing, measurement of time.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:time-monotonic}}
```

## Timers and Stopwatches

There are also facilities to help you track specific intervals or timings:
[`Timer`][bevy::Timer] and [`Stopwatch`][bevy::Stopwatch]. You can create
many instances of these, to track whatever you want. You can use them in
your own [component][cb::component] or [resource][cb::res] types.

Timers and Stopwatches need to be ticked. You need to have some system
calling `.tick(delta)`, for it to make progress, or it will be inactive.
The delta should come from the [`Time`][bevy::Time] resource.

### Timer

[`Timer`][bevy::Timer] allows you to detect when a certain interval of time
has elapsed. Timers have a set duration. They can be "repeating" or
"non-repeating".

Both kinds can be manually "reset" (start counting the time interval from the
beginning) and "paused" (they will not progress even if you keep ticking them).

Repeating timers will automatically reset themselves after they reach their
set duration.

Use `.finished()` to detect when a timer has reached its set duration. Use
`.just_finished()`, if you need to detect only on the exact tick when the
duration was reached.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:timer}}
```

Note that Bevy's timers do *not* work like typical real-life timers (which
count downwards toward zero). Bevy's timers start from zero and count *up*
towards their set duration. They are basically like stopwatches with extra
features: a maximum duration and optional auto-reset.

### Stopwatch

[`Stopwatch`][bevy::Stopwatch] allow you to track how much time has passed
since a certain point.

It will just keep accumulating time, which you can check with
`.elapsed()`/`.elapsed_secs()`. You can manually reset it at any time.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:stopwatch}}
```
