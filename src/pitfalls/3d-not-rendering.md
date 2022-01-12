# 3D objects not displaying

This page will list some common issues that you may encounter, if you are
trying to spawn a 3D object, but cannot see it on the screen.

## Incorrect usage of Bevy GLTF assets

Refer to the [GLTF page](../features/gltf.md) to learn how to correctly
use GLTF with Bevy.

GLTF files are complex. They contain many sub-assets, represented by
different Bevy types. Make sure you are using the correct thing.

Make sure you are spawning a GLTF Scene, or using the correct `Mesh` and
`StandardMaterial` associated with the correct GLTF Primitive.

If you are using an asset path, be sure to include a label for the sub-asset you want:

```rust,no_run,noplayground
asset_server.load("my.gltf#Scene0");
```

If you are spawning the top-level `Gltf` master asset, it won't work.

If you are spawning a GLTF Mesh, it won't work.

If you are using the top-level `Gltf` master asset or a `GltfMesh` on a Bevy
`PbrBundle` entity that you have created yourself, it won't work. You need
a specific GLTF Primitive. Or just use Scenes. :)

## Unsupported GLTF

Bevy's GLTF support is still limited in features. See the [Limitations
section in the GLTF page](../features/gltf.md#bevy-limitations) for some
considerations.

## Unoptimized / Debug builds

Maybe your asset just takes a while to load? Bevy (and Rust in general)
is very slow without compiler optimizations. It's actually possible that
complex GLTF files with big textures can take over a minute to load and
show up on the screen. It would be almost instant in optimized builds. [See
here](./performance.md).

## Vertex Order and Culling

By default, the Bevy renderer assumes Counter-Clockwise vertex order and has
back-face culling enabled.

If you are generating your [Mesh](../features/meshes.md) from code, make sure
your vertices are in the correct order.
