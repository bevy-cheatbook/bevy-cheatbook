{{#include ../include/header014.md}}

# Strange Build Errors

Sometimes, you can get strange and confusing build errors when trying to
compile your project.

If none of the advice on this page helps you, your issue might require
further investigation. Reach out to the Bevy community via GitHub or
[Discord][bevy::discord], and ask for help.

If you are using bleeding-edge Bevy ("main"), also see [this
page](../setup/bevy-git.md#how-to-use-bleeding-edge-bevy) for advice.

## Update your Rust

First, make sure your Rust is up-to-date. Bevy only officially supports the
latest stable version of Rust at the time the Bevy version you are using
was released, or nightly.

If you are using [`rustup`][rustup] to manage your Rust installation, you
can run:

```sh
rustup update
```

## Clear the cargo state

Many kinds of build errors can often be fixed by forcing `cargo` to regenerate
its internal state (recompute dependencies, etc.). You can do this by deleting
the `Cargo.lock` file and the `target` directory.

```sh
rm -rf target Cargo.lock
```

Try building your project again after doing this. It is likely that the
mysterious errors will go away.

## Multiple versions of dependencies

If not, another reason might be that you have multiple versions of Bevy
(or other dependencies) in your dependency tree. Rust/cargo allows multiple
versions of the same crate to be linked at the same time into the same
executable.

If you are using 3rd-party plugins, make sure you have specified the correct
versions of all the plugins you use and that they are compatible with the
Bevy version you are using. If you depend on a plugin that uses a different
version of Bevy from the one you are using, they will not be interoperable.

You will get compiler errors like:

```
error[E0308]: mismatched types
  --> src/main.rs:12:20
   |
12 |         transform: Transform::from_xyz(1.0, 2.0, 3.0),
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Transform`, found a different `Transform`
   |
   = note: `Transform` and `Transform` have similar names, but are actually distinct types
note: `Transform` is defined in crate `bevy_transform`
  --> /home/iyes/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_transform-0.14.0-rc.2/src/components/transform.rs:43:1
   |
43 | pub struct Transform {
   | ^^^^^^^^^^^^^^^^^^^^
note: `Transform` is defined in crate `bevy_transform`
  --> /home/iyes/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_transform-0.12.1/src/components/transform.rs:41:1
   |
41 | pub struct Transform {
   | ^^^^^^^^^^^^^^^^^^^^
   = note: perhaps two different versions of crate `bevy_transform` are being used?

```

Or perhaps errors about common Bevy traits like `Component`, `Bundle`, or `Plugins`
not being implemented on types that clearly should have them.

{{#include ../include/resolver2.md}}
