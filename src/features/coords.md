# Coordinate System

{{#include ../include/links.md}}

Bevy uses a right-handed Y-up coordinate system.

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

This is a right-handed coordinate system. You can use the fingers of your right
hand to visualize the 3 axes: thumb=X, index=Y, middle=Z.

It is the same as Godot, Maya, and OpenGL. Compared to Unity, the Z axis
is inverted.

![Chart comparing coordinate system orientation in different game engines and 3D software](/img/handedness.png)

(graphic modifed and used with permission; original by [@FreyaHolmer](https://twitter.com/FreyaHolmer))

---

Note: In Bevy, the Y axis always points *UP*.

This may feel [unintuitive when working with UI][pitfall::ui-y-up]
(as it is the opposite from web pages), or if you are used to working with
2D libraries where the Y axis points down.

Also beware of a common pitfall when working in 2D: [the camera must be
positioned at a far away Z coordinate (=999.9 by default), or you might not
be able to see your sprites!][pitfall::2d-camera-z]
