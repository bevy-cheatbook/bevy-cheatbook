# Strange Compilation Errors

First, make sure your Rust is up-to-date. When using Bevy, you must use at
least the latest stable version of Rust (as of the time of the Bevy release),
or nightly.

---

Sometimes, you might get strange compiler errors when building a Bevy project.
Confusing, seemingly-nonsensical errors that don't seem to be caused by
mistakes in your code. For instance, errors about Bevy's internal types (like
"expected type `Transform`, found type `Transform`").

Such errors are often caused by `cargo`'s internal state and caches being
broken. Maybe it is not resolving dependencies properly, or trying to
link multiple versions of Bevy into your project. This often occurs when
transitioning your project between the release and the git version of Bevy.

These errors can often be fixed by forcing `cargo` to regenerate its
internal state (recompute dependencies, etc.). You can do this trivially
by deleting the `Cargo.lock` file and the `target` directory.

```shell
rm -rf target Cargo.lock
```

Try building your project again after doing this. It is likely that the
mysterious errors will go away.

This trick often fixes the broken build, but if it doesn't help you,
your issue might require further investigation. Reach out to the Bevy
community via GitHub or Discord, and ask for help.

