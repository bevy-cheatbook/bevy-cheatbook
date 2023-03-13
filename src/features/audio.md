Warning: this page has not been updated for Bevy 0.10 yet!

# Audio

{{#include ../include/links.md}}

Bevy's own built-in audio support is extremely barebones and limited. It
can play sounds and give some control over its volume and playback.

**TODO:** show how to use Bevy audio, now that it its usability has improved.

## Kira Audio

Instead, you could try the [`bevy_kira_audio`][project::bevy_kira_audio]
community plugin, which integrates the Kira sound library with bevy. Kira
is much more feature-rich, including support for managing many audio tracks
(like background music and sound effects), with volume control, stereo panning,
playback rate, and streaming.

Using `bevy_kira_audio` in your project requires some extra configuration,
because you need to disable Bevy's own audio. Bevy's audio is a cargo feature
that is enabled by default, but must be disabled. Cargo does not let you
disable individual default features, so you need to disable all default bevy
features and re-enable the ones you need. [See here][cb::features] for more info.

You must not include the `bevy_audio` feature, or any of the audio file
formats (such as the default `vorbis`). Enable the file formats you care
about on `bevy_kira_audio` instead of Bevy.

```toml
[dependencies.bevy]
version = "0.9"
default-features = false
# These are the remaining default features other than `bevy_audio` and `vorbis`
features = [
  "render",
  "animation",
  "bevy_asset",
  "bevy_scene",
  "bevy_winit",
  "bevy_gilrs",
  "png",
  "hdr",
  "filesystem_watcher",
  "x11"
]

[dependencies.bevy_kira_audio]
version = "0.11"
# `ogg` format support is enabled by default, disable if you don't want it
default-features = false
# enable the features you care about
features = [
  "wav",
  "flac",
  "mp3",
  "ogg",
]
```
