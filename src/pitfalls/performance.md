# Performance

This page is a summary of performance issues when working with Bevy.

(WIP)

## Unoptimized debug builds

Rust without compiler optimizations is *very slow*. With Bevy in
particular, the default cargo build debug settings will lead to *awful* runtime
performance. Assets are slow to load and FPS is low.

Common symptoms:
  - Loading high-res 3D models with a lot of large textures, from GLTF
    files, can take minutes! This can trick you into thinking
    that your code is not working, because you will not see anything on
    the screen until it is ready.
  - After spawning even a few 2D sprites or 3D models, framerate may drop
    to unplayable levels.

However, fully-optimized release builds can be slow to compile.

Solutions:

```toml
# in `Cargo.toml` or `.cargo/config.toml`

# Enable optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3

# Maybe also enable only a small amount of optimization for our code:
[profile.dev]
opt-level = 1
```

The above config changes the default settings for debug builds, to compile
dependencies with full optimizatons (slowing down initial clean builds, but
not incremental builds), and enable minimal optimization for your own code
(minimal slowdown of incremental builds).

Now you no longer need to build with `--release` just to test your game with
adequate performance. You can use it for *actual* release builds that you
send to your users.

If you want, you can also enable LTO (Link-Time-Optimization) for the actual
release builds, to squeeze out even more performance at the cost of very
slow compile times:

```toml
[profile.release]
lto = "thin"
```
