{{#include ../include/header010.md}}

# Windows Desktop

If you have any additional Windows-specific knowledge,
please help improve this page!

Create Issues or PRs on [GitHub][project::cb].

---

Windows is one of the best-supported platforms by Bevy.

Both the MSVC and the GNU compiler toolchains should work.

[You can also build Windows EXEs while working in Linux][cross::linux-windows].

## Distributing Your App

The EXE built with `cargo build` can work standalone without any extra files or DLLs.

Your `assets` folder needs be distributed alongside it. Bevy will search for it in
the same directory as the EXE on the user's computer.

The easiest way to give your game to other people to play is to put them
together in a ZIP file. If you use some other method of installation,
install the `assets` folder and the EXE to the same path.

If built with the MSVC toolchain, your users may need the Microsoft C/C++
Runtime Redistributables installed.

## Disabling the Windows Console

By default, when you run a Bevy app (or any Rust program for that matter)
on Windows, a Console window also shows up. To disable this,
place this Rust attribute at the top of your `main.rs`:

```rust,no_run,noplayground
#![windows_subsystem = "windows"]
```

This tells Windows that your executable is a graphical application, not a
command-line program. Windows will know not display a console.

However, the console can be useful for development, to see log messages.
You can disable it only for release builds, and leave it enabled in debug
builds, like this:

```rust,no_run,noplayground
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

## Creating an icon for your app

There are two places where you might want to put your application icon:
 - The EXE file (how it looks in the file explorer)
 - The window at runtime (how it looks in the taskbar and the window title bar)

### Setting the EXE icon

(adapted from [here][project::bevy_game_template])

The EXE icon can be set using a cargo build script.

Add a build dependency of `embed_resources` to your `Cargo.toml` allow embedding assets into your compiled executables
```toml
[build-dependencies]
embed-resource = "1.6.3"
```

Create a `build.rs` file in your project folder:

```rust,no_run,noplayground
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

See [Bevy Cookbook: Setting the Window Icon][cookbook::window-icon].
