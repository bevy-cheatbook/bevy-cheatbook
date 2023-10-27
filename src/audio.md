{{#include ./include/header012.md}}

# Audio

Bevy offers a (somewhat barebones, but still useful) ECS-based Audio framework.
This chapter will teach you how to use it.

You can [play sound effects and music][cb::audio-basic] from your game, with
volume control. There is a rudimentary ["spatial audio"][cb::audio-spatial]
implementation, which can pan sounds left/right in stereo, based on the
[transforms][cb::transform] of [entities][cb::ec]. You can also implement your
own [custom sources of audio data][cb::audio-custom], if you want to synthesize
sound from code, stream data from somewhere, or any other custom use case.

There are also 3rd-party alternatives to Bevy's audio support:
 - [`bevy_kira_audio`][project::bevy_kira_audio]: uses [`kira`][project::kira]; provides a richer set of features and playback controls
 - [`bevy_oddio`][project::bevy_oddio]: uses [`oddio`][project::oddio]; seems to offer more advanced 3D spatial sound
 - [`bevy_fundsp`][project::bevy_fundsp]: uses [`fundsp`][project::fundsp]; for advanced sound synthesis and effects

(Bevy's official audio is based on the [`rodio`][project::rodio] library.)

As you can see, the Rust audio ecosystem is quite fragmented. There are
many backend libraries, all offering a different mix of features, none of
them particularly exhaustive. All of them are somewhat immature. You have
to pick your poison.

Audio is an area sorely in need of improvement. If you are an enthusiastic
audio developer, consider joining [Discord][bevy::discord] and helping
with development!
