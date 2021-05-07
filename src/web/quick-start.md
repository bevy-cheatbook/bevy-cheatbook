# Bevy on the Web

Below is a set of instructions to run your Bevy project on the web. This is
achieved by compiling to a WebAssembly (often abbreviated as WASM) target,
which can be run in the browser.

This guide demonstrates a multi-target project setup. This means that following
the instructions below should result in a project that can compile for both
native (i.e. Windows, MacOS and/or Linux) and WebAssembly, without the use of
any feature flags or build scripts. You will need at least `Rust 1.51` or
later.

Note that by default, errors in a browser context do not forward panic
messages, making it difficult to debug. See [Better
Errors](./web/better-errors.md).

## Setting up dependencies

Bevy's rendering back-end currently does not support the web. You will need
[bevy_webgl2](https://github.com/mrk-its/bevy_webgl2) instead. Additionally,
you will need to opt into a new Cargo resolver in order for platform-specific
features to work. The following setup in `Cargo.toml` works starting from `Rust
1.51`:

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
bevy = {version = "0.4", default-features = false, features = ["bevy_wgpu", "bevy_winit", "render", "x11"]}

# Dependencies for WASM only.
[target.'cfg(target_arch = "wasm32")'.dependencies]
bevy = {version = "0.4", default-features = false, features = ["bevy_winit", "render"]}
bevy_webgl2 = "0.4"
```

Note that certain Bevy features (such as audio) are currently not compatible
with WebAssembly and will produce compilation errors. Before adding features
or 3rd party crates, please verify whether they are WASM-compatible.

## Converting your application to a library

Next, you will have to restructure your project as a library. This is more
straightforward than it sounds. Simply rename your `main.rs` to `lib.rs` and
set up your main function as follows:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    // Your code.
}
```

You still require a `main.rs` if you desire to also run your code natively, but all that needs to be in it is a call to your library run function:

```rust
fn main() {
    myproject::run();
}

```

## Configuring the Bevy application builder

You are now compiling a WASM-compatible rendering back-end, but you still need
to load it in Bevy's application builder. Normally this would break native
compilation. Instead, we load the plugin conditionally:

```rust
fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}
```

## Building for WebAssembly

In order to build a Bevy project (or any Rust project) for the web, simply
targeting the `wasm32-unknown-unknown` platform and building is not
sufficient. At the very least you also require a set of JavaScript mappings so
the browser knows what functions in your binary to call. Depending on the
complexity of your project this can get complex fast. Luckily, tooling
exists to automate this.

There are two common high-level tools that can produce workable WASM binaries,
JavaScript mappings and potentially other handy files:
[wasm-pack](https://github.com/rustwasm/wasm-pack) and
[cargo-make](https://github.com/sagiegurari/cargo-make).

`wasm-pack` is the recommended approach, as it is easier. If you would prefer
to use `cargo make`, see [Cargo Make](./web/cargo-make.md).

### Building with `wasm-pack`

First, you will need to install `wasm-pack`. You can do this with the help of
cargo:

```shell
cargo install wasm-pack
```

You can then simply build your project for the web by issuing
`wasm-pack build --target web --release`. The resulting files will be in the
`pkg/` directory.

## Deploying to the web

In order for other people to use your application on the web, you must host it
first. We'll explore using either a custom webpage, or `[GitHub Pages](https://pages.github.com/)` to achieve this.

### Custom webpage

Simply serve the following files (foundin the `/pkg` directory after building).
If your project has any assets, don't forget to include them.

```
myproject_bg.wasm
myproject.js
```

Of course, simply serving these files is not enough. You need to actually have
a webpage in which you call the generated JavaScript file. Simply add a HTML
script tag to your webpage (be sure to rename files for your project):

```html
<script type="module">
  import init from "./myproject.js";
  init("./myproject_bg.wasm").then(function (wasm) {
    wasm.run();
  });
</script>
```

With `wasm-pack` you could also publish your project as a `Node.js` package or
set it up with `webpack`, but that is considered out of scope for this guide.
Refer to the [wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/) for more information.

### GitHub Pages

The most straightforward way to deploy a WASM application to GitHub Pages is
from within the same repository of the source code. You will need the same
files generated as in the [subsection](#Custom webpage) above.

1. Create an empty branch

```shell
git checkout --orphan web
git reset --hard
```

2. Put all files necessary for hosting there (see above section). Remember to add your assets if you have any.
3. Go to repository settings, scroll down to “GitHub Pages” section, then under “Source” pick the branch “web” and the `/` (root) folder. Then click “Save”.

For more details, visit the official [GitHub Pages documentation](https://guides.github.com/features/pages/);
