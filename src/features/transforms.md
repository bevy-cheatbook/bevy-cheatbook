# Transforms and Coordinates

First, a quick definition, if you are new to game development:

a Transform is what allows you to place an object in the game world. It is a
combination of the object's "translation" (position/coordinates), "rotation",
and "scale" (size adjustment).

You move objects around by modifying the translation, rotate them by modifying
the rotation, and make them larger or smaller by modifying the scale.

## Bevy's Coordinate System

Bevy uses the same coordinate system for 3D, 2D, and UI, for consistency.

It is easiest to explain in terms of 2D:
 - The X axis goes from left to right (+X points right).
 - The Y axis goes from bottom to top (+Y points up).
 - The Z axis goes from far to near (+Z points towards you, out of the screen).
 - For 2D, the origin (X=0.0; Y=0.0) is at the *center of the screen* by default.
   - For UI, the origin is at the *bottom left* corner.

When you are working with 2D sprites, you can put the background on Z=0.0, and
place other sprites at increasing positive Z coordinates to layer them on top.

In 3D, the axes are oriented the same way.

This is a right-handed coordinate system.

It is the same as Godot, Maya, and OpenGL. Compared to Unity, the Z axis is inverted.

Note: In Bevy, the Y axis always points *UP*.

This may feel [unintuitive when working with UI](../pitfalls/ui-y-up.md) (as it
is the opposite from web pages), or if you are used to working with 2D libraries
where the Y axis points down.

## Bevy's Transforms

In Bevy, transforms are represented by *two* [components](../programming/ec.md):
`Transform` and `GlobalTransform`. Any [Entity](../programming/ecs-intro.md) that
represents an object in the game world needs to have both.

`Transform` is what you typically work with. It is a `struct` containing the
translation, rotation, and scale. To read or manipulate these values, access
them from your [systems](../programming/systems.md) using a [query](../programming/queries.md).

If the entity has a [parent](../programming/parent-child.md), the `Transform`
component is relative to the parent. This means that the child object will
move/rotate/scale along with the parent.

`GlobalTransform` represents the absolute global position in the world. If the
entity does not have a parent, then this will have the same value as the
`Transform`. The value of `GlobalTransform` is calculated/managed internally by
Bevy. You should treat it as read-only; do not mutate it.
