# UV coordinates in Bevy

{{#include ../include/links.md}}

In Bevy, the vertical axis for the pixels of textures / images, and when
sampling textures in a shader, points *downwards*, from top to bottom. The
origin is at the top left.

This is inconsistent with the [World-coordinate system used everywhere else
in Bevy][cb::coords], where the Y axis points up.

It is, however, consistent with how most image file formats store pixel data,
and with how most graphics APIs work (including DirectX, Vulkan, Metal,
WebGPU, but *not* OpenGL).

OpenGL (and frameworks based on it) is different. If your prior experience
is with that, you may find that your textures appear flipped vertically.

---

If you are using a mesh, make sure it has the correct UV values. If it was
created with other software, be sure to select the correct settings.

If you are writing a custom shader, make sure your UV arithmetic is correct.

## Sprites

If the images of your 2D sprites are flipped (for whatever reason), you can
correct that using Bevy's sprite-flipping feature:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sprite-flipping}}
```
