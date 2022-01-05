# Configuring Bevy

Bevy is very modular and configurable. It is implemented as many separate
cargo crates, allowing you to remove the parts you don't need. Higher-level
functionality is built on top of lower-level foundational crates, and can
be disabled or replaced with alternatives.

The lower-level core crates (like the Bevy ECS) can also be used completely
standalone, or integrated into otherwise non-Bevy projects.

## Bevy Cargo Features

In Bevy projects, you can enable/disable various parts of Bevy using cargo features.

Here I will explain some of them and why you might want to change them.

Many common features are enabled by default. If you want to disable some of
them, note that, unfortunately, Cargo does not let you disable individual
default features, so you need to disable all default bevy features and
re-enable the ones you need.

Here is how you might configure your Bevy:

```toml
[dependencies.bevy]
version = "0.6"
# Disable the default features if there are any that you do not want
default-features = false
features = [
  # These are the default features:
  # (keep whichever you like)
  "render",
  "bevy_winit",
  "bevy_gilrs",
  "bevy_audio",
  "png",
  "hdr",
  "vorbis",
  "x11",
  "filesystem_watcher",
  # These are other features that may be of interest:
  # (add any of these that you need)
  "bmp",
  "tga",
  "dds",
  "jpeg",
  "wav"
  "flac",
  "mp3",
  "subpixel_glyph_atlas",
  "dynamic",
  "serialize",
  "trace",
  "trace_tracy",
  "trace_chrome",
  "wgpu_trace",
  "wayland"
]
```

(See [here](https://docs.rs/crate/bevy/0.6.0/features) for a full list of
Bevy's cargo features.)

### Graphics / Rendering

For a graphical application or game (most Bevy projects),
you need `render`, `bevy_winit`.

If you don't need graphics (like for a dedicated game server, scientific
simulation, etc.), you may remove these features.

There are also [community-made alternative backend
crates](https://bevyengine.org/assets/#backends) that could replace Bevy's
default graphics, if you want that for any reason.

### Audio

Bevy's audio is very limited in functionality and
somewhat broken. It is recommended that you use the
[`bevy_kira_audio`](https://github.com/NiklasEi/bevy_kira_audio) plugin
instead. Disable `bevy_audio` and `vorbis`.

See [this page](../features/audio.md) for more information.

### File Formats

You can use the relevant cargo features to enable/disable support for loading
assets (images/textures and audio) with various different file formats.

See [this page](../pitfalls/file-formats.md) for more information.

### Input Devices

If you do not care about controller/joystick support, you can disable `bevy_gilrs`.

### Linux Windowing Backend

On [Linux](../platforms/linux.md), you can choose to support X11, Wayland,
or both. Only `x11` is enabled by default, as it is the legacy system
that should be compatible with most/all distributions, to make your builds
smaller and compile faster. You might want to additionally enable `wayland`,
to fully and natively support modern Linux environments. This will add a few
extra transitive dependencies to your project.

### Development Features

While you are developing your project, these features might be useful:

#### Dynamic Linking

`dynamic` causes Bevy to be built and linked as a shared/dynamic library
(`*.so` on Linux, `*.dylib` on macOS, `*.DLL` on Windows). This will make
incremental builds *much* faster, which can reduce frustration when you are
trying to test out changes to your code. On my machine, my projects recompile
in ~2 sec without this option, and in ~0.5 sec with it enabled. This makes
starting the game feel almost instant.

This is known to work very well on Linux, but you may encounter issues on
other platforms. YMMV. I've heard people have issues on Windows.

Do not enable this for release builds you intend to publish to other people;
it introduces unneeded complexity (you need to bundle extra files) and
potential for things to not work correctly. Use this only during development.

For this reason, it may be convenient to specify the feature as a commandline
option to cargo, instead of putting it in your `Cargo.toml`. Simply run your
project like this:

```sh
cargo run --features bevy/dynamic
```

#### Tracing

The features `trace` and `wgpu_trace` may be useful for profiling and
diagnosing performance issues.
