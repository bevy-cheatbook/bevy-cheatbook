# Quitting the App

{{#include ../include/links.md}}

To cleanly shut down bevy, send an [`AppExit`][bevy::AppExit]
[event][cb::event] from any [system][cb::system]:

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:system}}
```

For prototyping, bevy provides a system you can [add to your
`App`][cb::app], to exit on pressing the `Esc` key:

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:main}}
```

