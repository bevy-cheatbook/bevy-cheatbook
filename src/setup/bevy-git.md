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

## Should you use bleeding-edge Bevy? What version of Bevy should you use?

Bevy follows a "train release" model, with loose deadlines. Every 3 months,
a new major release is prepared, which will contain all new developments
(features, fixes, etc.) since the last release. The release date is not
strict and is often delayed by a few weeks to tie up loose ends.

Further, Bevy usually follows up every major release with a patch release
or two, as needed, to fix any bugs discovered soon after release. It will
not contain all fixes, just small non-breaking things that are considered
critical enough.

Most Bevy projects should use the latest release on crates.io. If you want
to play it safe, you can wait until the first patch release (`0.*.1`),
before upgrading to a new major version. You might also want to wait for
any 3rd-party plugins you are using to support the new Bevy version.

On the other hand, for experimentation and for Bevy development, you are
encouraged to try the latest in-development code from git! The latest
release is often missing the freshest bug fixes, usability improvements,
and features. It may be compelling to join in on the action!

If you are new to Bevy, this might not be for you. You might be more
comfortable using the released version. It will have the best compatibility
with community plugins and documentation.

The in-development version of Bevy has frequent breaking changes. Therefore,
it can be very annoying to use for real projects. Also, 3rd-party plugin
authors often don't bother to stay compatible. You will face breakage often
and probably have to fix it yourself.

It is only recommended to do this for more experimental or toy projects.

Though, there are ways you can manage the breakage and make it less of a
problem. Thanks to cargo, you can update bevy at your convenience, whenever you
feel ready to handle any possible breaking changes.

You may want to consider forking the repositories of Bevy and any plugins you
use. Using your own forks allows you to easily apply fixes if needed, or edit
their `Cargo.toml` for any special configuration to make your project work.

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

If you are using any 3rd-party plugins, please consider forking them, so you can
edit their `Cargo.toml` and have control over how everything is configured.

## Cargo Patches

In some cases, you might be able to use "cargo patches" to locally override
dependencies. For example, you might be able to point plugins to use your
fork of bevy, without forking and editing the plugin's `Cargo.toml`, by
doing something like this:

```toml
# replace the bevy git URL source with ours
[patch."https://github.com/bevyengine/bevy"]
# if we have our own fork
bevy = { git = "https://github.com/me/bevy" }
# if we want to use a local path
bevy = { path = "../bevy" }
# some plugins might depend on individual bevy crates,
# instead of all of bevy, which means we need to patch
# every individual bevy crate specifically:
bevy_ecs = { path = "../bevy/crates/bevy_ecs" }
bevy_app = { path = "../bevy/crates/bevy_app" }
# ...

# replace released versions of crates (crates.io source) with ours
[patch.crates-io]
bevy_some_plugin = { git = "https://github.com/me/bevy_some_plugin", branch = "bevy_main" }
# also replace bevy itself
bevy = { path = "../bevy" }
# ...
```

## Updating Bevy

It is recommended that you specify a known-good Bevy commit in your
`Cargo.toml`, so that you can be sure that you only update it when you
actually want to do so, avoiding unwanted breakage.

```toml
bevy = { git = "https://github.com/bevyengine/bevy", rev = "7a1bd34e" }
```

When you change anything, be sure to run:

```sh
cargo update
```

(or delete `Cargo.lock`)

Otherwise you risk errors from cargo not resolving dependencies correctly.

## Advice for plugin authors

If you are publishing a plugin crate, here are some recommendations:
  - Use the main branch in your repository for targeting the released version of Bevy
  - Have a separate branch in your repository, to keep support for bevy main
    separate from your version for the released version of bevy
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
        run: sudo apt-get update && sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Check code
        run: cargo update && cargo check --lib --examples
```
