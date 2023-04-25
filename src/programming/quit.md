{{#include ../include/header09.md}}

# Quitting the App

To cleanly shut down bevy, send an [`AppExit`][bevy::AppExit]
[event][cb::event] from any [system][cb::system]:

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:system}}
```

For prototyping, bevy provides a system you can [add to your `App`][cb::app],
to close the focused window on pressing the `Esc` key. When all windows are
closed, Bevy will quit automatically.

```rust,no_run,noplayground
{{#include ../code/examples/quit.rs:main}}
```
