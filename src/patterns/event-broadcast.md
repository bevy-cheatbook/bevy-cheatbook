# Event Broadcast

[Click here to download the code from this page.](../code_bevy_release/src/event-broadcast.rs)

---

Bevy events can be sent by any system and received by any system. This is very flexible and allows for interesting possibilities.

You can receive the same events type from multiple systems, but only selectively handle specific events relevant to each system.

---

For example, imagine a multiplayer strategy game. You get messages from the server about game updates.

There are many different game mechanics, like unit movement, production, etc., and you can get updates about all of them.

It is convenient to have a single event type, so that you don't have to create boilerplate for new event types for every game mechanic.

```rust,no_run,noplayground
{{#include ../code_bevy_release/examples/event-broadcast.rs:event_enum}}
```

It also makes your networking system simple -- you can just send events of that type, when you get updates from the server:

```rust,no_run,noplayground
{{#include ../code_bevy_release/examples/event-broadcast.rs:net_system}}
```

Now, you can implement different game mechanics in separate systems, as they each likely need access to different things (resources/queries).

Each of those systems can receive the game events and handle only the ones it is interested in:

```rust,no_run,noplayground
{{#include ../code_bevy_release/examples/event-broadcast.rs:game_systems}}
```
