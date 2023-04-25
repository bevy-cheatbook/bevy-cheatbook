{{#include ../include/header09.md}}

# Generic Systems

Bevy [systems][cb::system] are just plain rust functions, which means they
can be generic. You can add the same system multiple times, parametrized to
work on different Rust types or values.

## Generic over Component types

You can use the generic type parameter to specify what
[component][cb::component] types (and hence what [entities][cb::ecs-intro])
your [system][cb::system] should operate on.

This can be useful when combined with Bevy [states][cb::state].
You can do the same thing to different sets of entities depending on state.

### Example: Cleanup

One straightforward use-case is for cleanup. We can make a generic cleanup
system that just despawns all entities that have a certain component
type. Then, trivially run it on exiting different states.

```rust,no_run,noplayground
{{#include ../code/examples/generic-systems.rs:cleanup}}
```

Menu entities can be tagged with `cleanup::MenuExit`, entities from the game
map can be tagged with `cleanup::LevelUnload`.

We can add the generic cleanup system to our state transitions, to take care
of the respective entities:

```rust,no_run,noplayground
{{#include ../code/examples/generic-systems.rs:main}}
```

## Using Traits

You can use this in combination with Traits, for when you need some sort of
varying implementation/functionality for each type.

### Example: Bevy's Camera Projections

(this is a use-case within Bevy itself)

Bevy has a [`CameraProjection`][bevy::CameraProjection] trait. Different
projection types like [`PerspectiveProjection`][bevy::PerspectiveProjection]
and [`OrthographicProjection`][bevy::OrthographicProjection] implement that
trait, providing the correct logic for how to respond to resizing the window,
calculating the projection matrix, etc.

There is a generic system `fn camera_system::<T: CameraProjection +
Component>`, which handles all the cameras with a given projection type. It
will call the trait methods when appropriate (like on window resize events).

The [Bevy Cookbook Custom Camera Projection
Example][cb::camera-custom-projection] shows this API in action.

## Using Const Generics

Now that Rust has support for Const Generics, functions can also be
parametrized by values, not just types.

```rust,no_run,noplayground
{{#include ../code/examples/generic-systems.rs:const}}
```

Note that these values are static / constant at compile-time. This can be
a severe limitation. In some cases, when you might suspect that you could
use const generics, you might realize that you actually want a runtime value.

If you need to "configure" your system by passing in some data, you could,
instead, use a [Resource][cb::res] or [Local][cb::local].

Note: As of Rust 1.65, support for using `enum` values as const generics is
not yet stable. To use `enum`s, you need Rust Nightly, and to enable the
experimental/unstable feature (put this at the top of your `main.rs` or
`lib.rs`):

```rust,no_run,noplayground
#![feature(adt_const_params)]
```
