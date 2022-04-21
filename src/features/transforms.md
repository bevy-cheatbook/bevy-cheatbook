# Transforms

{{#include ../include/links.md}}

Relevant official examples:
[`parenting`][example::parenting],
anything that spawns 2D or 3D objects.

---

First, a quick definition, if you are new to game development:

a Transform is what allows you to place an object in the game world. It
is a combination of the object's "translation" (position/coordinates),
"rotation", and "scale" (size adjustment).

You move objects around by modifying the translation, rotate them by modifying
the rotation, and make them larger or smaller by modifying the scale.

## Transform Components

In Bevy, transforms are represented by *two* [components][cb::component]:
[`Transform`][bevy::Transform] and [`GlobalTransform`][bevy::GlobalTransform].
Any [Entity][cb::ecs-intro] that represents an object in the game world
needs to have both. All of Bevy's [bundle types][builtins::bundle]
include them.  If you are creating a custom entity, you can use
[`TransformBundle`][bevy::TransformBundle] to ensure you don't miss them.

[`Transform`][bevy::Transform] is what you typically work with. It is
a `struct` containing the translation, rotation, and scale. To read or
manipulate these values, access them from your [systems][cb::system] using a
[query][cb::query].

If the entity has a [parent][cb::hierarchy], the [`Transform`][bevy::Transform]
component is relative to the parent. This means that the child object will
move/rotate/scale along with the parent.

[`GlobalTransform`][bevy::GlobalTransform] represents the absolute global
position in the world. If the entity does not have a parent, then this will
have the same value as the [`Transform`][bevy::Transform]. The value of
[`GlobalTransform`][bevy::GlobalTransform] is calculated/managed internally
by Bevy. You should treat it as read-only; do not mutate it.

Beware: The two components are synchronized by a bevy-internal system
(the "transform propagation system"), which runs in the [`PostUpdate`][bevy::CoreStage]
[stage][cb::stage]. This is somewhat finnicky and can result in tricky pitfalls
if you are trying to do advanced things that rely on both the relative/local
and the absolute/global transforms of entities. When you mutate the
[`Transform`][bevy::Transform], the [`GlobalTransform`][bevy::GlobalTransform]
is not updated immediately. They will be out-of-sync until the transform
propagation system runs.
