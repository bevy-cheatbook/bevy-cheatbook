#UV coordinate system is oriented top to bottom

If you have done any work with OpenGL meshes, UV coordinates are oriented with
the origin at the bottom left of the texture. 

Bevy uses `wgpu` for it's graphics pipeline and thus follows the trend of
maintaining UV origin at the top-left of the texture.

This is inline with most other graphics pipelines including DirectX, Vulkan and Metal.
