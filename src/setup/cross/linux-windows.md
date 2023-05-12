{{#include ../../include/header010.md}}

# Build Windows EXEs from Linux

(also check out the [Windows Platform page][platform::windows] for info
about developing for Windows generally)

---

Rust offers two different toolchains for building for Windows:
 - [MSVC](#first-time-setup-msvc): the default when working in Windows, requires downloading Microsoft SDKs
 - [GNU](#first-time-setup-gnu): alternative MINGW-based build

The instructions on this page use the `x86_64` architecture, but you could also
set up a toolchain to target `i686` (32-bit) or `aarch64` (Windows-on-Arm) the
same way.

## First-Time Setup (MSVC)

The MSVC toolchain is what the Rust community usually recommends for targetting
the Windows platform. You can actually set it up and use it on Linux (and other
UNIX-like systems), using some special tooling, which will be explained below.

### Rust Toolchain (MSVC)

Add the target to your Rust installation (assuming you use [`rustup`][rustup]):

```sh
rustup target add x86_64-pc-windows-msvc
```

This installs the files Rust needs to compile for Windows, including the
Rust standard library.

### Microsoft Windows SDKs

You need to install the Microsoft Windows SDKs, just like when working on
Windows. On Linux, this can be done with an easy script called `xwin`. You
need to accept Microsoft's proprietary license.

Install `xwin`:

```sh
cargo install xwin
```

Now, use `xwin` to accept the Microsoft license, download all the files
from Microsoft servers, and install them to a directory of your choosing.

(The `--accept-license` option is to not prompt you, assuming you have already
seen the license. To read the license and be prompted to accept it, omit that
option.)

To install to `.xwin/` in your home folder:

```sh
xwin --accept-license splat --output /home/me/.xwin
```

### Linking (MSVC)

Rust needs to know how to link the final EXE file.

The default Microsoft linker (`link.exe`) is only available on Windows. Instead,
we need to use the LLD linker (this is also recommended when working on Windows
anyway). Just install the `lld` package from your Linux distro.

We also need to tell Rust the location of the Microsoft Windows SDK libraries
(that were installed with `xwin` in [the previous step](#microsoft-windows-sdks)).

Add this to `.cargo/config.toml` (in your home folder or in your bevy project):

```toml
[target.x86_64-pc-windows-msvc]
linker = "lld"
rustflags = [
  "-Lnative=/home/me/.xwin/crt/lib/x86_64",
  "-Lnative=/home/me/.xwin/sdk/lib/um/x86_64",
  "-Lnative=/home/me/.xwin/sdk/lib/ucrt/x86_64"
]
```

Note: you need to specify the correct full absolute paths to the SDK files,
wherever you installed them.

## First-Time Setup (GNU)

On many Linux distros, the alternative GNU/MINGW toolchain might be an easier
option. Your distro might provide packages that you can easily install. Also,
you do not need to accept any Microsoft licenses.

### Rust Toolchain (GNU)

Add the target to your Rust installation (assuming you use [`rustup`][rustup]):

```sh
rustup target add x86_64-pc-windows-gnu
```

This installs the files Rust needs to compile for Windows, including the
Rust standard library.

### MINGW

The GNU toolchain requires the MINGW environment to be installed. Your distro likely
provides a package for it. Search your distro for a cross-compilation mingw package.

It might be called something like: `cross-x86_64-w64-mingw32`, but that varies in different distros.

You don't need any files from Microsoft.

## Building Your Project

Finally, with all the setup done, you can just build your Rust/Bevy projects
for Windows:

MSVC:
```sh
cargo build --target=x86_64-pc-windows-msvc --release
```

MinGW:
```sh
cargo build --target=x86_64-pc-windows-gnu --release
```
