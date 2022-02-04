# Build Windows EXEs from Linux

{{#include ../../include/links.md}}

(also check out the [Windows Platform page][platform::windows] for info
about developing for Windows generally)

---

## First-Time Setup

### Rust Toolchain

You can actually use the same MSVC-based Rust toolchain, that is the standard
when working on Windows, from Linux.

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

For example, to install to `/opt/xwin/`:

```sh
xwin --accept-license 1 splat --output /opt/xwin
```

### Linking

Rust needs to know how to link the final EXE file.

The default Microsoft linker (`link.exe`) is not available on Linux. Instead,
we need to use the LLD linker (this is also recommended when working on
Windows anyway). Just install the `lld` package from your Linux distro.

We also need to tell Rust the location of the Microsoft Windows SDK libraries
(that were installed with `xwin` in [the previous step](#microsoft-windows-sdks)).

Add this to `.cargo/config.toml` (in your home folder or in your bevy project):

```toml
[target.x86_64-pc-windows-msvc]
linker = "lld"
rustflags = [
  "-Lnative=/opt/xwin/crt/lib/x86_64",
  "-Lnative=/opt/xwin/sdk/lib/um/x86_64",
  "-Lnative=/opt/xwin/sdk/lib/ucrt/x86_64"
]
```

## Building Your Project

Finally, with all the setup done, you can just build your Rust/Bevy projects
for Windows:

```sh
cargo build --target=x86_64-pc-windows-msvc --release
```

