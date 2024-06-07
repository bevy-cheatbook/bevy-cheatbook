{{#include ../../include/header-none.md}}

# Build Windows EXEs from Linux

(also check out the [Windows Platform page][platform::windows] for info
about developing for Windows generally)

If you are working in WSL2, please also see [this page for additional instructions][platform::windows::wsl2].

---

Rust offers two different toolchains for building for Windows:
 - [MSVC](#first-time-setup-msvc)
 - [GNU](#first-time-setup-gnu)

The instructions on this page use the `x86_64` architecture, but you could also
set up a toolchain to target `i686` (32-bit) or `aarch64` (Windows-on-Arm) the
same way.

## First-Time Setup (GNU)

On many Linux distros, the GNU/MINGW toolchain is the easier option. Your
distro likely provides packages that you can easily install. Also, you do
not need to accept any Microsoft licenses.

<details>
  <summary>
  Setup Instructions:
  </summary>

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

It might be called something like: `mingw-w64-x86-64-dev`, `cross-x86_64-w64-mingw32`, etc.,
the name varies in different distros.

You don't need any files from Microsoft.

</details>

## First-Time Setup (MSVC)

The MSVC toolchain is the native Microsoft way to target Windows. It is what
the Rust community usually recommends for targeting the Windows platform. It
may provide better compatibility with Windows DLLs / libraries and tooling.

Even though it is meant to be used on Windows, you can actually set it up
and use it on Linux (and other UNIX-like systems). It requires downloading
the Windows SDKs and accepting the Microsoft license. There is a script to
automate that for you.

<details>
  <summary>
  Setup Instructions:
  </summary>

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

</details>

## Building Your Project

Finally, with all the setup done, you can just build your Rust/Bevy projects
for Windows:

GNU:
```sh
cargo build --target=x86_64-pc-windows-gnu --release
```

MSVC:
```sh
cargo build --target=x86_64-pc-windows-msvc --release
```

## Bevy Caveats

As of Bevy 0.12, a workaround is needed for building with MSVC. If you
use the MSVC toolchain, the `blake3` dependency assumes you are building
on Windows and tries to run some EXEs during its build process, which do
not exist in the Linux cross-compilation environment. The solution is
to tell it to not do that and use pure Rust code instead.

Set an environment variable when building:

```sh
export CARGO_FEATURE_PURE=1
cargo build --target=x86_64-pc-windows-msvc --release
```

Or add `blake3` to your `Cargo.toml` if you want to persist the configuration:

```toml
[dependencies]
blake3 = { version = "1.5", features = [ "pure" ] }
```
