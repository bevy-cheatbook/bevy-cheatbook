{{#include ../include/header09.md}}

# Bloom

The "Bloom" effect creates a glow around bright lights. It is not a
physically-accurate effect, though it is inspired by how light looks through
a dirty or imperfect lens.

Bloom does a good job of helping the perception of very bright light,
especially when outputting HDR to the display hardware is not supported.
Your monitor can only display a certain maximum brightness, so Bloom is a
common artistic choice to try to convey light intensity brighter than can
be displayed.

Bloom required [HDR mode][cb::hdr] to be enabled on your camera.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/hdr.rs:bloom-config}}
```

Here is an example of Bloom in 3D:

![The Bloom effect on street lamps.](/img/bloom_3d.png)

And here is a 2D example:

![The Bloom effect on a simple hexagon.](/img/bloom_2d.png)
