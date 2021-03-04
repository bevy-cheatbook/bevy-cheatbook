# Multi-Target (targeting both native and WASM)

It is possible to compile a project for both native systems (i.e. Windows,
MacOS and/or Linux) and WebAssembly without the use of custom feature flags.
The following snippet from `Cargo.toml` works in Rust nightly.

```toml
# This part only gets compiled for native.
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
bevy = {version = "0.4", default-features = false, features = ["bevy_dynamic_plugin", "bevy_wgpu", "bevy_winit", "png", "render", "x11", "wayland"]}

# This part only gets compiled for WASM.
[target.'cfg(target_arch = "wasm32")'.dependencies]
bevy = {version = "0.4", default-features = false, features = ["bevy_winit", "render"]}
bevy_webgl2 = "0.4"
```

This depends on [target-specific features](https://github.com/rust-lang/cargo/issues/1197), which is expected to land in the next Rust 1.51 stable release.

As utilizing `bevy_webgl2` requires loading its Bevy plugin which does not
compile natively, you will also need a way to only load it when compiling for
the web. This can be achieved as follows:

```
fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}
```
