Bevy does not fully support all features of the GLTF format and has some
specific requirements about the data. Not all GLTF files can be loaded and
rendered in Bevy. Unfortunately, in many of these cases, you will not get
any error or diagnostic message.

Commonly-encountered limitations:

  - Textures embedded in ascii (`*.gltf`) files (base64 encoding) cannot be loaded.
    Put your textures in external files, or use the binary (`*.glb`) format.
  - Mipmaps are not supported. Your asset should still load/render,
    but without mipmapping. It may look noisy/grainy/aliased.
  - Bevy's renderer requires all meshes/primitives to have per-vertex positions,
    UVs, and normals. Make sure all of this data is included.
  - Meshes/primitives without textures (if the material is just a solid color)
    must still include UVs regardless. Bevy will not render meshes without UVs.
  - When using normal maps in your material, tangents must also be included in the mesh.
    Meshes with normal maps but without tangents are valid; other software would
    typically autogenerate the tangents if they are missing, but Bevy does not support
    this yet. Be sure to tick the checkbox for including tangents when exporting.
  - Bevy does not have built-in skeletal animation support yet. Animations are
    completely ignored.

This list is not exhaustive. There may be other unsupported scenarios that I
did not know of or forgot to include here. :)
