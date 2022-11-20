# Events

{{#include ../include/links.md}}

Relevant official examples:
[`event`][example::event].

---

Send data between systems! Let your [systems][cb::system] communicate with each other!

 - To send events, use an [`EventWriter<T>`][bevy::EventWriter].
 - To receive events, use an [`EventReader<T>`][bevy::EventReader].

Every reader tracks the events it has read independently, so you can handle
the same events from multiple [systems][cb::system].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:events}}
```

You need to add your custom event types via the [app builder][cb::app]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:events-appbuilder}}
```

Events should be your go-to data flow tool. As events can be sent from any
system and received by multiple systems, they are *extremely* versatile.

## Possible Pitfalls

Beware of frame delay / 1-frame-lag. This can occur if Bevy runs the receiving
system before the sending system. The receiving system will only get a chance
to receive the events on the next frame update. If you need to ensure that
events are handled immediately / during the same frame, you can use [explicit
system ordering][cb::system-order].

Events don't persist. They are stored until the end of the next frame,
after which they are lost. If your systems do not handle events every frame,
you could miss some.

The advantage of this design is that you don't have to worry about excessive
memory use from unhandled events.

If you don't like this, [you can have manual control over when events are
cleared][cb::event-manual] (at the risk of leaking / wasting memory if you
forget to clear them).
