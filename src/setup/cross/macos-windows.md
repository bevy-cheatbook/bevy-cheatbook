{{#include ../../include/header-none.md}}

# Build Windows EXEs from macOS

(also check out the [Windows Platform page][platform::windows] for info
about developing for Windows generally)

**Note: As of Bevy 0.12, the instructions on this page no longer work!**

There are some dependencies that assume you are running Windows and they
can run Windows EXEs from their build scripts, which is not the case when
cross-compiling.

However, I decided to leave this page in the book, because it could be useful
for other Rust projects (that do not use Bevy or something else that pulls
in broken dependencies).

---

Rust offers two different toolchains for building for Windows:
 - [MSVC](#first-time-setup-msvc)
 - [GNU](#first-time-setup-gnu)

On macOS, the GNU software is not readily-available. I don't know how it could
be installed. Even if possible, it might be difficult to set up.

This page will teach you how to setup a MSVC-based toolchain, which works on
macOS and [can be set up similarly to how it can be done on
Linux.][cross::linux-windows]. You will need to accept Microsoft licenses.

The instructions on this page use the `x86_64` architecture, but you could also
set up a toolchain to target `i686` (32-bit) or `aarch64` (Windows-on-Arm) the
same way.

## First-Time Setup

### Rust Toolchain

Add the target to your Rust installation (assuming you use [`rustup`][rustup]):

```sh
rustup target add x86_64-pc-windows-msvc
```

This installs the files Rust needs to compile for Windows, including the
Rust standard library.

### Microsoft Windows SDKs

You need to install the Microsoft Windows SDKs, just like when working on
Windows. This can be done with an easy script called `xwin`. You need to accept
Microsoft's proprietary license.

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
xwin --accept-license splat --disable-symlinks --output /Users/me/.xwin
```

On Windows and macOS, the filesystem is case-insensitive. On Linux and BSD, the
filesystem is case-sensitive. `xwin` was made for Linux, so it tries to work
around this by default, by creating symlinks. On macOS, we need to tell `xwin`
not to do this, using the `--disable-symlinks` option.

### Linking

Rust needs to know how to link the final EXE file.

The default Microsoft linker (`link.exe`) is only available on Windows. Instead,
we need to use the LLD linker (this is also recommended when working on Windows
anyway).

#### Installing LLD

Unfortunately, last I checked, neither `brew` nor `macports` offer packages (LLD
is not commonly used when developing for macOS).

We can, however, build it ourselves from source. You need a C++ compiler and
CMake. You probably already have the C++ toolchain installed, if you have
installed Apple XCode development tools.

CMake can be installed from `brew` ([Homebrew][project::homebrew]):

```sh
brew install cmake
```

Now, we are ready to compile LLD from the LLVM project:

Note: the `--depth=1` option to `git clone` allows us to save a lot of disk
space and download bandwidth, because the LLVM respository is *huge*.

```sh
git clone --depth=1 https://github.com/llvm/llvm-project
cd llvm-project
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_PROJECTS=lld -DCMAKE_INSTALL_PREFIX=/usr/local ../llvm
sudo make -j10 install # adjust `-j10` based on your number of CPU cores
cd ../../; rm -rf llvm-project # delete the git repo and build files to free disk space
```

This will install it to `/usr/local`. Change the path above if you would rather
have it somewhere else, to not pollute your macOS or need `sudo` / root privileges.

#### Using LLD

We also need to tell Rust to use our linker, and the location of the Microsoft
Windows SDK libraries (that were installed with `xwin` in [the previous
step](#microsoft-windows-sdks)).

Add this to `.cargo/config.toml` (in your home folder or in your bevy project):

```toml
[target.x86_64-pc-windows-msvc]
linker = "/usr/local/bin/lld"
rustflags = [
  "-Lnative=/Users/me/.xwin/crt/lib/x86_64",
  "-Lnative=/Users/me/.xwin/sdk/lib/um/x86_64",
  "-Lnative=/Users/me/.xwin/sdk/lib/ucrt/x86_64"
]
```

Note: you need to specify the correct full absolute paths to the SDK files,
wherever you installed them.

## Building Your Project

Finally, with all the setup done, you can just build your Rust/Bevy projects
for Windows:

```sh
cargo build --target=x86_64-pc-windows-msvc --release
```
