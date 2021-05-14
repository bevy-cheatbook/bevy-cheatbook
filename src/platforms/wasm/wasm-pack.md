# Project Setup (wasm-pack)

Make sure to first read the [overview page](../wasm.md). It contains important
general information.

---

`wasm-pack` is a simple and lightweight way to build your application for WASM.
Compared to the [Cargo Make](./cargo-make.md) approach, it is not as automated,
but you may prefer the simpler setup and configuration.

This page will show you how to configure a single Bevy project that can
be compiled for both Web and Desktop (Linux/Mac/Windows). If you only care
about Web, you could simplify the configuration shown here.

## Prerequisites

You need to have support for the WASM target for the Rust compiler. If you are
using `rustup` to manage your Rust installation, you can install it like this:

```shell
rustup target add wasm32-unknown-unknown
```

You also need to install `wasm-pack`. You can do that using `cargo`:

```shell
cargo install wasm-pack
```

## Cargo

The cross-platform setup requires the new `cargo` resolver, which was added
in Rust 1.51.

Add the following options to your `Cargo.toml`:

```toml
[package]
resolver = "2"

[lib]
crate-type = ["cdylib", "rlib"]

# Dependencies for all targets go here.
[dependencies]
wasm-bindgen = "0.2"

# Dependencies for native only.
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
bevy = {version = "0.5", default-features = false, features = ["bevy_wgpu", "bevy_winit", "render", "x11"]}

# Dependencies for WASM only.
[target.'cfg(target_arch = "wasm32")'.dependencies]
bevy = {version = "0.5", default-features = false, features = ["bevy_winit", "render"]}
bevy_webgl2 = "0.5"
```

## Main Source File

When using `wasm-pack`, it is treating the project as a "library" rather
than an "application". This means that your main file is `lib.rs` instead of
`main.rs`.

In `src/lib.rs`, write your "main function" as follows:

```rust,no_run,noplayground
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    
    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    
    // TODO: add all your other stuff to `app` as usual

    app.run();
}
```

To be able to also build the game as a normal desktop application, we also
need to add a trivial `main.rs`. It just calls the function we defined above:

```rust,no_run,noplayground
fn main() {
    myproject::run();
}
```

## Web Page

To display and run your game, you need to create a website. As the bare
minimum, you can create a simple HTML file (call it `index.html`) containing:

```html
<html>
<head>
<title>My Bevy Project</title>
</head>
<body>
<script type="module">
  import init from "./myproject.js";
  init("./myproject_bg.wasm").then(function (wasm) {
    wasm.run();
  });
</script>
</body>
</html>
```

(make sure to rename the `myproject.js` and `myproject_bg.wasm` to the
correct file names for your project; [see below](#building-and-running))

The above HTML is enough for you to test your game. You can enhance and
extend it as much as you want, to make a fancy website.

## Building and Running

You can build your project like this:

```shell
wasm-pack build --target web --release
```

This will produce output files in `pkg/`:
  - `myproject_bg.wasm`
  - `myproject.js`

To create the website that will display your game, you need to place the
above files, your `assets`, and the HTML file, together in the same folder.

You can then open the HTML file in a web browser, and enjoy your game!

