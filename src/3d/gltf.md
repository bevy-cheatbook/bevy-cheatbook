# 3D Models and Scenes (GLTF)

{{#include ../include/links.md}}

Relevant official examples:
[`load_gltf`][example::load_gltf],
[`update_gltf_scene`][example::update_gltf_scene].

---

Bevy uses the GLTF 2.0 file format for 3D assets.

(other formats such as Wavefront OBJ may be unofficially available via 3rd-party plugins)

## Quick-Start: Spawning 3D Models into your World

The simplest use case is to just load a "3D model" and spawn it into the game world.

"3D models" can often be complex, consisting of multiple parts. Think of a
house: the windows, roof, doors, etc., are separate pieces, that are likely
made of multiple meshes, materials, and textures. Bevy would technically
need multiple ECS Entities to represent and render the whole thing.

This is why your GLTF "model" is represented by Bevy as a Scene.  This way,
you can easily spawn it, and Bevy will create all the relevant [child
entities][cb::hierarchy] and configure them correctly.

So that you can treat the whole thing as "a single object" and position
it in the world, you can just [spawn it under a parent entity, and use its
`Transform`][cb::transform].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:spawn-gltf-simple}}
```

If your GLTF Scene represents "a whole level/map", rather than "an individual
3d model", and you don't need to move it around, you can just spawn the
scene directly, without creating a parent entity.

Also, this example assumes that you have a simple GLTF file containing only
one "default scene". GLTF is a very flexible file format. A single file can
contain many "models" or more complex "scenes". To get a better understanding
of GLTF and possible workflows, read the rest of this page. :)

## Introduction to GLTF

GLTF is a modern open standard for exchanging 3D assets between different
3D software applications, like game engines and 3D modeling software.

The GLTF file format has two variants: human-readable ascii/text (`*.gltf`)
and binary (`*.glb`). The binary format is more compact and preferable
for packaging the assets with your game. The text format may be useful for
development, as it can be easier to manually inspect using a text editor.

A GLTF file can contain many objects (sub-assets): meshes, materials,
textures, scenes. When loading a GLTF file, Bevy will load all of the assets
contained inside. They will be mapped to the [appropriate Bevy-internal
asset types][builtins::asset].

## The GLTF sub-assets

GLTF terminology can be confusing, as it sometimes uses the same words to
refer to different things, compared to Bevy. This section will try explain
the various GLTF terms.

To understand everything, it helps to mentally consider how these concepts are
represented in different places: in your 3D modeling software (like Blender),
in the GLTF file itself, and in Bevy.

GLTF **Scenes** are what you spawn into your game world. This is typically
what you see on the screen in your 3D modeling software. Scenes combine
all of the data needed for the game engine to create all the needed
entities to represent what you want. Conceptually, think of a scene as one
"unit". Depending on your use case, this could be one "3d model",
or even a whole map or game level. In Bevy, these are represented as Bevy
Scenes with all the child ECS entities.

GLTF Scenes are composed of GLTF **Nodes**. These describe the "objects"
in the scene, typically GLTF Meshes, but can also be other things like
Cameras and Lights. Each GLTF Node has a transform for positioning it in
the scene.  GLTF Nodes do not have a core Bevy equivalent; Bevy just uses
this data to create the ECS Entities inside of a Scene. Bevy has a special
[`GltfNode`][bevy::GltfNode] asset type, if you need access to this data.

GLTF **Meshes** represent one conceptual "3D object". These correspond
to the "objects" in your 3D modeling software. GLTF Meshes may be complex
and composed of multiple smaller pieces, called GLTF Primitives, each of
which may use a different Material. GLTF Meshes do not have a core Bevy
equivalent, but there is a special [`GltfMesh`][bevy::GltfMesh] asset type,
which describes the primitives.

GLTF **Primitives** are individual "units of 3D geometry", for the purposes of
rendering. They contain the actual geometry / vertex data, and reference the
Material to be used when drawing. In Bevy, each GLTF Primitive is represented
as a Bevy [`Mesh`][bevy::Mesh] asset, and must be spawned as a separate ECS
Entity to be rendered.

GLTF **Materials** describe the shading parameters for the surfaces of
your 3D models. They have full support for Physically-Based Rendering
(PBR). They also reference the textures to use. In Bevy, they are represented
as [`StandardMaterial`][bevy::StandardMaterial] assets, as used by the Bevy
PBR 3D renderer.

GLTF **Textures** (images) can be embedded inside the GLTF file, or stored
externally in separate image files alongside it. For example, you can have
your textures as separate PNG or JPEG files for ease of development, or
package them all inside the GLTF file for ease of distribution. In Bevy,
GLTF textures are loaded as Bevy [`Image`][bevy::Image] assets.

GLTF **Samplers** describe the settings for how the GPU should use a
given Texture. Bevy does not keep these separate; this data is stored
inside the Bevy [`Image`][bevy::Image] asset (the `sampler` field of type
[`SamplerDescriptor`][bevy::SamplerDescriptor]).

## GLTF Usage Patterns

Because GLTF is so flexible, it is up to you how to structure your assets.

A single GLTF file might be used:
  - To represent a single "3D model", containing a single
    GLTF Scene with the model, so you can spawn it into your game.
  - To represent a whole level, as a GLTF Scene, possibly also including
    the camera. This lets you load and spawn a whole level/map at once.
  - To represent a piece of a level/map, such as a room.
  - To contain a set of many different "3D models", each as a separate GLTF Scene.
    This lets you load and manage the whole collection at once and spawn them individually as needed.
  - … others?

## Tools for Creating GLTF Assets

If you are using a recent version of Blender (2.8+) for 3D modeling, GLTF
is supported out of the box. Just export and choose GLTF as the format.

For other tools, you can try these exporter plugins:
  - [Old Blender (2.79)][gltf-export-blender-old]
  - [3DSMax][gltf-export-3dsmax]
  - [Autodesk Maya][gltf-export-maya]
    - (or this [alternative][gltf-export-maya2])

## Using GLTF Sub-Assets in Bevy

The various sub-assets contained in a GLTF file can be addressed in two ways:
  - by index (integer id, in the order they appear in the file)
  - by name (text string, the names you set in your 3D modeling software
    when creating the asset, which can be exported into the GLTF)

To get handles to the respective assets in Bevy, you can use the
[`Gltf`][bevy::Gltf] ["master asset"][#gltf-master-asset], or alternatively,
[AssetPath with Labels](#assetpath-with-labels).

### `Gltf` master asset

If you have a complex GLTF file, this is likely the most flexible and useful
way of navigating its contents and using the different things inside.

You have to wait for the GLTF file to load, and then use the [`Gltf`][bevy::Gltf] asset.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:gltf-complex}}
```

For a more convoluted example, say we want to directly create a 3D PBR
entity, for whatever reason. (This is not recommended; you should probably
just use scenes)

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:gltf-manual-pbr}}
```

### AssetPath with Labels

This is another way to access specific sub-assets. It is less reliable,
but may be easier to use in some cases.

Use the [`AssetServer`][bevy::AssetServer] to convert a path string into a
[`Handle`][bevy::Handle].

The advantage is that you can get handles to your sub-assets immediately,
even if your GLTF file hasn't loaded yet.

The disadvantage is that it is more error-prone. If you specify a sub-asset
that doesn't actually exist in the file, or mis-type the label, or use the
wrong label, it will just silently not work. Also, currently only using a
numerial index is supported. You cannot address sub-assets by name.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:gltf-assetpath}}
```

{{#include ../include/builtins.md:gltf-asset-labels}}

The [`GltfNode`][bevy::GltfNode] and [`GltfMesh`][bevy::GltfMesh]
asset types are only useful to help you navigate the contents of
your GLTF file. They are not core Bevy renderer types, and not used
by Bevy in any other way. The Bevy renderer expects Entities with
[`MaterialMeshBundle`][bevy::MaterialMeshBundle]; for that you need the
[`Mesh`][bevy::Mesh] and [`StandardMaterial`][bevy::StandardMaterial].

## Bevy Limitations

{{#include ../include/gltf-limitations.md}}
