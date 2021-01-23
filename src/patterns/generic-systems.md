# Generic Systems

[Click here to download the code from this page.](../code_bevy_release/src/generic-systems.rs)

---

Bevy systems are just plain rust functions, which means they can be generic.

This is especially useful when combined with bevy states. You can do the same thing to different sets of entities depending on state.

---

One use-case is for cleanup.

```rust,no_run,noplayground
{{#include ../code_bevy_release/examples/generic-systems.rs:cleanup}}
```

Menu entities can be tagged with `cleanup::MenuExit`, entities from the game map can be tagged with `cleanup::LevelUnload`.

We can add the generic cleanup system to our state transitions, to take care of the respective entities:

```rust,no_run,noplayground
{{#include ../code_bevy_release/examples/generic-systems.rs:main}}
```

