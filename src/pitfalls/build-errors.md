{{#include ../include/header09.md}}

# Strange Build Errors

Sometimes, you can get strange and confusing build errors when trying to
compile your project.

## Update your Rust

First, make sure your Rust is up-to-date. When using Bevy, you must use at
least the latest stable version of Rust (or nightly).

If you are using [`rustup`][rustup] to manage your Rust installation, you
can run:

```shell
rustup update
```

## Clear the cargo state

Many kinds of build errors can often be fixed by forcing `cargo` to regenerate
its internal state (recompute dependencies, etc.). You can do this by deleting
the `Cargo.lock` file and the `target` directory.

```shell
rm -rf target Cargo.lock
```

Try building your project again after doing this. It is likely that the
mysterious errors will go away.

This trick often fixes the broken build, but if it doesn't help you,
your issue might require further investigation. Reach out to the Bevy
community via GitHub or [Discord][bevy::discord], and ask for help.

If you are using bleeding-edge Bevy ("main"), and the above does not solve
the problem, your errors might be caused by 3rd-party plugins. See [this
page](../setup/bevy-git.md#how-to-use-bleeding-edge-bevy) for solutions.

{{#include ../include/resolver2.md}}

---

## What errors?

One common example is the "failed to select a version" error, which can
look something like this:

```
error: failed to select a version for `web-sys`.
    ... required by package `wgpu v0.9.0`
    ... which is depended on by `bevy_wgpu v0.5.0 (https://github.com/bevyengine/bevy#6a8a8c9d)`
    ... which is depended on by `bevy_internal v0.5.0 (https://github.com/bevyengine/bevy#6a8a8c9d)`
    ... which is depended on by `bevy v0.5.0 (https://github.com/bevyengine/bevy#6a8a8c9d)`
    ... which is depended on by `bevy-scratchpad v0.1.0 (C:\Users\Alice\Documentsevy-scratchpad)`
versions that meet the requirements `=0.3.50` are: 0.3.50

all possible versions conflict with previously selected packages.

  previously selected package `web-sys v0.3.46`
    ... which is depended on by `bevy_app v0.5.0 (https://github.com/bevyengine/bevy#6a8a8c9d)`
    ... which is depended on by `bevy_asset v0.5.0 (https://github.com/bevyengine/bevy#6a8a8c9d)`
    ... which is depended on by `bevy_audio v0.5.0 (https://github.com/bevyengine/bevy#6a8a8c9d)`
    ... which is depended on by `bevy_internal v0.5.0 (https://github.com/bevyengine/bevy#6a8a8c9d)`
    ... which is depended on by `bevy v0.5.0 (https://github.com/bevyengine/bevy#6a8a8c9d)`
    ... which is depended on by `bevy-scratchpad v0.1.0 (C:\Users\Alice\Documentsevy-scratchpad)`

failed to select a version for `web-sys` which could resolve this conflict
```

(there are many variations, yours might not be identical to the example above)

Another related error are seemingly-nonsensical compiler messages about
conflicts with Bevy's internal types (like "expected type `Transform`,
found type `Transform`").

## Why does this happen?

Such errors are often caused by `cargo`'s internal state being broken. Usually,
it is because of dependencies not being resolved properly, causing cargo to
try to link multiple versions of Bevy into your project. This often occurs when
transitioning your project between the release and the git version of Bevy. Cargo
remembers the versions it was previously using, and gets confused.
