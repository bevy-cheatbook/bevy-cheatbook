{{#include ../include/header013.md}}

# Getting Started

This page covers the basic setup needed for Bevy development.

---

For the most part, Bevy is just like any other Rust library. You need to
install Rust and setup your dev environment just like for any other Rust
project. You can install Rust using [Rustup][rustup]. See
[Rust's official setup page][rust::getting-started].

On Linux, you need the development files for some system libraries. See the
[official Bevy Linux dependencies page][bevy::linux-dependencies].

Also see the [Setup page in the official Bevy Book][bevy::getting-started]
and the [official Bevy Readme][bevy::readme].

## Creating a New Project

You can simply create a new Rust project, either from your IDE/editor, or the commandline:

```sh
cargo new --bin my_game
```

(creates a project called `my_game`)

The `Cargo.toml` file contains all the configuration of your project.
Add the latest version of `bevy` as a dependency. Your file should now
look something like this:

```toml
[package]
name = "my_game"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.13"
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

## Documentation

You can generate your own docs (like what is on [docs.rs](https://docs.rs)), for
offline use, including everything from your own project and all dependencies, in
one place.

```sh
cargo doc --open
```

This will build all the HTML docs and open them in your web browser.

It does not require an internet connection, and gives you an easy way to search
the API docs for all crates in your dependency tree all at once. It is more
useful than the online version of the docs.

## Optional Extra Setup

You will likely quickly run into unusably slow performance with the default
Rust unoptimized dev builds. [See here how to fix.][pitfall::perf]

Iterative recompilation speed is important to keep you productive, so you don't
have to wait long for the Rust compiler to rebuild your project every time you
want to test your game. [Bevy's getting started page][bevy::getting-started]
has advice about how to speed up compile times.

Also have a look in the [Dev Tools and Editors][cb::tools] page for suggestions
about additional external dev tools that may be helpful.

## What's Next?

Have a look at the [guided tutorial][chapter::tutorial] page of this book,
and Bevy's [official examples][bevy::examples].

Check out the [Bevy Assets Website][bevy::assets] to find other tutorials
and learning resources from the community, and [plugins][cb::3rdparty]
to use in your project.

Join the community on [Discord][bevy::discord] to chat with us!

## Running into Issues?

If something is not working, be sure to check the [Common
Pitfalls][chapter::pitfalls] chapter, to see if this book has something to
help you. Solutions to some of the most common issues that Bevy community
members have encountered are documented there.

If you need help, use [GitHub Discussions][bevy::ghdiscussions], or feel
welcome to come chat and ask for help in [Discord][bevy::discord].

{{#include ../include/gpu-driver-requirements.md}}
