{{#include ../include/header012.md}}

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

### DXC Compiler Support

Bevy (technically `wgpu`) supports using the Microsoft DXC compiler for
improved shader compilation when using DirectX 12.

To do this, you need to [download it from Microsoft's
repo][project::dxc::download] and put `dxcompiler.dll` and `dxil.dll`
alongside your game's EXE.

Bevy should detect these DLL files automatically and use them.

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

## Working in WSL2

If you prefer to have a more Linux-centric development workflow, you might want
to work inside of WSL2 and build your project there. Another reason to do it is
compile times; they are often much faster in WSL2 than on the Windows host
system.

### Cross-compiling to run Windows Native

The recommended way to run your Bevy app from WSL is to [cross-compile for
Windows][cross::linux-windows]. The Windows EXE you build inside of WSL2 can
be run just fine from the Linux commandline, and it will seamlessly run on the
host system! This way, you don't need any GPU drivers or GUI support inside
your WSL2 Linux environment. Also, you will be running and testing the Windows
build of your game, so you can see how it will really perform on Windows.

Note that when you run Windows binaries from WSL2, they don't get the Linux
environment variables. `cargo run` does not just work, because your Bevy game
will look for its `assets` folder in the path where the EXE is (which would be
in the `target` build output folder). My simple solution is to just copy the
EXE into the project folder after building, and run it directly from there.

This can be automated with a little script, to use instead of `cargo run`:

```sh
#!/bin/sh
cargo build --target x86_64-pc-windows-msvc &&
cp target/x86_64-pc-windows-msvc/debug/mygame.exe . &&
exec ./mygame.exe "$@"
```

This way you also don't have to type the cross-compilation target every time
(and you can also add any other options you want there).

### Running Linux builds using WSLg

This is an alternative way of running your Bevy game from WSL. It does not
require cross-compilation, but is likely to have other issues and limitations.

Newer installs of WSL2 should have support for WSLg: Microsoft's Linux GUI
support. It should allow you to simply compile your Bevy game in Linux and
run it. WSLg will do the dark magic needed to forward graphics and audio to
the Windows host.

Your game will only run with 60 FPS, and there will be other performance
overheads. Some things might be broken/unsupported.

Both Wayland and X11 should work. Wayland is recommended, so be sure to
enable the `"wayland"` [cargo feature in Bevy][cb::features].

There are many dependencies (such as graphics drivers) needed for GUI support
in Linux, which are likely missing if you have never used any other GUI app
from your WSL environment. The easiest way to make sure you have everything installed,
is to just install some random Linux GUI app. For example:

```sh
sudo apt install gucharmap # the GNOME Character Map app
```

It will pull in everything needed for a Linux GUI environment. Bevy should then
also be able to work.

This will be sufficient for OpenGL support. However, to use all features of
Bevy, you need Vulkan. For Vulkan in WSL, it is recommended that you use
a PPA (unofficial repository) to get an updated version of Mesa (graphics
drivers). Here is how to install everything:

```sh
sudo add-apt-repository ppa:kisak/kisak-mesa
sudo apt update
sudo apt upgrade
sudo apt install vulkan-tools
```

(`dzn`, Microsoft's Vulkan driver for WSL2, is technically non-conformant,
so there may be bugs and other issues, but it seems to work fine)

Now, you can simply run your Bevy project in Linux:

```sh
cargo run
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

See: [Setting the Window Icon][cookbook::window-icon].
