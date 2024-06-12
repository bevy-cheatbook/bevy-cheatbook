{{#include ../include/header014.md}}

# Exclusive Systems

Exclusive systems are [systems][cb::system] that Bevy will not run in parallel
with any other system. They can have [full unrestricted access][cb::world]
to the whole ECS [`World`], by taking a [`&mut World`] parameter.

Inside of an exclusive system, you have full control over all data stored
in the ECS. You can do whatever you want.

Some example situations where exclusive systems are useful:
 - Dump various entities and components to a file, to implement things like
   saving and loading of game save files, or scene export from an editor
 - Directly spawn/despawn [entities][cb::ec], or insert/remove [resources][cb::res],
   immediately with no delay (unlike when using [Commands][cb::commands]
   from a regular system)
 - Run arbitrary [systems][cb::system] and [schedules][cb::schedule] with your
   own custom control flow logic
 - …

See the [direct World access page][cb::world] to learn more about how to do
such things.

```rust,no_run,noplayground
{{#include ../code014/src/programming/exclusive.rs:fn}}
```

You need to add exclusive systems to the [App][cb::app], just like
regular systems. All scheduling APIs ([ordering][cb::system-order], [run
conditions][cb::rc], [sets][cb::systemset]) are supported and work the same
as with regular systems.

```rust,no_run,noplayground
{{#include ../code014/src/programming/exclusive.rs:app}}
```

## Exclusive System Parameters

There are a few other things, besides [`&mut World`], that can be used as
parameters for exclusive systems:

{{#include ../include/builtins.md:systemparam-exclusive}}

[`SystemState`] can be used to emulate a normal system.  You can put regular
system parameters inside. This allows you to access the [`World`] as you would
from a normal system, but you can confine it to a specific scope inside your
function body, making it more flexible.

[`QueryState`] is the same thing, but for a single query.  It is a simpler
alternative to [`SystemState`] for when you just need to be able to query for
some data.

```rust,no_run,noplayground
{{#include ../code014/src/programming/exclusive.rs:systemstate}}
```

Note: if your [`SystemState`] includes [`Commands`], you must call `.apply()`
after you are done! That is when the deferred operations queued via
[commands][cb::commands] will be applied to the [`World`].

## Performance Considerations

Exclusive systems, by definition, limit parallelism and multi-threading, as
nothing else can access the same ECS World while they run. The whole schedule
needs to come to a stop, to accomodate the exclusive system. This can easily
introduce a performance bottleneck.

Generally speaking, you should avoid using exclusive systems, unless you need
to do something that is only possible with them.

On the other hand, if your alternative is to use [commands][cb::commands],
and you need to process a huge number of entities, exclusive systems are faster.

[`Commands`] is effectively just a way to ask Bevy do to exclusive [`World`]
access for you, at a later time. Going through the commands queue is much
slower than just doing the exclusive access yourself.

Some examples for when exclusive systems can be faster:
 - You want to spawn/despawn a ton of entities.
   - Example: Setup/cleanup for your whole game map.
 - You want to do it every frame.
   - Example: Managing hordes of enemies.

Some examples for when normal systems with [`Commands`] can be faster:
 - You need to check some stuff every frame, but only use [commands][cb::commands] sometimes.
   - Example: Despawn enemies when they reach 0 HP.
   - Example: Spawn/despawn entities when [timers][cb::timer] finish.
   - Example: Add/remove some UI elements depending on what is happening in-game.
