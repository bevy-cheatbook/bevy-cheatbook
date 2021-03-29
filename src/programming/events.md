# Events

Send data between systems! Let your systems communicate with each other!

To send events, use an `EventWriter<T>`. To receive events, use an `EventReader<T>`.

Every reader tracks the events it has read independently, so you can handle the
same events from multiple systems.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:events}}
```

Events should be your go-to data flow tool. As events can be sent from any
system and received by multiple systems, they are *extremely* versatile.

## Possible Pitfalls

Events don't persist. They are stored until the end of the next frame, after
which they are lost. If your systems do not handle events every frame, you could
miss some.

The advantage of this design is that you don't have to worry about excessive
memory use from unhandled events.

Also beware of [frame delay / 1-frame-lag](../pitfalls/frame-delay.md). This can
occur if Bevy runs the receiving system before the sending system. The receiving
system will get the events the next time it runs, typically on the next frame update.

If you need to ensure that events are handled immediately / during the same frame,
you can use [explicit system ordering](./system-order.md).
