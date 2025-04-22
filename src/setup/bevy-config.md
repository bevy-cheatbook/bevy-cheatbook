{{#include ../include/header016.md}}

# Configuring Bevy

Bevy is very modular and configurable. It is implemented as many separate
cargo crates, allowing you to remove the parts you don't need. Higher-level
functionality is built on top of lower-level foundational crates, and can
be disabled or replaced with alternatives.

The lower-level core crates (like the Bevy ECS) can also be used completely
standalone, or integrated into otherwise non-Bevy projects.

## Bevy Cargo Features

You can enable/disable various parts of Bevy using cargo features.

Many common features are enabled by default. Unfortunately, Cargo does not
let you just disable individual default features. If you want to disable
some of them, you need to disable all of them and re-enable the ones you need.

This is highly recommended if you are developing a library crate. It is also
a good way to reduce binary sizes, if you care about that.

Here is how you might configure your Bevy:

```toml
[dependencies.bevy]
version = "0.12"
# Disable the default features if there are any that you do not want
default-features = false
features = [
  # These are the default features:
  # (re-enable whichever you like)

  # Parts of Bevy:
  "animation",                   # Enable animation for everything that supports it
  "bevy_asset",                  # Asset management
  "bevy_audio",                  # Audio support
  "bevy_color",                  # Color management
  "bevy_core_pipeline",          # Bevy's GPU rendering architecture
  "bevy_gilrs",                  # Gamepad/controller support
  "bevy_gizmos",                 # Gizmos (drawing debug lines and shapes)
  "bevy_image",                  # Image support
  "bevy_input_focus",            # Input focusing system for UI
  "bevy_log",                    # Logging to console
  "bevy_mesh_picking_backend",   # 3D mesh picking (selection by cursor)
  "bevy_pbr",                    # 3D rendering and Physically Based Shading
  "bevy_picking",                # Picking (selection of objects by cursor)
  "bevy_render",                 # GPU support (based on `wgpu`)
  "bevy_scene",                  # ECS Scenes
  "bevy_sprite",                 # 2D rendering (sprites, meshes, text)
  "bevy_sprite_picking_backend", # 2D sprite picking (selection by cursor)
  "bevy_state",                  # App state management
  "bevy_text",                   # Text rendering
  "bevy_ui",                     # UI toolkit
  "bevy_ui_picking_backend",     # UI node picking (selection by cursor)
  "bevy_window",                 # Window management
  "bevy_winit",                  # Cross-platform window management support

  # Low-level tunables
  "std",            # Use the Rust standard library (important!)
  "async_executor", # Enable the Async Executor (Bevy task pools)
  "multi_threaded", # Enable CPU multithreading
  "sysinfo_plugin", # Support CPU and RAM usage diagnostics
  "custom_cursor",  # Support custom cursors

  # Platform-Specific
  "x11",                   # Linux: Support X11 windowing system
  "android_shared_stdcxx", # Android: use shared C++ library
  "android-game-activity", # Android: use GameActivity instead of NativeActivity
  "webgl2",                # Web: use WebGL2 instead of WebGPU

  # Built-in Data
  "default_font",       # Built-in default font for UI (Fira Mono)
  "smaa_luts",          # Support SMAA antialiasing
  "tonemapping_luts",   # Support different camera Tonemapping modes (enables KTX2+zstd)

  # Asset File Format Support
  "bevy_gltf", # GLTF 3D asset support
  "png",       # PNG image format for simple 2D images
  "hdr",       # HDR image format
  "ktx2",      # KTX2 format for GPU texture data
  "zstd",      # ZSTD compression support in KTX2 files
  "vorbis",    # Audio: OGG Vorbis


  # These are other (non-default) features that may be of interest:
  # (add any of these that you need)

  # General Bevy features
  "ghost_nodes", # UI ghost nodes

  # Low level tunables
  "async-io",             # Use `async-io` instead of `futures-lite`
  "serialize",            # Support for `serde` Serialize/Deserialize
  "subpixel_glyph_atlas", # Subpixel antialiasing for text/fonts
  "reflect_documentation", # Documentation reflection support
  "reflect_functions",    # Function reflection support

  # Platform-Specific
  "wayland",                  # Linux: Support Wayland windowing system
  "accesskit_unix",           # UNIX-like: AccessKit Accessibility Framework support
  "android-native-activity",  # Android: Use NativeActivity instead of GameActivity
  "spirv_shader_passthrough", # Vulkan: allow direct loading of SPIR-V shader blobs without validation
  "webgpu",                   # Web: use the faster, modern, experimental WebGPU API instead of WebGL2
  "statically-linked-dxc",    # Windows: embed the DirectX Shader Compiler into your game binary
  "web",                      # Web platform integration
  "libm",                     # Use libm for math on non-std / embedded platforms
  "critical-section",         # For supporting no-std / embedded
  "default_no_std",

  # Graphics/rendering features (may cause issues on old/weak GPUs)
  "experimental_pbr_pcss", # PCSS shadow filtering
  "meshlet",               # Meshlet / virtual geometry rendering (like Unreal's Nanite)
  "pbr_anisotropy_texture", # Support Anisotropy Map texture
  "pbr_multi_layer_material_textures", # Support multi-layer textures
  "pbr_specular_textures",  # Support specular map textures
  "pbr_transmission_textures", # Support textures for light transimssion (translucency)

  # Development features
  "dynamic_linking",     # Dynamic linking for faster compile-times
  "asset_processor",     # Enable asset processing support
  "bevy_debug_stepping", # Enable stepping through ECS systems for debugging
  "bevy_dev_tools",      # Extra dev functionality (like FPS overlay)
  "bevy_remote",         # Enable BRP (Bevy Remote Protocol) for integration with editors and external dev tools
  "file_watcher",        # Asset hot-reloading
  "meshlet_processor",   # Asset processor to convert meshes into meshlet format
  "glam_assert",         # Math validation / debug assertions
  "debug_glam_assert",   # Math validation / debug assertions
  "bevy_ui_debug",       # UI debugging functionality
  "bevy_ci_testing",     # For testing of Bevy itself in CI
  "embedded_watcher",    # Hot-reloading for Bevy's internal/builtin assets
  "configurable_error_handler",
  "trace",               # Enable tracing
  "trace_chrome",        # Enable tracing using the Chrome backend
  "trace_tracy",         # Enable tracing using Tracy
  "trace_tracy_memory",  # Enable memory tracking for Tracy
  "track_location",      # Enable location tracking
  "detailed_trace",      # Extra verbose tracing

  # Asset File Format Support
  "basis-universal", # Basis Universal GPU texture compression format
  "bmp",  # Uncompressed BMP image format
  "dds",  # DDS (DirectX) format for GPU textures, alternative to KTX2
  "exr",  # OpenEXR advanced image format
  "ff",   # FF image support
  "flac", # Audio: FLAC lossless format
  "gif",  # GIF legacy image format
  "ico",  # ICO image format (Windows icons)
  "jpeg", # JPEG lossy format for photos
  "mp3",  # Audio: MP3 format (not recommended)
  "minimp3", # Alternative MP3 decoder
  "pnm",  # PNM (pam, pbm, pgm, ppm) image format
  "qoi",  # QOI image format
  "shader_format_glsl",  # GLSL shader support
  "shader_format_spirv", # SPIR-V shader support
  "shader_format_wesl",  # WESL (Extended WGSL) shader support
  "tga",  # Truevision Targa image format
  "tiff", # TIFF image format
  "wav",  # Audio: Uncompressed WAV
  "webp", # WebP image format
  "zlib", # zlib compression support in KTX2 files
  "symphonia-aac", # AAC audio format (via Symphonia library)
  "symphonia-flac", # Alternative FLAC audio decoder (via Symphonia library)
  "symphonia-isomp4", # MP4 format (via Symphonia library)
  "symphonia-vorbis", # Alternative Ogg Vorbis audio support (via Symphonia library)
  "symphonia-wav", # Alternative WAV audio support (via Symphonia library)
  "symphonia-all", # All Audio formats supported by the Symphonia library
]
```

(See [here][bevy::features] for a full list of Bevy's cargo features.)

### Graphics / Rendering

For a graphical application or game (most Bevy projects), you can
include `bevy_winit` and your selection of Rendering features. For
[Linux][platform::linux] support, you need at least one of `x11` or `wayland`.

`bevy_render` and `bevy_core_pipeline` are required for any application using
Bevy rendering. They are automatically added if you enable higher-level stuff
like `bevy_ui`, `bevy_pbr`, `bevy_sprite`, etc.

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

The `webgl2` cargo feature selects WebGL2, `webgpu` selects WebGPU. Also
consider enabling `web` for general web support.

### Development Features

While you are developing your project, these features might be useful:

#### Asset hot-reloading and processing

The `file_watcher` feature enables support for [hot-reloading of
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

The `trace` feature is useful for profiling and diagnosing performance issues.

`trace_chrome` and `trace_tracy` choose the backend you want to use to
visualize the traces.

See [Bevy's official docs on profiling][bevy::profiling] to learn more.
