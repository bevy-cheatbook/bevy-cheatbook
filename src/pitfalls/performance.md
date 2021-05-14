# Performance

This page is a summary of performance issues when working with Bevy.

(WIP)

## Unoptimized debug builds

Rust without compiler optimizations is *very slow*. With Bevy in
particular, the default debug build settings will lead to *awful* runtime
performance. Assets are slow to load and FPS is low.

Common symptoms:
  - Loading high-res 3D models with a lot of large textures, from GLTF
    files, can take over 20 seconds! This can trick you into thinking
    that your code is not working, because you will not see anything on
    the screen until it is ready.
  - After spawning some 2D sprites or 3D models, framerate may drop to
    unplayable levels.

However, fully-optimized release builds can be slow to compile.

Solutions:

```toml
# in `Cargo.toml` or `.cargo/config`

# Enable optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3

# Maybe also enable only a small amount of optimization for our code:
[profile.dev]
opt-level = 1
```
