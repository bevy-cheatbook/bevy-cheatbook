# UV coordinates in Bevy

In Bevy, the vertical axis for the pixels of textures / images, and when
sampling textures in a shader, points *downwards*, from top to bottom. The
origin is at the top left.

This is consistent with how most image file formats store pixel data, and
with how most graphics APIs work (including DirectX, Vulkan, Metal, WebGPU,
but *not* OpenGL).

This is different from OpenGL (and frameworks based on it). If your prior
experience is with these, you may find that the textures on your meshes are
flipped vertically. You will have to reexport / regenerate your meshes in
the correct UV format.

This is also inconsistent with the [World-coordinate
system used everywhere else in Bevy](../features/transforms.md), where the
Y axis points up.

If the images of your 2D sprites are flipped (for whatever reason), you can
correct that using Bevy's sprite-flipping feature:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:sprite-flipping}}
```
