# Audio

Bevy's own built-in audio support is extremely barebones and limited. It
can play sounds, but that's about it. It doesn't even have volume control.

Instead, we recommend that you try the
[`bevy_kira_audio`](https://github.com/NiklasEi/bevy_kira_audio) community
plugin, which integrates the Kira sound library with bevy. Kira is much
more feature-rich, including support for managing many audio tracks (like
background music and sound effects), with volume control, stereo panning,
playback rate, and streaming. It also has [web](../platforms/wasm.md) support.

The community largely considers Bevy's audio to be obsolete and useless;
it will probably be removed and replaced with something else (maybe
`bevy_kira_audio`).

Using `bevy_kira_audio` in your project requires some extra configuration,
because you need to disable Bevy's own audio. Bevy's audio is a cargo feature
that is enabled by default, but must be disabled. Cargo does not let you
disable individual default features, so you need to disable all default bevy
features and re-enable the ones you need.

You must not include the `bevy_audio` feature, or any of the audio file
formats (such as the default `mp3`). Enable the file formats you care about on
`bevy_kira_audio` instead of Bevy.

```toml
[dependencies.bevy]
version = "0.5"
default-features = false
# These are the remaining default features other than `bevy_audio` and `mp3`
features = [
  "bevy_dynamic_plugin",
  "render",
  "bevy_wgpu",
  "bevy_winit",
  "bevy_gilrs",
  "bevy_gltf",
  "png",
  "hdr",
  "x11"
]

[dependencies.bevy_kira_audio]
version = "0.5.0"
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

See [this page](../setup/bevy-modular.md#bevy-cargo-features) for more
information about Bevy's cargo features.
