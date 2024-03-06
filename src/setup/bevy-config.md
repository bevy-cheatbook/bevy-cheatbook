{{#include ../include/header012.md}}

# Configuring Bevy

Bevy is very modular and configurable. It is implemented as many separate
cargo crates, allowing you to remove the parts you don't need. Higher-level
functionality is built on top of lower-level foundational crates, and can
be disabled or replaced with alternatives.

The lower-level core crates (like the Bevy ECS) can also be used completely
standalone, or integrated into otherwise non-Bevy projects.

## Bevy Cargo Features

In Bevy projects, you can enable/disable various parts of Bevy using cargo features.

Many common features are enabled by default. If you want to disable some of
them, you need to disable all of them and re-enable the ones you need.
Unfortunately, Cargo does not let you just disable individual default features.

Here is how you might configure your Bevy:

```toml
[dependencies.bevy]
version = "0.12"
# Disable the default features if there are any that you do not want
default-features = false
features = [
  # These are the default features:
  # (re-enable whichever you like)

  # Bevy functionality:
  "multi-threaded",     # Run with multithreading
  "bevy_asset",         # Assets management
  "bevy_audio",         # Builtin audio
  "bevy_gilrs",         # Gamepad input support
  "bevy_scene",         # Scenes management
  "bevy_winit",         # Window management (cross-platform Winit backend)
  "bevy_render",        # Rendering framework core
  "bevy_core_pipeline", # Common rendering abstractions
  "bevy_gizmos",        # Support drawing debug lines and shapes
  "bevy_sprite",        # 2D (sprites) rendering
  "bevy_pbr",           # 3D (physically-based) rendering
  "bevy_gltf",          # GLTF 3D assets format support
  "bevy_text",          # Text/font rendering
  "bevy_ui",            # UI toolkit
  "animation",          # Animation support
  "tonemapping_luts",   # Support different camera Tonemapping modes (enables KTX2+zstd)
  "default_font",       # Embed a minimal default font for text/UI

  # File formats:
  "png",    # PNG image format for simple 2D images
  "hdr",    # HDR images
  "ktx2",   # Preferred format for GPU textures
  "zstd",   # ZSTD compression support in KTX2 files
  "vorbis", # Audio: OGG Vorbis

  # Platform-specific:
  "x11",                   # Linux: Support X11 windowing system
  "android_shared_stdcxx", # Android: use shared C++ library
  "webgl2",                # Web: use WebGL2 instead of WebGPU

  # These are other (non-default) features that may be of interest:
  # (add any of these that you need)

  # Bevy functionality:
  "asset_processor",      # Asset processing
  "filesystem_watcher",   # Asset hot-reloading
  "subpixel_glyph_atlas", # Subpixel antialiasing for text/fonts
  "serialize",            # Support for `serde` Serialize/Deserialize
  "async-io",             # Make bevy use `async-io` instead of `futures-lite`
  "pbr_transmission_textures", # Enable Transmission textures in PBR materials
                               # (may cause issues on old/lowend GPUs)

  # File formats:
  "dds",  # Alternative DirectX format for GPU textures, instead of KTX2
  "jpeg", # JPEG lossy format for 2D photos
  "webp", # WebP image format
  "bmp",  # Uncompressed BMP image format
  "tga",  # Truevision Targa image format
  "exr",  # OpenEXR advanced image format
  "pnm",  # PNM (pam, pbm, pgm, ppm) image format
  "basis-universal", # Basis Universal GPU texture compression format
  "zlib", # zlib compression support in KTX2 files
  "flac", # Audio: FLAC lossless format
  "mp3",  # Audio: MP3 format (not recommended)
  "wav",  # Audio: Uncompressed WAV
  "symphonia-all", # All Audio formats supported by the Symphonia library
  "shader_format_glsl", # GLSL shader support
  "shader_format_spirv", # SPIR-V shader support

  # Platform-specific:
  "wayland",              # (Linux) Support Wayland windowing system
  "accesskit_unix",       # (Unix-like) AccessKit integration for UI Accessibility
  "bevy_dynamic_plugin",  # (Desktop) support for loading of `DynamicPlugin`s

  # Development/Debug features:
  "dynamic_linking",   # Dynamic linking for faster compile-times
  "trace",             # Enable tracing for performance measurement
  "detailed_trace",    # Make traces more verbose
  "trace_tracy",       # Tracing using `tracy`
  "trace_tracy_memory", # + memory profiling
  "trace_chrome",      # Tracing using the Chrome format
  "wgpu_trace",        # WGPU/rendering tracing
  "debug_glam_assert", # Assertions to validate math (glam) usage
  "embedded_watcher",  # Hot-reloading for Bevy's internal/builtin assets
]
```

(See [here][bevy::features] for a full list of Bevy's cargo features.)

### Graphics / Rendering

For a graphical application or game (most Bevy projects), you can include
`bevy_winit` and your selection of Rendering features. For
[Linux][platform::linux] support, you need at least one of `x11` or `wayland`.

`bevy_render` and `bevy_core_pipeline` are required for any application using
Bevy rendering.

If you only need 2D and no 3D, add `bevy_sprite`.

If you only need 3D and no 2D, add `bevy_pbr`. If you are [loading 3D models
from GLTF files][cb::gltf], add `bevy_gltf`.

If you are using Bevy UI, you need `bevy_text` and `bevy_ui`. `default_font`
embeds a simple font file, which can be useful for prototyping, so you don't
need to have a font asset in your project. In a real project, you probably
want to use your own fonts, so your text can look good with your game's art
style. In that case, you can disable the `default_font` feature.

If you want to draw debug lines and shapes on-screen, add `bevy_gizmos`.

If you don't need any graphics (like for a dedicated game server, scientific
simulation, etc.), you may remove all of these features.

### File Formats

You can use the relevant cargo features to enable/disable support for loading
assets with various different file formats.

See [here][builtins::file-formats] for more information.

### Input Devices

If you do not care about [gamepad (controller/joystick)][input::gamepad]
support, you can disable `bevy_gilrs`.

### Platform-specific

#### Linux Windowing Backend

On [Linux][platform::linux], you can choose to support X11, Wayland,
or both. Only `x11` is enabled by default, as it is the legacy system
that should be compatible with most/all distributions, to make your builds
smaller and compile faster. You might want to additionally enable `wayland`,
to fully and natively support modern Linux environments. This will add a few
extra transitive dependencies to your project.

Some Linux distros or platforms might struggle with X11 and work better with
Wayland. You should enable both for best compatibility.

#### WebGPU vs WebGL2

On [Web/WASM][platform::web], you have a choice between these two rendering backends.

WebGPU is the modern experimental solution, offering good performance and
full feature support, but browser support for it is limited (only known to
work in very recent versions of Chrome and Firefox nightly).

WebGL2 gives the best compatibility with all browsers, but has worse performance
and some limitations on what kinds of graphics features you can use in Bevy.

The `webgl2` cargo feature selects WebGL2 if enabled. If disabled, WebGPU is used.

### Development Features

While you are developing your project, these features might be useful:

#### Asset hot-reloading and processing

The `filesystem_watcher` feature enables support for [hot-reloading of
assets][cb::asset-hotreload], supported on desktop platforms.

The `asset_processor` feature enables support for [asset
processing][cb::asset-processor], allowing you to automatically convert and
optimize assets during development.

#### Dynamic Linking

`dynamic_linking` causes Bevy to be built and linked as a shared/dynamic
library. This will make recompilation *much* faster during development.

This is only supported on desktop platforms. Known to work very well on Linux.
Windows and macOS are also supported, but are less tested and have had issues in
the past.

It is not recommended to enable this for release builds you intend to publish
to other people, unless you have a very good special reason to and you know
what you are doing. It introduces unneeded complexity (you need to bundle
extra files) and potential for things to not work correctly. You should only
use it during development.

For this reason, it may be convenient to specify the feature as a commandline
option to `cargo`, instead of putting it in your `Cargo.toml`. Simply run your
project like this:

```sh
cargo run --features bevy/dynamic_linking
```

You could also add this to your [IDE/editor configuration][cb::ide].

#### Tracing

The features `trace` and `wgpu_trace` may be useful for profiling and
diagnosing performance issues.

`trace_chrome` and `trace_tracy` choose the backend you want to use to
visualize the traces.

See [Bevy's official docs on profiling][bevy::profiling] to learn more.
