Warning: this page has not been updated for Bevy 0.10 yet!

# Exclusive Systems

{{#include ../include/links.md}}

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
{{#include ../code/src/basics.rs:exclusive-fn}}
```

You need to add exclusive systems to the [App][cb::app], just like
regular systems, but you must call `.exclusive_system()` on them.

They cannot be ordered in-between regular parallel systems. Exclusive systems
always run at one of the following places:
 - `.at_start()`: at the beginning of a [stage][cb::stage]
 - `.at_end()`: at the end of a [stage][cb::stage],
   after [commands][cb::commands] from regular systems have been applied
 - `.before_commands()`: after all the regular systems in a [stage][cb::stage],
   but before [commands][cb::commands] are applied

(if you don't specify anything, the default is assumed `.at_start()`)

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:exclusive-app}}
```
