# Quitting the App

To cleanly shut down bevy, send an `AppExit` [event](./events.md) from any
[system](./systems.md):

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:system}}
```

For prototyping, bevy provides a system you can [add to your
`App`](./app-builder.md), to exit on pressing the `Esc` key:

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:main}}
```

