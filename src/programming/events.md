{{#include ../include/header013.md}}

# Events

Relevant official examples:
[`event`][example::event].

---

Send data between systems! Let your [systems][cb::system] communicate with each other!

Like [resources][cb::res] or [components][cb::component], events are
simple Rust `struct`s or `enum`s. When creating a new event type, derive
the [`Event`][bevy::Event] trait.

Then, any [system][cb::system] can send (broadcast) values of that type,
and any system can receive those events.

 - To send events, use an [`EventWriter<T>`][bevy::EventWriter].
 - To receive events, use an [`EventReader<T>`][bevy::EventReader].

Every reader tracks the events it has read independently, so you can handle
the same events from multiple [systems][cb::system].

```rust,no_run,noplayground
{{#include ../code013/src/programming/events.rs:events}}
```

You need to register your custom event types via the [app builder][cb::app]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/events.rs:events-appbuilder}}
```

## Usage Advice

Events should be your go-to data flow tool. As events can be sent from any
[system][cb::system] and received by multiple systems, they are *extremely*
versatile.

Events can be a very useful layer of abstraction. They allow you to decouple
things, so you can separate different functionality and more easily reason
about which [system][cb::system] is responsible for what.

You can imagine how, even in the simple "player level up" example shown above,
using events would allow us to easily extend our hypothetical game with more
functionality. If we wanted to display a fancy level-up effect or animation,
update UI, or anything else, we can just add more systems that read the events
and do their respective things. If the `player_level_up` system had simply
checked the player XP and managed the player level directly, without going via
events, it would be unwieldy for future development of the game.

## How it all works

When you register an event type, Bevy will create an [`Events<T>`][bevy::Events]
[resource][cb::res], which acts as the backing storage for the event queue. Bevy
also adds an "event maintenance" [system][cb::system] to clear events periodically,
preventing them from accumulating and using up memory.

Bevy ensures that events are kept around for at least two frame update cycles,
or two [fixed timestep][cb::fixedtimestep] cycles, whichever is longer. After
that, they are silently dropped. This gives your systems enough opportunity
to handle them, assuming your systems are running all the time. Beware when
adding [run conditions][cb::rc] to your systems, as you might miss some events
when your systems are not running!

If you don't like this, [you can have manual control over when events are
cleared][cb::event-manual] (at the risk of leaking / wasting memory if you
forget to clear them).

The [`EventWriter<T>`][bevy::EventWriter] system parameter is just syntax sugar
for mutably accessing the [`Events<T>`][bevy::Events] [resource][cb::res] to
add events to the queue. The [`EventReader<T>`][bevy::EventReader] is a little
more complex: it accesses the events storage immutably, but also stores an
integer counter to keep track of how many events you have read. This is why it
also needs the `mut` keyword.

## Possible Pitfalls

Beware of frame delay / 1-frame-lag. This can occur if Bevy runs the
receiving system before the sending system. The receiving system will only
get a chance to receive the events the next time it runs. If you need to
ensure that events are handled immediately / on time, you can use [explicit
system ordering][cb::system-order].

If your systems have [run conditions][cb::rc], beware that they might miss
some events when they are not running! If your system does not check for events
at least once every other frame or [fixed timestep][cb::fixedtimestep], the
events will be lost.

If you want events to persist for longer than that, you can [implement a
custom cleanup/management strategy][cb::event-manual]. However, you can
only do this for your own event types. There is no solution for Bevy's
[built-in][builtins::event] types.
