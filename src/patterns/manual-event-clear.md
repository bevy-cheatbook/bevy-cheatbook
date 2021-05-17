# Manual Event Clearing

[Click here to download a full example file with the code from this page.](../code/examples/manual-event-clear.rs)

---

The [event](../programming/events.md) queue needs to be cleared periodically,
so that it does not grow indefinitely and waste unbounded memory.

Bevy's default cleanup strategy is to clear events every frame, but with double
buffering, so that events from the previous frame update stay available. This
means that you can handle the events only until the end of the next frame
after the one when they are sent.

This default works well for systems that run every frame and check for events
every time, which is the typical usage pattern.

However, if you have systems that do not read events every frame, they might
miss some events. Some common scenarios where this occurs are:
  - systems with an early-return, that don't read events every time they run
  - when using [fixed timestep](../features/fixed-timestep.md)
  - systems that only run in specific [states](../programming/states.md),
    such as if your game has a pause state
  - when using custom [run criteria](../programming/run-criteria.md) to control
    your systems

To be able to reliably manage events in such circumstances, you might want
to have manual control over how long the events are held in memory.

You can replace Bevy's default cleanup strategy with your own.

To do this, simply add your event type (wrapped as `Events<T>`) to the
[app builder](../programming/app-builder.md) using `.init_resource`,
instead of `.add_event`.

(`.add_event` is actually just a convenience method that
initializes the resource and adds Bevy's built-in system
([generic](./generic-systems.md) over your event type) for the default
cleanup strategy)

You must then clear the events at your discretion. If you don't do this often
enough, your events might pile up and waste memory.

```rust,no_run,noplayground
{{#include ../code/examples/manual-event-clear.rs:main}}
```
