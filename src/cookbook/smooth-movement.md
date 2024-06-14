{{#include ../include/header013.md}}

# Transform Interpolation / Extrapolation

Movement code for controlling the player (and other gameplay entities)
can pose a tricky problem.

You want it to be computed reliably as part of your gameplay/physics
simulation, which means doing it [on a fixed timestep][cb::fixedtimestep]. This
is to ensure consistent gameplay behavior regardless of the display
framerate. It is a must, to avoid glitchy behavior like clipping into walls.

However, you also want movement to look smooth on-screen. If you simply
mutate the [transforms][cb::transform] from within [`FixedUpdate`], that
will look choppy, especially on modern high-refresh-rate gaming displays.

The solution is to not manipulate [`Transform`] directly, but to create your
own custom [component][cb::component] types to use instead. Implement your
gameplay using your own types. Then, have a system in [`Update`], which uses
your custom components to compute the [`Transform`] that Bevy should use to
display the entity on every frame.

## Interpolation vs. Extrapolation

Interpolation means computing a [`Transform`] that is somewhere in-between the
current state of the entity, and the old state from the previous gameplay tick.

Extrapolation means computing a [`Transform`] that is somewhere in-between
the current state of the entity, and the predicted future state on the next
gameplay tick.

Interpolation creates movement that always looks both smooth and accurate,
but feels laggy / less responsive. The user will never see a truly up-to-date
representation of the gameplay state, as what you are rendering is always
delayed by one fixed timestep duration. Thus, interpolation is not suitable
for games where a responsive low-latency feel is important to gameplay.

Extrapolation creates movement that looks smooth and feels responsive, but
may be inaccurate. Since you are trying to predict the future, you might
guess wrong, and occasionally the rendered position of the entity on-screen
might jump slightly, to correct mispredictions.

## Example

First, create some custom [components][cb::component] to store your movement
state.

If you'd like to do interpolation, you need to remember the old position from
the previous gameplay tick. We created a separate component for that purpose.

If you'd like to do extrapolation, it might not be necessary, depending on
how you go about predicting the future position.

```rust,no_run,noplayground
{{#include ../code014/src/cookbook/smooth_movement.rs:types}}
```

Now, you can create your [systems][cb::system] to implement your movement
simulation. These systems should run in [`FixedUpdate`]. For this simple
example, we just apply our velocity value.

```rust,no_run,noplayground
{{#include ../code014/src/cookbook/smooth_movement.rs:controller}}
```

```rust,no_run,noplayground
{{#include ../code014/src/cookbook/smooth_movement.rs:controller-app}}
```

Now we need to create the [system][cb::system] to run every frame in
[`Update`], which computes the actual [transform][cb::transform] that Bevy
will use to display our entity on-screen.

[`Time<Fixed>`][`Time`] can give us the "overstep fraction", which is a
value between `0.0` and `1.0` indicating how much of a "partial timestep"
has accumulated since the last [fixed timestep][cb::fixedtimestep] run.
This value is our lerp coefficient.

### Interpolation

```rust,no_run,noplayground
{{#include ../code014/src/cookbook/smooth_movement.rs:interpolation}}
```

```rust,no_run,noplayground
{{#include ../code014/src/cookbook/smooth_movement.rs:interpolation-app}}
```

### Extrapolation

To do extrapolation, you need some sort of prediction about the future
position on the next gameplay tick.

In our example, we have our `velocity` value and we can reasonably assume
that on the next tick, it will simply be added to the position. So we can
use that as our prediction. As a general principle, if you have the necessary
information to make a good prediction about the future position, you should
use it.

```rust,no_run,noplayground
{{#include ../code014/src/cookbook/smooth_movement.rs:extrapolation-velocity}}
```

```rust,no_run,noplayground
{{#include ../code014/src/cookbook/smooth_movement.rs:extrapolation-velocity-app}}
```

If you'd like to make a general implementation of extrapolation, that does
not rely on knowing any information about how the movement works (such as
our `velocity` value in this example), you could try predicting the future
position based on the old position, assuming it will continue moving the
same way.

```rust,no_run,noplayground
{{#include ../code014/src/cookbook/smooth_movement.rs:extrapolation-from-old}}
```

```rust,no_run,noplayground
{{#include ../code014/src/cookbook/smooth_movement.rs:extrapolation-from-old-app}}
```

However, such an implementation will always guess wrong if the velocity is
changing, leading to poor results (jumpy movement that needs to correct its
course often).
