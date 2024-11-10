{{#include ../include/header014.md}}

# Jittering Time, Choppy Movement/Animation

## Fixed Timestep

Gameplay movement/simulation code is typically run on a [fixed
timestep][cb::fixedtimestep] (in the [`FixedUpdate`] [schedule][cb::schedule]).
This is important to make sure these computations happen consistently and
correctly, regardless of display framerate.

However, obviously, that means they do not follow the display's frame rate.
This causes movement to look choppy on-screen.

The solution to this problem is [transform
interpolation/extrapolation][cookbook::smooth-movement].

## Bevy Time vs. Rust/OS time

Do *not* use [`std::time::Instant::now()`][`Instant`] to get the
current time. [Get your timing information from Bevy][cb::time], using
[`Res<Time>`].

Rust (and the OS) give you the precise time of the moment you call that
function. However, that's not what you want.

Your game systems are run by Bevy's parallel scheduler, which means that they
could be called at vastly different instants every frame! This will result in
inconsistent / jittery timings and make your game misbehave or look stuttery.

Bevy's [`Time`] gives you timing information that is consistent throughout the
frame update cycle. It is intended to be used for game logic.

This is not Bevy-specific, but applies to game development in general. Always
get your time from your game engine, not from your programming language or
operating system.

## Imprecise Frame Delta Time

That said, it is actually often impossible for any game engine (not just Bevy)
to give precise values for the frame delta time.

The time when the final rendered frame is actually displayed on-screen is
called "presentation time". On most OSs, there is no API to measure that. The
game engine does not know when the user can actually see the rendered frame
produced by the GPU.

Therefore, the frame time must be measured differently. Typically, what is
measured is the time between runs of the game engine's main frame update
loop on the CPU. Bevy measures its timings at the point when GPU work is
submitted to the GPU for processing.

This is a good approximation, but it will never perfectly match reality.

If you run with VSync on a 60Hz display, you would expect every frame delta
to be exactly `16.667ms`. But if you log the delta time values from Bevy,
you will see that they vary. They are close, but never exactly that value.

There is no known complete solution to this. Bevy developers are investigating
ways to improve the quality of Bevy's time measurements.
