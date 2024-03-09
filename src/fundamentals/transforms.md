{{#include ../include/header013.md}}

# Transforms

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

A Transform is what allows you to place an object in the game world. It
is a combination of the object's "translation" (position/coordinates),
"rotation", and "scale" (size adjustment).

You move objects around by modifying the translation, rotate them by modifying
the rotation, and make them larger or smaller by modifying the scale.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:transform-init}}
```

## Transform Components

In Bevy, transforms are represented by **two** [components][cb::component]:
[`Transform`] and [`GlobalTransform`].

Any [Entity][cb::ecs-intro] that represents an object in the game world
needs to have both. All of Bevy's [built-in bundle types][builtins::bundle]
include them.

If you are creating a custom entity without using those [bundles][cb::bundle],
you can use one of the following to ensure you don't miss them:
 - [`SpatialBundle`] for transforms + [visibility][cb::visibility]
 - [`TransformBundle`] for just the transforms

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:spatialbundle}}
```

### `Transform`

[`Transform`] is what you typically work with. It is a `struct` containing the
translation, rotation, and scale. To read or manipulate these values, access it
from your [systems][cb::system] using a [query][cb::query].

If the entity has a [parent][cb::hierarchy], the [`Transform`] component is
relative to the parent. This means that the child object will move/rotate/scale
along with the parent.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:transform-mut}}
```

### `GlobalTransform`

[`GlobalTransform`] represents the absolute global position in the world.

If the entity does not have a [parent][cb::hierarchy], then this will match the
[`Transform`].

The value of [`GlobalTransform`] is calculated/managed internally by Bevy
(["transform propagation"](#transform-propagation)).

Unlike [`Transform`], the translation/rotation/scale are not accessible
directly. The data is stored in an optimized way (using [`Affine3A`]) and it is
possible to have complex transformations in a hierarchy that cannot be
represented as a simple transform. For example, a combination of rotation and
scale across multiple parents, resulting in shearing.

If you want to try to convert a [`GlobalTransform`] back into a workable
translation/rotation/scale representation, you can try the methods:
 - `.translation()`
 - `.to_scale_rotation_translation()` (may be invalid)
 - `.compute_transform()` (may be invalid)

## Transform Propagation

The two components are synchronized by a bevy-internal system (the "transform
propagation system"), which runs in the [`PostUpdate`] [schedule][cb::schedule].

Beware: When you mutate the [`Transform`], the [`GlobalTransform`] is not
updated immediately. They will be out-of-sync until the transform propagation
system runs.

If you need to work with [`GlobalTransform`] directly, you should [add][cb::app]
your [system][cb::system] to the [`PostUpdate`] [schedule][cb::schedule] and
[order it after][cb::system-order] [`TransformSystem::TransformPropagate`][`TransformSystem`].

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:globaltransform}}
```
```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:globaltransform-app}}
```

## `TransformHelper`

If you need to get an up-to-date [`GlobalTransform`] in a [system][cb::system]
that has to run before transform propagation, you can use the special
[`TransformHelper`] system parameter.

It allows you to compute a specific entity's [`GlobalTransform`] immediately, on
demand.

An example of where this could be useful might be a system to make a camera
follow an entity on-screen. You need to update the camera's [`Transform`] (which
means you have to do it before Bevy's transform propagation, so it can account
for the camera's new transform), but you also need to know the current
up-to-date position of the entity you are following.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:transformhelper}}
```

Internally, [`TransformHelper`] behaves like two read-only [queries][cb::query].
It needs access to the [`Parent`] and [`Transform`] components to do its job. It
would conflict with our other `&mut Transform` query. That's why we have to use
a [param set][cb::paramset] in the example above.

Note: if you over-use [`TransformHelper`], it could become a performance issue.
It calculates the global transform for you, but it does not update the data
stored in the entity's [`GlobalTransform`]. Bevy will still do the same
computation again later, during transform propagation. It leads to repetitive
work. If your system can run after transform propagation, so it can just read
the value after Bevy updates it, you should prefer to do that instead of using
[`TransformHelper`].
