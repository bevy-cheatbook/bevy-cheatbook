{{#include ../include/header-main.md}}

# Using bleeding-edge Bevy (bevy main)

Bevy development moves very fast, and there are often exciting new things that
are yet unreleased. This page will give you advice about using development
versions of bevy.

## Quick Start

If you are *not* using any 3rd-party plugins and just want to use the bevy
main development branch:

```toml
[dependencies]
bevy = { git = "https://github.com/bevyengine/bevy" }
```

However, if you *are* working with external plugins, you should read the rest
of this page. You will likely need to do more to make everything compatible.

## Should you use bleeding-edge Bevy?

Currently, Bevy does not make patch releases (with rare exceptions for
critical bugs), only major releases. The latest release is often missing
the freshest bug fixes, usability improvements, and features. It may be
compelling to join in on the action!

If you are new to Bevy, this might not be for you. You might be more
comfortable using the released version. It will have the best compatibility
with community plugins and documentation.

The biggest downside to using unreleased versions of Bevy is 3rd-party plugin
compatibility. Bevy is unstable and breaking changes happen often. However,
many actively-maintained community plugins have branches for tracking the
latest Bevy main branch, although they might not be fully up-to-date. It's
possible that a plugin you want to use does not work with the latest changes
in Bevy main, and you may have to fix it yourself.

The frequent breaking changes might not be a problem for you, though. Thanks
to cargo, you can update bevy at your convenience, whenever you feel ready
to handle any possible breaking changes.

You may want to consider forking the repositories of any plugins you use.
This allows you to easily apply fixes if needed, or edit their `Cargo.toml`
for any special configuration to make your project work.

If you choose to use Bevy main, you are highly encouraged to interact with
the Bevy community on [Discord][bevy::discord] and [GitHub][project::bevy], so
you can keep track of what's going on, get help, or participate in discussions.

## Common pitfall: mysterious compile errors

When changing between different versions of Bevy (say, transitioning an existing
project from the released version to the git version), you might get lots of
strange unexpected build errors.

You can typically fix them by removing `Cargo.lock` and the `target` directory:

```sh
rm -rf Cargo.lock target
```

See [this page][pitfall::build-errors] for more info.

If you are still getting errors, it is probably because cargo is trying
to use multiple different versions of bevy in your dependency tree
simultaneously. This can happen if some of the plugins you use have specified
a different Bevy version/commit from your project.

Make sure you use the correct branch of each plugin you depend on, with support
for Bevy main. If you have your own forks, check that the dependencies are
correctly and consistently specified everywhere.

If you have issues, they might still be fixable. Read the next section
below for advice on how to configure your project in a way that minimizes
the chances of this happening.

## How to use bleeding-edge bevy?

```toml
[dependencies]
# recommended: specify a known-working commit hash to pin to
# (specify it in the URL, to make the patch tricks below work)
bevy = { git = "https://github.com/bevyengine/bevy?rev=a420beb0" }

# add any 3rd-party plugins you use, and make sure to use the correct branch
# (alternatively, you could also specify a commit hash, with "rev")
bevy_thing = { git = "https://github.com/author/bevy_thing?branch=bevy_main" }

# For each plugin we use, patch them to use the same bevy commit as us:

# If they have specified a different commit:
# (you need to figure this out)
[patch."https://github.com/bevyengine/bevy?rev=146123ea"] # their bevy commit
bevy = { git = "https://github.com/bevyengine/bevy?rev=a420beb0" } # ours

# For those that have not specified anything:
[patch."https://github.com/bevyengine/bevy"]
bevy = { git = "https://github.com/bevyengine/bevy?rev=a420beb0" }
```

Some 3rd-party plugins depend on specific bevy sub-crates, rather than the
full bevy. You may additionally have to patch those individually:

```toml
[patch."https://github.com/bevyengine/bevy"]
# specific crates as needed by the plugins you use (check their `Cargo.toml`)
bevy_ecs = { git = "https://github.com/bevyengine/bevy?rev=a420beb0" }
bevy_math = { git = "https://github.com/bevyengine/bevy?rev=a420beb0" }
# ... and so on
```

To collect all the information you need, in order to fully patch all your
dependencies, you can either look at their `Cargo.toml`, or figure it out
by running `cargo tree` or searching inside your `Cargo.lock` file for
duplicate entries (multiple copies of bevy crates).

Make sure to delete `Cargo.lock` every time you make a change to your
dependencies configuration, to force cargo to resolve everything again.

## Updating Bevy

It is recommended that you specify a known-good Bevy commit in your
`Cargo.toml`, so that you can be sure that you only update it when you
actually want to do so, avoiding unwanted breakage.

```toml
bevy = { git = "https://github.com/bevyengine/bevy?rev=7a1bd34e" }
```

Even if you do not, the `Cargo.lock` file always keeps track of the exact
version (including the git commit) you are working with. You will not be
affected by new changes in upstream bevy or plugins, until you update it.

To update, run:
```sh
cargo update
```

or delete `Cargo.lock`.

Make sure you do this every time you change the configuration in your
`Cargo.toml`. Otherwise you risk errors from cargo not resolving dependencies
correctly.

## Advice for plugin authors

If you are publishing a plugin crate, here are some recommendations:
  - Have a separate branch in your repository, to keep support for bevy main
    separate from your main version for the released version of bevy
  - Put information in your README to tell people how to find it
  - Set up CI to notify you if your plugin is broken by new changes in bevy

Feel free to follow all the advice from this page, including cargo patches
as needed. Cargo patches only apply when you build your project directly,
not as a dependency, so they do not affect your users and can be safely kept
in your `Cargo.toml`.

### CI Setup

Here is an example for GitHub Actions. This will run at 8:00 AM (UTC) every day
to verify that your code still compiles. GitHub will notify you when it fails.

```yaml
name: check if code still compiles

on:
  schedule:
    - cron: '0 8 * * *'

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v2

      - name: Install Dependencies
        run: sudo apt-get update && sudo apt-get install --no-install-recommends pkg-config libx11-dev libasound2-dev libudev-dev

      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Check code
        run: cargo update && cargo check --lib --examples
```
