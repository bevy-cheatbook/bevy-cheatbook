# WASM related tips

## Dynamic fullscreen

To enable *fullscreen* in bevy wasm you need to resize the bevy `Window` acccording to the browser size.

To do this you will need to register the following `system` (only for `wasm32` arch):

```rust
fn main() {
    let mut app = App::new();
    // ...
    // Register your systems and plugins
    // ...
    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize);
    // Run the app
    app.run();
}


#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
    }
}
```

[`web_sys`](https://crates.io/crates/web-sys) will provide you with the brower window and its *inner_size*.
Unfortunately there is not yet any possible [change detection](../../programming/change-detection.md) and this system will run every frame.

## Using a custom `index.html`

This is useful to define extra behaviour using Javascript or to add custom HTML elements to your page.

The basic page would look like this:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <title>My Bevy App</title>
</head>
<body style="margin: 0 !important; padding: 0 !important;">
<script type="module">
    // Import and run your bevy wasm code
    import init from './$APP.js'
    init()
</script>
</body>
</html>
```

> `$APP` represents your crate name

Using [wasm-bindgen](https://crates.io/crates/wasm-bindgen-cli) the complete set of instructions is the following:

- `rustup target add wasm32-unknown-unknown`: Adds the wasm rustup target
- `cargo install wasm-bindgen-cli`:  Installs `wasm-bindgen`
- `cargo build --release --target wasm32-unknown-unknown`: Builds your bevy app $APP in `release` mode to `target/wasm32-unknown-unknown/release/$APP.wam`
- `mkdir public`: creates the directory you can then put on your server
- `cp index.html public`: moves the html index to the public directory
- `cp -r assets public`: copies all of your bevy `assets` folder into the public directory 
- `wasm-bindgen --out-dir public --target web target/wasm32-unknown-unknown/release/$APP.wasm`: generate the bindings for your native `wasm` build 

The last instruction will create the required `$APP.js` file for your `index.html` to load

### Disable the right click default behaviour

If your bevy game detects *right clicks* for some gameplay, on native wasm unfortunately you will have the browser context menu popping.

A solution is adding a basic Javascript instruction to your custom `index.html` file:

```diff
<!DOCTYPE html>
<html lang="en">
<head>
    <title>My Bevy App</title>
</head>
<body style="margin: 0 !important; padding: 0 !important;">
<script type="module">
    // Import and run your bevy wasm code
    import init from './$APP.js'
    init()

+    // This disables the basic right click behaviour
+    document.addEventListener('contextmenu', function (evt) {
+        evt.preventDefault();
+    })
</script>
</body>
</html>
```
