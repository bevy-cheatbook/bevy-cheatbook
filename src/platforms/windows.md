# Windows Desktop

If you have any additional Windows-specific knowledge,
please help improve this page!

Create Issues or PRs on
[GitHub](https://github.com/bevy-cheatbook/bevy-cheatbook).

---

Windows is one of the best-supported platforms by Bevy.

Both the MSVC and the GNU compiler toolchains should work.

## Distributing Your App

The EXE built with `cargo build` can work standalone without any extra files or DLLs.

Your `assets` folder needs be distributed alongside it. Bevy will search for it in
the same directory as the EXE on the user's computer.

The easiest way to give your game to other people to play is to put them
together in a ZIP file. If you use some other method of installation,
install the `assets` folder and the EXE to the same path.

## Creating an icon for your app

There are two places where you might want to put your application icon:
 - The EXE file (how it looks in the file explorer)
 - The window at runtime (how it looks in the taskbar and the window title bar)

### Setting the EXE icon

(adapted from [here](https://github.com/NiklasEi/bevy_game_template))

The EXE icon can be set using a cargo build script.

Add a build dependency of `embed_resources` to your `Cargo.toml` allow embedding assets into your compiled executables
```toml
[build-dependencies]
embed-resource = "1.6.3"
```

Create a `build.rs` file in your project folder:

```rust
extern crate embed_resource;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("icon.rc");
    }
}
```

Create a `icon.rc` file in your project folder:

```
app_icon ICON "icon.ico"
```

Create your icon as `icon.ico` in your project folder.

### Setting the Window Icon

See [Bevy Cookbook: Setting the Window Icon](../cookbook/window-icon.md).

