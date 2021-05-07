# Cargo make as an Alternative

`cargo-make` is a task runner that lets you define and configure a set of tasks
in a script to execute upon building. It can not only be used to build for the
web and generate the necessary JavaScript files, but even to quickly
serve a webpage for development and testing. The downside here is that we must
define all this behavior ourselves by creating a build script. There are
templates you can use, but these may be opinionated in terms of project
structure required.

While broader in scope than `wasm-pack`, you can use `cargo-make` as a replacement for it.

### Building with `cargo-make`

To get `cargo-make` building for the web, you need a build manifest.
Writing this file is out of scope for this guide, but luckily the author of
`bevy_webgl2` has provided [bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template) which you can use. If you are starting your
project from scratch you can use the full template, otherwise you will need at
least `Makefile.toml` and `index.html` from that repository.

This template should allow for a project that runs both natively and on the
web, but you'll need to adhere to a specific project setup that is different
from the [quick-start guide](./web/quick-start.md).

First, use Cargo to install `cargo-make`:

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

Serve the following files in the same directory tree as they're generated in:

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

And add the following to your `index.html`:

```html
<script type="module">
  import init from "./wasm.js";
  init();
</script>
```

Note that the [bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template) already has this included in the example `index.html`.
