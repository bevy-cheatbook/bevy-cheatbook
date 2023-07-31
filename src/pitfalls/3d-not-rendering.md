{{#include ../include/header011.md}}

# 3D objects not displaying

This page will list some common issues that you may encounter, if you are
trying to spawn a 3D object, but cannot see it on the screen.

## Missing visibility components on parent

If your entity is in a hierarchy, all its parents need to have
[visibility][cb::visibility] components. It is required even if those parent
entities are not supposed to render anything.

Fix it by inserting a [`VisibilityBundle`][bevy::VisibilityBundle]:

```rust
{{#include ../code011/src/pitfalls/d3_not_rendering.rs:insert-visibilitybundle}}
```

Or better, make sure to spawn the parent entities correctly in the first place.
You can use a [`VisibilityBundle`][bevy::VisibilityBundle] or
[`SpatialBundle`][bevy::SpatialBundle] (with [transforms][cb::transform]) if you
are not using a bundle that already includes these components.

## Too far from camera

If something is further away than a certain distance from the camera, it will be
culled (not rendered). The default value is `1000.0` units.

You can control this using the `far` field of
[`PerspectiveProjection`][bevy::PerspectiveProjection]:

```rust
{{#include ../code011/src/pitfalls/d3_not_rendering.rs:perspective-far}}
```

## Missing Vertex Attributes

Make sure your [`Mesh`][bevy::Mesh] includes all vertex attributes required
by your shader/material.

Bevy's default PBR [`StandardMaterial`][bevy::StandardMaterial]
requires *all* meshes to have:
 - Positions
 - Normals

Some others that may be required:
 - UVs (if using textures in the material)
 - Tangents (only if using normal maps, otherwise not required)

If you are generating your own mesh data, make sure to provide everything
you need.

If you are loading meshes from asset files, make sure they include everything
that is needed (check your export settings).

If you need Tangents for normal maps, it is recommended that you include them
in your GLTF files. This avoids Bevy having to autogenerate them at runtime.
Many 3D editors (like Blender) do not enable this option by default.

## Incorrect usage of Bevy GLTF assets

Refer to the [GLTF page][cb::gltf] to learn how to correctly
use GLTF with Bevy.

GLTF files are complex. They contain many sub-assets, represented by
different Bevy types. Make sure you are using the correct thing.

Make sure you are spawning a GLTF Scene, or using the correct
[`Mesh`][bevy::Mesh] and [`StandardMaterial`][bevy::StandardMaterial]
associated with the correct GLTF Primitive.

If you are using an asset path, be sure to include a label for the sub-asset you want:

```rust,no_run,noplayground
{{#include ../code011/src/pitfalls/d3_not_rendering.rs:gltf-ass-label}}
```

If you are spawning the top-level [`Gltf`][bevy::Gltf] [master asset][cb::gltf-master], it won't work.

If you are spawning a GLTF Mesh, it won't work.

## Unsupported GLTF

{{#include ../include/gltf-limitations.md}}

## Vertex Order and Culling

By default, the Bevy renderer assumes Counter-Clockwise vertex order and has
back-face culling enabled.

If you are generating your [`Mesh`][bevy::Mesh] from code, make sure your
vertices are in the correct order.

## Unoptimized / Debug builds

Maybe your asset just takes a while to load? Bevy is very slow without
compiler optimizations. It's actually possible that complex GLTF files with
big textures can take over a minute to load and show up on the screen. It
would be almost instant in optimized builds. [See here][pitfall::perf].
