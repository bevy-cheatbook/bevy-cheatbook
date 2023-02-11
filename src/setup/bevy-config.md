# Configuring Bevy

{{#include ../include/links.md}}

Bevy is very modular and configurable. It is implemented as many separate
cargo crates, allowing you to remove the parts you don't need. Higher-level
functionality is built on top of lower-level foundational crates, and can
be disabled or replaced with alternatives.

The lower-level core crates (like the Bevy ECS) can also be used completely
standalone, or integrated into otherwise non-Bevy projects.

## Bevy Cargo Features

In Bevy projects, you can enable/disable various parts of Bevy using cargo features.

Many common features are enabled by default. If you want to disable some of
them, note that, unfortunately, Cargo does not let you disable individual
default features, so you need to disable all default bevy features and
re-enable the ones you need.

Here is how you might configure your Bevy:

```toml
[dependencies.bevy]
version = "0.9"
# Disable the default features if there are any that you do not want
default-features = false
features = [
  # These are the default features:
  # (re-enable whichever you like)

  # Bevy functionality:
  "bevy_asset",         # Assets management
  "bevy_scene",         # Scenes management
  "bevy_gilrs",         # Gamepad input support
  "bevy_audio",         # Builtin audio
  "bevy_winit",         # Window management
  "animation",          # Animation support
  "x11",                # Linux: Support X11 windowing system
  "filesystem_watcher", # Asset hot-reloading
  "render",             # Graphics Rendering

  ## "render" actually just includes:
  ## (feel free to use just a subset of these, instead of "render")
  "bevy_render",        # Rendering framework core
  "bevy_core_pipeline", # Common rendering abstractions
  "bevy_sprite",        # 2D (sprites) rendering
  "bevy_pbr",           # 3D (physically-based) rendering
  "bevy_gltf",          # GLTF 3D assets format support
  "bevy_text",          # Text/font rendering
  "bevy_ui",            # UI toolkit

  # File formats:
  "png",
  "hdr",
  "vorbis",

  # These are other features that may be of interest:
  # (add any of these that you need)

  # Bevy functionality:
  "wayland",              # Linux: Support Wayland windowing system
  "subpixel_glyph_atlas", # Subpixel antialiasing for text/fonts
  "serialize",            # Support for `serde` Serialize/Deserialize
  "bevy_dynamic_plugin",   # Support for loading of `DynamicPlugin`s

  # File formats:
  "ktx2", # preferred format for GPU textures
  "dds",
  "jpeg",
  "bmp",
  "tga",
  "basis-universal",
  "zstd", # needed if using zstd in KTX2 files
  "flac",
  "mp3",
  "wav",

  # Development/Debug features:
  "dynamic",      # Dynamic linking for faster compile-times
  "trace",        # Enable tracing for performance measurement
  "trace_tracy",  # Tracing using `tracy`
  "trace_chrome", # Tracing using the Chrome format
  "wgpu_trace",   # WGPU/rendering tracing
]
```

(See [here][bevy::features] for a full list of Bevy's cargo features.)

### Graphics / Rendering

For a graphical application or game (most Bevy projects), you can include
`render` and `bevy_winit`. For [Linux][platform::linux] support, you need
at least one of `x11` or `wayland`.

However, `render` is a meta-feature; it simply enables all the graphics-related
features of Bevy. If you want, you can strip it down and include only what
you need.

`bevy_render` and `bevy_core_pipeline` are required for any application using
Bevy rendering.

If you only need 2D and no 3D, add `bevy_sprite`.

If you only need 3D and no 2D, add `bevy_pbr`. If you are [loading 3D models
from GLTF files][cb::gltf], add `bevy_gltf`.

If you are using Bevy UI, you need `bevy_text` and `bevy_ui`.

If you don't need any graphics (like for a dedicated game server, scientific
simulation, etc.), you may remove all of these features.

### Audio

Bevy's audio is very limited in functionality. It is recommended that you
use the [`bevy_kira_audio`][project::bevy_kira_audio] plugin instead. Disable
`bevy_audio` and `vorbis`.

See [this page][cb::audio] for more information.

### File Formats

You can use the relevant cargo features to enable/disable support for loading
assets with various different file formats.

See [here][builtins::file-formats] for more information.

### Input Devices

If you do not care about [gamepad (controller/joystick)][input::gamepad]
support, you can disable `bevy_gilrs`.

### Linux Windowing Backend

On [Linux][platform::linux], you can choose to support X11, Wayland,
or both. Only `x11` is enabled by default, as it is the legacy system
that should be compatible with most/all distributions, to make your builds
smaller and compile faster. You might want to additionally enable `wayland`,
to fully and natively support modern Linux environments. This will add a few
extra transitive dependencies to your project.

### Asset hot-reloading

The `filesystem_watcher` feature controls support for [hot-reloading of
assets][cb::asset-hotreload], supported on desktop platforms.

### Development Features

While you are developing your project, these features might be useful:

#### Dynamic Linking

`dynamic` causes Bevy to be built and linked as a shared/dynamic library.
This will make incremental builds *much* faster.

This is only supported on desktop platforms. Known to work very well on Linux.
Windows and macOS might have issues.

Do not enable this for release builds you intend to publish to other people.
It introduces unneeded complexity (you need to bundle extra files) and
potential for things to not work correctly. Use this only during development.

For this reason, it may be convenient to specify the feature as a commandline
option to `cargo`, instead of putting it in your `Cargo.toml`. Simply run your
project like this:

```sh
cargo run --features bevy/dynamic
```

#### Tracing

The features `trace` and `wgpu_trace` may be useful for profiling and
diagnosing performance issues.

`trace_chrome` and `trace_tracy` choose the backend you want to use to
visualize the traces.
