{{#include ../include/header011.md}}

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
