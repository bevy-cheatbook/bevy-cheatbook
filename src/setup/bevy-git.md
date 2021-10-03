# Using bleeding-edge Bevy (bevy main)

Bevy development moves very fast, and there are often exciting new things that
are yet unreleased. This page will give you advice about using development
versions of bevy.

## Should you use bleeding-edge bevy?

Currently, bevy does not make patch releases, only major releases when there
are enough exciting new changes for a flashy press release. The latest release
is often missing the latest bug fixes, usability improvements, and features. It
may be compelling to join in on the action!

If you are new to bevy, this might not be for you; you might be more
comfortable using the released version. It will have the best compatibility
with community plugins and documentation.

The biggest downside to using unreleased versions of bevy is 3rd-party plugin
compatibility. Bevy is unstable and breaking changes happen often. However,
many actively-maintained community plugins have branches for tracking the
latest bevy main branch, although they might not be fully up-to-date. It's
possible that a plugin you want to use does not work with the latest changes
in bevy main, and you may have to fix it yourself.

The frequent breaking changes might not be a problem for you, though. Thanks
to cargo, you can update bevy at your convenience, whenever you feel ready
to handle any possible breaking changes.

If you choose to use bevy main, you are highly encouraged to
interact with the Bevy community on [Discord](https://discord.gg/bevy) and
[GitHub](https://github.com/bevyengine/bevy), so you can keep track of what's
going on, get help, or participate in discussions.

## Common pitfall: mysterious compile errors

When changing between different versions of bevy (say, transitioning an existing
project from the released version to the git version), you might get lots of
strange unexpected build errors.

This tends to happen because cargo hasn't fully updated the whole dependency
tree. It is often fixed by removing `Cargo.lock` and the `target` directory:

```sh
rm -rf Cargo.lock target
```

If you are still getting lots of strange build errors, it is probably
because cargo is trying to use multiple different versions of bevy in your
dependency tree simultaneously. This can happen if some of the plugins you
use have specified a different bevy version from your project. This is
annoying, but easy to fix. Read the next section below for advice on how to
configure your project in a way that minimizes the chances of this happening.

## How to use bleeding-edge bevy?

The recommended way is using a cargo patch:

```toml
[dependencies]
# keep this as normal
bevy = "0.5"

[patch.crates-io]
# override it with bevy from git
bevy = { git = "https://github.com/bevyengine/bevy" }
# or if you have it cloned locally:
bevy = { path = "../bevy" }
```

Doing it this way will tell cargo to replace the version of bevy in your
entire dependency tree, including for 3rd-party plugins (assuming that they
also list the crates-io version (`bevy = "0.5"`) in their dependencies).

This works better than specifying the `git` or `path` directly in your
`[dependencies]`, because you avoid the risk of potentially having multiple
different versions of bevy in your dependency tree, if any 3rd-party plugins
you use have specified a different version.

Unfortunately, some plugin authors put the git version directly in their
`[dependencies]`, which breaks the above setup. This can be fixed by adding
another cargo patch, to also override the git repository:

```toml
[patch."https://github.com/bevyengine/bevy"]
bevy = { path = "../bevy" }
```

## Updating Bevy

The `Cargo.lock` file keeps track of the exact version (including the git
commit) you are working with. You will not be affected by new changes in
upstream bevy, until you update manually.

To update, run:
```sh
cargo update
```

If you delete or lose the `Cargo.lock` file, cargo will have to regenerate
it and update your bevy in the process. To prevent this, you should add it
to your git repository along with your source code. Alternatively, you can
be more explicit and require an exact git commit hash in your `Cargo.toml`:

```toml
bevy = { git = "https://github.com/bevyengine/bevy", rev = "7a1bd34e" }
```

## Advice for plugin authors

If you are publishing a plugin for bevy, and you want to support bevy main,
it is recommended that you:
  - Do it on a separate branch in your repository, to keep it separate from
    your main version for the released version of bevy. 
  - Put information in your README to tell people how to find it.
  - Configure your `Cargo.toml` as shown [above](#how-to-use-bleeding-edge-bevy).
  - Set up CI to notify you if your plugin is broken by new changes in bevy.

### `Cargo.toml` and dependencies

Publish your plugin with a `Cargo.toml` as shown on this page.

By specifying the released version of bevy, even in your bevy main tracking
branch, you make life easier for your users. If they want to use a local clone,
a specific commit, or their own fork (instead of the upstream repository),
they can easily do it with a simple cargo patch in their project.

If you specify the bevy git repository directly in your dependencies, you
are making such workflows more difficult.

You can safely include the cargo patch, too. It would apply when you are
working on your plugin, so that you build against the correct version of bevy,
but it would not affect your users, letting them use whatever bevy they want.

### CI Setup

Here is an example for GitHub Actions. This will run at 8:00 AM (UTC) every day
to verify that your code still compiles. GitHub will notify you when it fails.

```yaml
name: check if code still compiles (next)

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
        with:
          ref: next

      - name: Install Dependencies
        run: sudo apt-get update && sudo apt-get install --no-install-recommends pkg-config libx11-dev libasound2-dev libudev-dev

      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Check code
        run: cargo update && cargo check --lib --examples
```
