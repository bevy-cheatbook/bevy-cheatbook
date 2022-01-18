# Bevy Time vs. Rust/OS time

{{#include ../include/links.md}}

Do *not* use [`std::time::Instant::now()`][std::Instant] to get the current
time. Use Bevy's [`Res<Time>`][bevy::Time].

Rust (and the OS) give you the precise time of the moment you call that
function. However, that's not what you want.

Your game systems are run by Bevy's parallel scheduler, which means that they
could be called at vastly different instants every frame! This will result in
inconsistent / jittery timings and make your game misbehave or look stuttery.

Bevy's [`Time`][bevy::Time] gives you timing information that is correct
and consistent. It's designed to be used for game logic.

This is not Bevy-specific, but applies to game development in general. Always
get your time from your game engine, not from your programming language or
operating system.
