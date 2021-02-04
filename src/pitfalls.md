# Common Pitfalls

## Slow performance in debug builds

Rust without compiler optimizations is *very slow*. With Bevy in particular, the default debug build settings will lead to *awful* runtime performance. Assets are slow to load and FPS is low.

Common symptoms:
 - Loading 3D models from GLB files can take over 20 seconds! This can trick you into thinking that your code is not working, because you will not see anything on the screen until it is ready.
 - After spawning some 2D sprites or 3D models, framerate may drop to unplayable levels.

However, fully-optimized release builds can be slow to compile.

Solutions:

```toml
# in `Cargo.toml` or `.cargo/config`

# Enable only a small amount of optimization:
[profile.dev]
opt-level = 1

# Enable optimizations for dependencies (incl. Bevy),
# but not for our own code:
[profile.dev.package."*"]
opt-level = 3
```

## Support for loading different file formats

By default, only a few asset file formats are enabled:
 - Images: PNG and HDR
 - Audio: MP3

You can enable more formats with cargo features:
 - Images: JPEG, TGA, BMP, DDS
 - Audio: FLAC, OGG, WAV

```toml
[dependencies.bevy]
version = "0.4"
features = ["jpeg", "tga", "bmp", "dds", "flac", "ogg", "wav"]
```

## Wayland support

For Linux builds, enable Wayland support (in addition to X11), with cargo feature:

```toml
[dependencies.bevy]
version = "0.4"
features = ["wayland"]
```

## Android support

MSAA does not work. If you get crashes or a black screen, ensure MSAA is *not* enabled.

Running in the background isn't handled properly. Switching out of the app may cause a crash.

Track the status of Android support [here](https://github.com/bevyengine/bevy/issues/86).
