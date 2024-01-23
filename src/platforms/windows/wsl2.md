{{#include ../../include/header012.md}}

## Working in WSL2

If you prefer to have a more Linux-centric development workflow, you might want
to work inside of WSL2 and build your project there. Another reason to do it is
compile times; they are often much faster in WSL2 than on the Windows host
system. Linux has faster I/O and filesystem than Windows, and that makes a big
difference to compile times.

### Cross-compiling to run Windows Native

The recommended way to run your Bevy app from WSL is to [cross-compile for
Windows][cross::linux-windows]. The Windows EXE you build inside of WSL2 can
be run just fine from the Linux commandline, and it will seamlessly run on the
host system! This way, you don't need any GPU drivers or GUI support inside
your WSL2 Linux environment. Also, you will be running and testing the Windows
build of your game, so you can see how it will really perform on Windows.
It will run with full performance and use your host Windows GPU drivers.

Note that when you run Windows binaries from WSL2, they don't get the Linux
environment variables. `cargo run` does not just work, because your Bevy game
will look for its `assets` folder in the path where the EXE is (which would be
in the `target` build output folder). My simple solution is to just copy the
EXE into the project folder after building, and run it directly from there.
For non-Bevy Rust projects, this would be unnecessary.

The process can be automated with a little script, to use instead of `cargo run`:

```sh
#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/mygame.exe . &&
exec ./mygame.exe "$@"
```

This way you also don't have to type the cross-compilation target every time
(and you can also add any other options you want there).

Just save the script (you can call it something like `win.sh`) and run your
game as:

```sh
./win.sh
```

### Running Linux builds using WSLg

This is an alternative way of running your Bevy game from WSL. It does not
require cross-compilation, but is likely to have other issues and limitations.

Newer installs of WSL2 should have support for WSLg: Microsoft's Linux GUI
support. It should allow you to simply compile your Bevy game in Linux and
run it. WSLg will do the dark magic needed to forward graphics and audio to
the Windows host.

Your game will run locked to 60 FPS, and there will be other performance
problems. WSLg is effectively RDP (Remote Desktop) under the hood. It's
like streaming video from the VM to the host. Some functionality might be
broken/unsupported.

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

Now, you can simply run your Bevy project in Linux in the usual way:

```sh
cargo run
```
