# Events

Send data between systems, to let your systems communicate with each other!

Accessed using an `Events<T>` resource.

Events don't persist. If receivers don't handle them every frame, they will be lost.

To receive, you need an `EventReader<T>`, to track the events received by the system. Put it in a `Local` resource.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:events}}
```

Events are very flexible data flow tool. As events can be sent from any system
and received by multiple systems, they are *extremely* versatile. Use them.
