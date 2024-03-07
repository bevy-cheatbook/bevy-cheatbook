{{#include ../include/header013.md}}

# Fixed Timestep

Relevant official examples:
[`fixed_timestep`][example::fixed_timestep].

---

If you need to run some [systems][cb::system] at a fixed rate, independent
of the display frame rate, Bevy provides a solution.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/fixed_timestep.rs:basic}}
```

Every frame update, Bevy will run the [`FixedUpdate`][bevy::FixedUpdate]
schedule as many times as needed to catch up. If the game is running slow,
it might run multiple times. If the game is running fast, it might be skipped.

This happens before the regular [`Update`][bevy::Update] schedule runs for that frame,
but after [state transitions][cb::state].

The default fixed timestep interval is 64 Hz. If you want something else,
you can configure it as follows:

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/fixed_timestep.rs:configure}}
```

## Checking the Time

Just use [`Res<Time>`][bevy::Time] as normal. When your system is running in
[`FixedUpdate`][bevy::FixedUpdate], Bevy will automatically detect that,
and all the timing information (such as delta) will represent the fixed
timestep instead of the display frame rate.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/fixed_timestep.rs:time}}
```

If you need to access the regular frame-time from a system running under
fixed timestep, you can use `Res<Time<Virtual>>` instead. `Res<Time<Real>>`
gives you the real (wall-clock) time, without pausing or scaling.

If you need to access the fixed-timestep-time from a system running outside
of fixed timestep, you can use `Res<Time<Fixed>>` instead.

## Should I put my systems in `Update` or `FixedUpdate`?

The purpose of fixed timestep is to make gameplay code behave predictably
and reliably. Things such as physics and simulation work best if they are
computed with fixed time intervals, as that avoids floating point errors
from accumulating and glitchy behavior from variable framerate.

The following things should probably be done in [`FixedUpdate`][bevy::FixedUpdate]:
 - Physics and collision detection
 - Networking / netcode
 - AI for enemies and NPCs (pathfinding, decisions, etc.)
 - Spawning/despawning gameplay-related entities
 - Other simulation and decision-making

However, anything that directly affects what is displayed on-screen should
run per-frame, in order to look smooth. If you do movement or animation under
fixed timestep, it will look choppy, especially on high-refresh-rate screens.

The following things should probably be done in [`Update`][bevy::Update]:
 - Camera movement and controls
 - Animations
 - UI
 - Visual effects
 - Anything that is part of your game's graphics/visuals or interactivity
 - [App state][cb::state] transitions

### Bridging the Gap

Sometimes there is a logical conflict:

For something like player movement, you want it to be computed reliably as part
of your gameplay/physics simulation, but you also want it to look smooth on-screen.

For input handling, you want it to be responsive and handled every frame, but
you also have game mechanics that need to respond to it.

The most elegant solution to both of these problems is to handle synchonization
yourself using custom types.

#### Movement

For player (and other) movement, you could create your own custom component
type to use instead of [`Transform`][bevy::Transform]. Implement your player
movement using your own types. Then have a system in [`Update`][bevy::Update]
to sync/update [`Transform`][bevy::Transform] from that, with some
interpolation to make it look smooth.

```rust,no_run,noplayground
// TODO show how to do this
```

#### Input Handling

If you use `Res<ButtonInput<...>>` and `.just_pressed`/`.just_released`
to check for key/button presses, beware that the state is updated once per
frame. This API is not reliable inside [`FixedUpdate`][bevy::FixedUpdate]. Use
[events][cb::event] for input handling instead, or roll your own abstractions.

One way to do this is to put your input handling systems
in [`PreUpdate`][bevy::PreUpdate], order them after the
[`bevy::input::InputSystem`][bevy::InputSystem] [set][cb::systemset], and do
your input handling there. Convert it into your own custom [event][cb::event]
types or some other useful representation, which you can then handle from
your gameplay code in [`FixedUpdate`][bevy::FixedUpdate].

```rust,no_run,noplayground
// TODO show how to do this
```

## Timing Caveats

Fixed timestep does not run in real-world time! You cannot rely on it for timing!

For example, if you try to play audio from it, or send network packets, you will
notice that they don't actually occur at the fixed timestep interval. They will
not be evenly spaced!

Your [systems][cb::system] are still called as part of the regular frame-update
cycle. Every frame update, Bevy will run the [`FixedMain`][bevy::FixedMain]
[schedule][cb::schedule] as many times as needed to catch up.

This means if you specify, for example, a 60 Hz fixed timestep interval, your
systems will not actually run in 1/60 second intervals in real time.

What will happen is the following:
 - If the display frame rate is faster than the timestep, some frame update cycles
   will skip the [`FixedMain`][bevy::FixedMain] schedule entirely.
 - If the display frame rate is slower than the timestep, some frame update cycles
   will run the [`FixedMain`][bevy::FixedMain] multiple times.

In any case, [`FixedMain`][bevy::FixedMain] will run right before [`Update`], where
your per-frame systems live.

## Additional Schedules

[`FixedUpdate`][bevy::FixedUpdate] is actually part of a larger [`FixedMain`][bevy::FixedMain]
[schedule][cb::schedule], which also contains other [schedules][cb::schedule]:

 - [`FixedFirst`][bevy::FixedFirst]
 - [`FixedPreUpdate`][bevy::FixedPreUpdate]
 - [`FixedUpdate`][bevy::FixedUpdate]
 - [`FixedPostUpdate`][bevy::FixedPostUpdate]
 - [`FixedLast`][bevy::FixedLast]

They are analogous to the [schedules][cb::schedule] in [`Main`][bevy::Main],
that run every frame update. They can be used for analogous purposes (to
contain "engine systems" from Bevy and plugins).
