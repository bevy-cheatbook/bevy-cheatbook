{{#include ../include/header014.md}}

# One-Shot Systems

One-Shot Systems are [systems][cb::system] that you intend to call yourself,
whenever you want. For example: on a button press, upon triggering a special
item or ability in your game, etc…

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:example}}
```

## Registration

You should not add these systems to a [schedule][cb::schedule].

Instead, you can register them into the [`World`], to get a [`SystemId`].
You can then store that [`SystemId`] somewhere and use it to run the
system later.

The most convenient way is probably to use [`FromWorld`] and put your
[`SystemId`]s in a [resource][cb::res]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-fromworld}}
```

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-fromworld-app}}
```

Alternative: register from an [exclusive system][cb::exclusive]:

<details>
  <summary>Code:</summary>

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-exclusive}}
```

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-exclusive-app}}
```

</details>

Or from the [app builder][cb::app]:

<details>
  <summary>Code:</summary>

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-app}}
```

</details>

## Running

The easiest way is using [Commands][cb::commands] ([`Commands`]):

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:run-commands}}
```

This queues up the system to be run later, whenever Bevy decides to
apply the [Commands][cb::commands].

If you want to run a one-shot system immediately, like a normal function
call, you need [direct `World` access][cb::world]. Do it from an [exclusive
system][cb::exclusive]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:run-exclusive}}
```

Either way, the one-shot system's [Commands][cb::commands]
are automatically applied immediately when it runs.

### Without Registration

It is possible to also run one-shot systems without [registering](#registration)
them beforehand:

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:run-once}}
```

If you do this, Bevy is unable to store any data related to the system:
 - [Locals][cb::local] will not retain their value from a previous run.
 - [Queries][cb::query] will not be able to cache their lookups, leading to slower performance.
 - etc…

It is therefore recommended to register your one-shot systems, unless
you really only intend to run them once.

## Performance Considerations

To run a one-shot system, exclusive [`World`] access is required. The
system can have arbitrary parameters, and Bevy cannot validate its data
access against other systems, like it does when the system is part of a
[schedule][cb::schedule]. So, no multi-threading allowed.

In practice, this isn't usually a problem, because the use cases for
one-shot systems are things that happen rarely.

But maybe don't overuse them! If something happens regularly, consider
doing it from a normal system that is part of a [schedule][cb::schedule],
and controlling it with [run conditions][cb::rc] instead.
