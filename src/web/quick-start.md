# Bevy on the Web

Below are a set of instructions to run your Bevy project on the web, or in other words, in the browser. Others might say you want to compile for WebAssembly (often abbreviated to WASM). These are all the same thing.

Note that theses steps provide instructions to target WASM *only*. if you
want your project to work on native (i.e. Windows, MacOS and/or Linux) *and*
WASM, you will need to perform additional steps. See
[Multi-Target](./web/multi-target.md).

## Getting a WASM-compatible rendering back-end

Bevy's rendering back-end currently does not support the web. You will need
[bevy_webgl2](https://github.com/mrk-its/bevy_webgl2) instead. At the very
least, you require the following in your `Cargo.toml`:

```toml
[dependencies]
bevy = {version = "0.4", default-features = false, features = ["bevy_winit", "render"]}
bevy_webgl2 = "0.4"
```

Note that some Bevy features (such as audio) are currently not compatible with
WebAssembly and will produce compilation errors. Before adding features or 3rd
party crates, verify whether they are WASM-compatible.

## Configuring the Bevy application builder

You are now compiling a WASM-compatible rendering back-end, but you still need
to load it in Bevy's application builder. In your code simply replace Bevy's
`DefaultPlugins` with `bevy_webgl2::DefaultPlugins`:

```rust
App::build()
    .add_plugins(bevy_webgl2::DefaultPlugins)
```

or add both Bevy's `DefaultPlugins` and `bevy_webgl2::WebGL2Plugin`:

```rust
App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(bevy_webgl2::WebGL2Plugin)
```

## Building for WebAssembly

In order to practically build a Bevy project (or any Rust project) for the web,
you need more than just a compiled binary. Simply targeting the
`wasm32-unknown-unknown` platform and building is not sufficient. At the very
least you also require a set of javascript mappings so the browser knows what
functions in your binary to call. Depending on the complexity of your project
this can get convoluted fast. Luckily, tooling exists to automate this.

There are two common high-level methods that result in workable WASM binaries,
javascript mappings and potentially other handy files:
[wasm-pack](https://github.com/rustwasm/wasm-pack) and
[cargo-make](https://github.com/sagiegurari/cargo-make).

`wasm-pack` lets you not only target the web natively but allows interop
with [Node.js](https://nodejs.org/en/) or otherwise use along any javascript
packages in a workflow such as [webpack](https://webpack.js.org/). The
downside is that you must adhere to a certain project structure, and there is
no built-in way to quickly serve a webpage for development and testing.

`cargo-make` is a task runner that lets you define and configure a set of tasks
in a script to execute upon building. It can not only be used to build for the
web and generate the necessary javascript file(s), but even to quickly
serve a webpage for development and testing. The downside here is that we must
define all this behavior ourselves by creating a build script. There are
templates you can use, but these are opinionated and you will have to follow
their project structure then, as well.

### Using `wasm-pack`

First, you will need to install `wasm-pack`. You can do this with the help of
cargo:

```shell
cargo install wasm-pack
```

Next, you will have to restructure your project as a library. This is more
straightforward than it sounds. Simply rename your `main.rs` to `lib.rs` and
rename your main function as follows:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    // Your code.
}
```

You still require a `main.rs` if you desire to also run your code natively
(i.e. on Windows, MacOS and/or Linux), but all that needs to be in it is a call
to your main library function:

```rust
fn main() {
    mylib::run();
}

```

Finally, you will need the following additions in `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
```

You can then simply build your project for the web by issuing
`wasm-pack build --target web --release`. The resulting files will be in the
`pkg/` directory.

### Using `cargo-make`

To actually get `cargo-make` building for the web, we need a build file.
Writing this file is out of scope for this guide, but luckily the author of
`bevy_webgl2` has provided [bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template) which you can use. If you are starting your
project from scratch you can use the full template, otherwise you will need at
least `Makefile.toml` and `index.html` from that repository.

This template should allow for a project that runs both natively and on the web
(but do read [Multi-Target](./web/multi-target.md), also), but you'll need
to adhere to a specific project setup..

First, install `cargo-make`:

```shell
cargo install cargo-make
```

Next, make sure your `Cargo.toml` looks like this:

```toml
[features]
default = [
  "bevy/bevy_gltf",
  "bevy/bevy_winit",
  "bevy/render",
  "bevy/png",
]

# feature used when building as native app
native = ["bevy/bevy_wgpu"]

# feature used when building as a WASM web app
web = ["bevy_webgl2"]

[dependencies]
bevy = {version="0.4.0", default-features=false}
bevy_webgl2 = {version="0.4.0", optional=true}
```

You can now use `cargo make` commands to run the application in two different
modes:

```shell
cargo make run # Run native version
cargo make --profile release build-native # Build native version

cargo make serve # Run WASM version: visit "http://127.0.0.1:4000/" to view
cargo make --profile release build-web # Build WASM version
```

## Deploying to the web

In order for other people to use your application on the web, you must host it
first. We'll explore using either `[GitHub Pages](https://pages.github.com/)` or a custom webpage to achieve this.

### Custom webpage

What files you need to serve with a webserver differs slightly between the
`cargo-make` and `wasm-pack` approaches, but the gist of it remains the same.
If your project has any assets, don't forget to serve these as well.

If you used `wasm-pack`, serve (found in the `/pkg` directory after building):

```
myproject_bg.wasm
myproject.js
```

If you used `cargo-make`, serve the following files in the same directory tree
as they're generated in:

```
|_ target
|  |_ wasm32-unknown-unknown
|  |  |_release
|  |    |_ myproject.d
|  |    |_ myproject.wasm
|  |_ wasm.js
|  |_ wasm_bg.wasm
|_ index.html
```

Of course, simply serving these files is not enough. You need to actually have
a webpage in which you call the generated javascript file. Simply add a HTML
script tag to your webpage:

If you used `wasm-pack` (be sure to rename files for your project):

```html
<script type="module">
  import init from './myproject.js';
  init('./myproject_bg.wasm').then(function (wasm) { wasm.run(); });
</script>
```
With `wasm-pack` you could also publish your project as a Node.js package or
set it up with Webpack, but that is considered out of scope for this guide.
Refer to the [wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/) for more information.

If you used `cargo-make`:

```html
<script type="module">
  import init from './wasm.js'
  init()
</script>
```

Note that the [bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template) already has this included in the example `index.html`.

### GitHub Pages

The most straightforward way to deploy a WASM app to GitHub Pages is from
within the same repository of the source code. You will need the same files
generated as in the [subsection](#Custom webpage) above.

1. Create an empty branch
```shell
git checkout --orphan web
git reset --hard
```
2. Put all files necessary for hosting there (see above section). Remember to add your assets if you have any.
3. Go to repository settings, scroll down to “GitHub Pages” section, then under “Source” pick the branch “web” and the `/` (root) folder. Then click “Save”.

For more details, visit the official [GitHub Pages documentation](https://guides.github.com/features/pages/);
