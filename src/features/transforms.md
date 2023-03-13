Warning: this page has not been updated for Bevy 0.10 yet!

# Transforms

{{#include ../include/links.md}}

Relevant official examples:
[`transform`][example::transform],
[`translation`][example::translation],
[`rotation`][example::rotation],
[`3d_rotation`][example::3d_rotation],
[`scale`][example::scale],
[`move_sprite`][example::move_sprite],
[`parenting`][example::parenting],
anything that spawns 2D or 3D objects.

---

First, a quick definition, if you are new to game development:

a Transform is what allows you to place an object in the game world. It
is a combination of the object's "translation" (position/coordinates),
"rotation", and "scale" (size adjustment).

You move objects around by modifying the translation, rotate them by modifying
the rotation, and make them larger or smaller by modifying the scale.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:transform-init}}
```

## Transform Components

In Bevy, transforms are represented by **two** [components][cb::component]:
[`Transform`][bevy::Transform] and [`GlobalTransform`][bevy::GlobalTransform].

Any [Entity][cb::ecs-intro] that represents an object in the game world
needs to have both. All of Bevy's [built-in bundle types][builtins::bundle]
include them.

If you are creating a custom entity without using those [bundles][cb::bundle],
you can use one of the following to ensure you don't miss them:
 - [`SpatialBundle`][bevy::SpatialBundle] for transforms + [visibility][cb::visibility]
 - [`TransformBundle`][bevy::TransformBundle] for just the transforms

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:spatialbundle}}
```

### `Transform`

[`Transform`][bevy::Transform] is what you typically work with. It is
a `struct` containing the translation, rotation, and scale. To read or
manipulate these values, access it from your [systems][cb::system] using a
[query][cb::query].

If the entity has a [parent][cb::hierarchy], the [`Transform`][bevy::Transform]
component is relative to the parent. This means that the child object will
move/rotate/scale along with the parent.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:transform-mut}}
```

### `GlobalTransform`

[`GlobalTransform`][bevy::GlobalTransform] represents the absolute global
position in the world.

If the entity does not have a [parent][cb::hierarchy], then this will have
the same value as the [`Transform`][bevy::Transform].

The value of [`GlobalTransform`][bevy::GlobalTransform] is calculated/managed
internally by Bevy. See below.

## Transform Propagation

Beware: The two components are synchronized by a bevy-internal
system (the "transform propagation system"), which runs in the
[`PostUpdate`][bevy::CoreStage] [stage][cb::stage].

When you mutate the [`Transform`][bevy::Transform], the
[`GlobalTransform`][bevy::GlobalTransform] is not updated immediately. They
will be out-of-sync until the transform propagation system runs.

If you need to work with [`GlobalTransform`][bevy::GlobalTransform] directly,
you should add your [system][cb::system] to the [`PostUpdate`][bevy::CoreStage]
[stage][cb::stage] and [order it after][cb::system-order] the
[`TransformSystem::TransformPropagate`][bevy::TransformSystem]
[label][cb::label].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:globaltransform}}
```
