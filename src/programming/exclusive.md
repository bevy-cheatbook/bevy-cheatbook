{{#include ../include/header010.md}}

# Exclusive Systems

Exclusive systems are [systems][cb::system] that Bevy will not run in parallel
with any other system. They can have [full unrestricted access][cb::world]
to the whole ECS [`World`][bevy::World], by taking a `&mut World` parameter.

Inside of an exclusive system, you have full control over all data stored
in the ECS. You can do whatever you want.

Note that exclusive systems can limit performance, as they prevent
multi-threading (nothing else runs at the same time).

Some example situations where exclusive systems are useful:
 - Dump various entities and components to a file, to implement things like
   saving and loading of game save files, or scene export from an editor
 - Directly spawn/despawn [entities][cb::ec], or create/remove [resources][cb::res],
   immediately with no delay (unlike when using [`Commands`][cb::commands]
   from a regular system)
 - Run arbitrary systems with your own scheduling algorithm
 - …

See the [direct World access page][cb::world] to learn more about how to do
such things.

```rust,no_run,noplayground
{{#include ../code010/src/programming/exclusive.rs:exclusive-fn}}
```

You need to add exclusive systems to the [App][cb::app], just like
regular systems, and exclusive systems can be ordered [just like regular
systems][cb::system-order].

One notable, already-provided exclusive system is `apply_system_buffers`. It
performs a command flush which applies all queued-up [`Commands`][cb::commands]
added in systems since the last command flush.

```rust,no_run,noplayground
{{#include ../code010/src/programming/exclusive.rs:command-flush}}
```
