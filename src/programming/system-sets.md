{{#include ../include/header013.md}}

# System Sets

System Sets allow you to easily apply common properties to multiple systems,
such as [ordering][cb::system-order] and [run conditions][cb::rc].

Anything you add to the set will automatically be applied to all systems
belonging to the set.

A system can belong to multiple different sets, and will inherit all the
properties from all of them. You can also add additional properties to
individual systems.

All of this combined gives you a lot of flexibility and control over how your systems run.

## Anonymous Sets

The simplest kind of system set is when you just [add][cb::app] a tuple of
multiple [systems][cb::system] using `.add_systems`.

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_sets.rs:anonymous}}
```

This syntax is useful when you just want to apply some common configuration to
multiple systems.

## Named Sets

This is the more formal and powerful way to use system sets.

You can create a Rust type (`struct` or `enum`) to serve as a label/identifier,
so you can refer to the set from different places.

For a single set, create an empty `struct`. If you need to create multiple
related sets, create an `enum`. Every variant of the `enum` is a separate system
set.

You need to derive [`SystemSet`] + an assortment of required standard Rust traits:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_sets.rs:definition}}
```

Now, you can apply the set to your systems using `.in_set()`:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_sets.rs:app}}
```

You can add [run conditions][cb::rc] and [ordering
dependencies][cb::system-order] on your set using `.configure_sets`:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_sets.rs:configure}}
```

The main use case of named system sets is for logical organization, so that you
can manage your systems and refer to the whole group.

Some examples:
 - A set for all your audio-related systems, so you can disable them all if sound is muted.
 - A set for all your touchscreen input systems, so you can disable them all if there is no touchscreen.
 - A set for all your input handling systems, so you can order them to run before gameplay systems.
 - A set for all your gameplay systems, so that they only run during the in-game [state][cb::state].

### With Plugins

Named sets are also very useful together with [plugins][cb::plugin]. When you are writing
a plugin, you can expose (make `pub`) some system set types, to allow users of your
plugin to control how things in your plugin run, or how their things run in relation to
your plugin. This way, you don't have to expose any of your individual systems.

Some examples:
 - You are making a physics plugin. Make a set for your whole plugin, so your users can
   easily order their systems to run before/after physics. They can also easily control
   whether your physics runs at all, by adding an extra run condition to your set.
 - You are using plugins for internal organization in your project. You have an UI plugin.
   Create a system set for the systems that need to update UI state from gameplay state,
   so that you can easily add ordering dependencies between UI and gameplay. Other plugins
   / places in your code now don't need to know about the internals of your UI plugin.

## Common Pitfalls

WARNING! System set configuration is stored *per-[schedule][cb::schedule]!* Notice how
we had to specify `.configure_sets(Update, ...)`. It can be very easy to configure your
sets once and then just assume you can use them anywhere, but that is not true.

If you try to use them in a [schedule][cb::schedule] other than the one
where you configured them, your code will compile and run (Bevy silently
initializes the sets in each schedule), but will not work correctly, as they
will not have any of your configurations.

Some common scenarios where this can occur:
 - You configure your set in [`Update`] and try to also use it in [`FixedUpdate`], or vice versa.
 - You try to use your sets in the [`OnEnter`]/[`OnExit`] schedules of various [app states][cb::state].
 - You add a system to [`PostUpdate`] or [`PreUpdate`].
