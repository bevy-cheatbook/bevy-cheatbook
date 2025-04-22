{{#include ../include/header016.md}}

# Getting Started

This page covers the basic setup needed for Bevy development.

Also see the [Setup page in the official Bevy Book][bevy::getting-started]
and the [official Bevy Readme][bevy::readme].

---

## Prerequisites

For the most part, Bevy is just like any other Rust library. You need to
install Rust and setup your dev environment just like for any other Rust
project. You can install Rust using [Rustup][rustup]. See
[Rust's official setup page][rust::getting-started]. You also need various
other dependencies, depending on your operating system.

<details>
  <summary>Linux / UNIX</summary>

  - Follow the official instructions for [installing dependencies][bevy::linux-dependencies].
  - Run the [rustup] install script.

</details>

<details>
  <summary>Windows</summary>

  - Download and run the [rustup] installer. It will prompt you to install the Visual Studio build tools as part of the process.
  - Alternatively, you can install Visual Studio yourself: [installer](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
    - Easy setup: select "Desktop development with C++"
    - Minimal setup: under "Individual Components", select:
      - "MSVC" for your architecture and version of Windows
      - "Windows SDK" for your version of Windows
      - "C++ CMake tools"

</details>

<details>
  <summary>Mac</summary>

  - Install the XCode command line tools with `xcode-select --install` or the [Xcode app](https://apps.apple.com/en/app/xcode/id497799835)
  - Run the [rustup] install script.

</details>

If you are interested in mobile development, check out the pages on
[Android][platform::android] and [iOS][platform::ios].

If you are interested in web development, check out the [WASM][platform::wasm]
chapter.

## Creating a New Project

You can simply create a new Rust project, either from your IDE/editor, or the commandline:

```sh
cargo new --bin my_game
```

(creates a project called `my_game`)

The `Cargo.toml` file contains all the configuration of your project.
Add the latest version of `bevy` as a dependency.

```sh
cargo add bevy --features wayland
```

(I recommend enabling the non-default `wayland` feature, so your game can
better support modern Linux distros)

You can also add other [optional features][cb::features] to bevy, if you'd like.

Your `Cargo.toml` should now look something like this:

```toml
[package]
name = "my_game"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = { version = "0.16", features = ["wayland"] }
```

If you plan to customize bevy a lot, I recommend a dedicated dependency section.
It makes everything easier to read, compared to having everything on one line.

```toml
[dependencies.bevy]
version = "0.16"
features = [
  "wayland",
  # ...
]
```

The `src/main.rs` file is your main source code file. This is where you
start writing your Rust code. For a minimal Bevy [app][cb::app], you need
at least the following:

```rust,no_run,noplayground
{{#include ../code012/examples/minimal.rs}}
```

You can now compile and run your project. The first time, this will take a
while, as it needs to build the whole Bevy engine and dependencies. Subsequent
runs should be fast. You can do this from your IDE/editor, or the commandline:

```sh
cargo run
```

While this is enough to get you started, there are many tweaks you can do to
your [compiler / build toolchain configuration][cb::toolchain], for faster
compile times and better performance. At the very least, consider [enabling
optimizations][pitfall::perf].

## Documentation

You can generate your own docs (like what is on [docs.rs](https://docs.rs)),
for offline use, including everything from your own project and all
dependencies, all in one place.

```sh
cargo doc --open
```

This will build all the HTML docs and open them in your web browser.

It does not require an internet connection, and gives you an easy way to search
the API docs for all crates in your dependency tree all at once. It is more
useful than the online version of the docs.

## What's Next?

Have a look at the [guided tutorial][chapter::tutorial] page of this book
and Bevy's [official examples][bevy::examples].

Check out the [Bevy Assets Website][bevyassets] to find other tutorials,
learning resources from the community, and [plugins][cb::3rdparty] to use
in your project.

Join the community on [Discord][bevy::discord] to chat with us!

## Running into Issues?

If something is not working, be sure to check the [Common
Pitfalls][chapter::pitfalls] chapter, to see if this book has something to
help you. Solutions to some of the most common issues that Bevy community
members have encountered are documented there.

If you need help, use [GitHub Discussions][bevy::ghdiscussions], or feel
welcome to come chat and ask for help in [Discord][bevy::discord].

{{#include ../include/gpu-driver-requirements.md}}
