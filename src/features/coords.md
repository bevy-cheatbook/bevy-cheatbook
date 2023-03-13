Warning: this page has not been updated for Bevy 0.10 yet!

# Coordinate System

{{#include ../include/links.md}}

## 2D and 3D scenes and cameras

Bevy uses a right-handed Y-up coordinate system.

Bevy uses the same coordinate system for 3D and 2D, for consistency.

It is easiest to explain in terms of 2D:
 - The X axis goes from left to right (+X points right).
 - The Y axis goes from bottom to top (+Y points up).
 - The Z axis goes from far to near (+Z points towards you, out of the screen).
 - For 2D, the origin (X=0.0; Y=0.0) is at the *center of the screen* by default.

When you are working with 2D sprites, you can put the background on Z=0.0, and
place other sprites at increasing positive Z coordinates to layer them on top.

In 3D, the axes are oriented the same way:
 - Y points up
 - The forward direction is -Z

This is a right-handed coordinate system. You can use the fingers of your right
hand to visualize the 3 axes: thumb=X, index=Y, middle=Z.

It is the same as Godot, Maya, and OpenGL. Compared to Unity, the Z axis
is inverted.

![Chart comparing coordinate system orientation in different game engines and 3D software](/img/handedness.png)

(graphic modifed and used with permission; original by [@FreyaHolmer](https://twitter.com/FreyaHolmer))

---

Also beware of a common pitfall when working in 2D: [the camera must be
positioned at a far away Z coordinate (=999.9 by default), or you might not
be able to see your sprites!][pitfall::2d-camera-z]

## UI

For UI, Bevy follows the same convention as most other UI toolkits, the Web, etc.
 - The origin is at the top left corner of the screen
 - The Y axis points downwards
 - X goes from 0.0 (left screen edge) to the number of screen pixels (right screen edge)
 - Y goes from 0.0 (top screen edge) to the number of screen pixels (bottom screen edge)

The units represent logical (compensated for DPI scaling) screen pixels.

UI layout flows from top to bottom, similar to a web page.
