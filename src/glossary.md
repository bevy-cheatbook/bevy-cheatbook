# Quick Reference: Alphabetical Index / Glossary

This page lists many common terms and concepts that you may come across while working
with Bevy. It is in alphabetical order. Every entry has a brief definition, and links
to relevant places in this book, where it is explained, if any.

It contains both bevy-specific jargon (including advanced topics that might
not yet be covered by this book), as well as general game development or
computer graphics topics that are relevant to working with Bevy.

---

## Bevy Programming Jargon

Terms and concepts related to Bevy-specific features and APIs.

|Topic                                                                 | Definition                                                                                                                            |
|----------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
|[App / App Builder](./programming/app-builder.md)                     | Bevy entry point; setup all the things to run.                                                                                        |
|[Assets](./features/assets.md)                                        | The data used by your game. Typically loaded from external files, or generated procedurally using code.                               |
|Asset Loader                                                          | Implementation for loading a specific asset type from a specific file format.                                                         |
|[`AssetServer`](./features/assets.md#loading-using-assetserver)       | Bevy API for accessing and loading assets from external data files.                                                                   |
|[Bundles](./programming/ec.md#component-bundles)                      | A "template" for easily spawning entities with a common set of component types.                                                       |
|[Change Detection](./programming/change-detection.md)                 | Write logic that responds to data being changed.                                                                                      |
|[Commands](./programming/commands.md)                                 | Spawn/despawn entities, manage components and resources. Deferred until they can be safely applied.                                   |
|[Components](./programming/ec.md)                                     | Per-entity data. Basic data primitive in Bevy.                                                                                        |
|[Entities](./programming/ec.md)                                       | ID for a set of component values. Often conceptually represents an "object".                                                          |
|[Events](./programming/events.md)                                     | Communicate between systems. Send/receive data.                                                                                       |
|[Exclusive Systems](./programming/world-exclusive.md)                 | Systems that have full access to the whole ECS world and do not run in parallel with any other systems.                               |
|[Fixed Timestep](./features/fixed-timestep.md)                        | Run some logic at a fixed time interval. Important for physics and simulations.                                                       |
|[Handles](./features/assets.md#handles)                               | ID for referring to an Asset in your game code. Reference counted (Bevy automatically unloads assets when there are no more handles). |
|[Hot Reloading](./features/assets.md#hot-reloading)                   | Automatically reloading assets from files if they are changed while the game is running.                                              |
|[Labels](./programming/labels.md)                                     | Names for systems, stages, and other things.                                                                                          |
|[Local Resources](./programming/local.md)                             | Per-system data.                                                                                                                      |
|[Non-Send](./programming/non-send.md)                                 | Data that cannot be used in a multithreaded context, must be confined to a single thread.                                             |
|[Parent/Child Hierarchy](./programming/parent-child.md)               | Entities in a hierarchical relationship.                                                                                              |
|[Plugins](./programming/plugins.md)                                   | Build the App in a modular way.                                                                                                       |
|[Queries](./programming/queries.md)                                   | Find matching entities and access their component data.                                                                               |
|[Query Filters](./programming/queries.md#query-filters)               | Criteria for narrowing down the entities to be accessed by a query.                                                                   |
|[Query Sets](./programming/query-sets.md)                             | Resolve query conflicts.                                                                                                              |
|[Removal Detection](./programming/removal-detection.md)               | Write logic that responds to data being removed from the ECS.                                                                         |
|[Resources](./programming/res.md)                                     | Global data for the whole app.                                                                                                        |
|[Run Criteria](./programming/run-criteria.md)                         | Low-level primitive for controlling if systems run.                                                                                   |
|[Scenes](./features/scenes.md)                                        | Collection of preconfigured entities that you can spawn into the world. Similar to "prefabs" in other game engines.                   |
|[Schedule (systems)](./programming/system-order.md)                   | All the systems that Bevy runs every frame update, optimized to run on the task pool with maximum parallelism and performance.        |
|[Stages](./programming/stages.md)                                     | Hard synchronization points for runtime scheduling.                                                                                   |
|[States](./programming/states.md)                                     | Control which systems run, by defining different "modes" your app can be in.                                                          |
|[Systems](./programming/systems.md)                                   | The functions that contain your game logic.                                                                                           |
|[`SystemParam`s](./pitfalls/into-system.html#supported-types)         | The acceptable Rust types to be used as function parameters in Bevy systems.                                                          |
|[System Chaining](./programming/system-chaining.md)                   | Combine multiple Rust functions into one big system.                                                                                  |
|[System Order](./programming/system-order.md)                         | Control the runtime order of execution of systems.                                                                                    |
|[System Sets](./programming/system-sets.md)                           | Group multiple systems together to apply common properties (labels, ordering, states, run criteria).                                  |
|[Texture](./features/textures.md)                                     | Asset type, typically an image (but more generally any data) that can be sampled by the GPU during rendering (in a shader).           |
|[Transforms](./features/transforms.md)                                | Position and orientation of an object. May be relative to a parent object.                                                            |
|[Untyped Handles](./features/assets.md#untyped-handles)               | Asset Handle that can refer to an asset regardless of the asset type.                                                                 |
|[Weak Handles](./features/assets.md#weak-handles)                     | Asset Handles that are not reference-counted (do not prevent the asset from being unloaded).                                          |
|[World](./programming/world-exclusive.md)                             | The underlying data structure / storage of the ECS. Contains all component and resource data.                                         |

## Ecosystem Jargon

Terms and concepts related to the community and ecosystem around bevy and the
development of the project, such as libraries and technologies that bevy uses.

|Topic                                                                 | Definition                                                                                                                            |
|----------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
|[Assets; Bevy Assets](https://bevyengine.org/assets)                  | Page on the Bevy Website listing community-made plugins and content (not to be confused with Assets, as in your game's data).         |
|Data-driven                                                           | Programming style / paradigm, where functionality is thought of in terms of the data it works with. Bevy is based on this philosophy. |
|[ECS](./programming/ecs-intro.md)                                     | The data storage / programming paradigm Bevy uses. Conceptually like a database. Bevy is often compared to other ECS implementations. |                                                     |
|[Features (cargo)](./setup/bevy-config.md#bevy-cargo-features)        | A way to configure what optional functionality is included in your build.                                                             |
|Flexbox                                                               | The layout model used in Bevy UI (originates from CSS in web browsers).                                                               |
|[GILRS](./features/input-handling.md#controller--gamepad--joystick)   | The library that Bevy uses for controller/joystick support.                                                                           |
|[GLTF](./features/gltf.md)                                            | File format for 3D assets; can contain meshes, textures, materials, scenes.                                                           |
|[Kira](./features/audio.md)                                           | 3rd-party audio library often used with Bevy (`bevy_kira_audio` plugin), much more feature-rich than Bevy's own audio.                |
|[main (bevy main)](./setup/bevy-git.md)                               | Development version of Bevy; git branch containing the latest unreleased changes.                                                     |
|Metal                                                                 | Low-level system API for accessing GPU hardware on Apple systems (macOS/iOS).                                                         |
|OpenGL                                                                | Legacy GPU API for systems that do not support Vulkan. Not yet supported by Bevy.                                                     |
|[Plugins (3rd-party crates)](./setup/unofficial-plugins.md)           | Libraries made by the community that can be added to your Bevy App.                                                                   |
|Vulkan                                                                | Low-level system API for accessing GPU hardware. The interface to the graphics driver. Available on most platforms except Apple.      |
|[WebAssembly (WASM)](./platforms/wasm.md)                             | New technology that allows running software like Bevy inside of a web browser.                                                        |
|WebGPU (WGPU)                                                         | The cross-platform GPU API that Bevy uses. Allows using modern GPU features safely across different platforms (desktop/mobile/web).   |
|[Winit](./features/windowing.md)                                      | The library that Bevy uses for windowing (managing the OS windows to display in)                                                      |

## Game Development Jargon

General game development concepts that are applicable to working with Bevy.

|Topic                                                                 | Definition                                                                                                                            |
|----------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
|[Assets](./features/assets.md)                                        | The data used by your game. Typically loaded from external files, or generated procedurally using code.                               |
|[Camera](./features/camera.md)                                        | Entity representing a "view" into the game world, to be rendered.                                                                     |
|[Coordinate System](./features/transforms.md#bevys-coordinate-system) | Orientation of the X/Y/Z axes, defining the 3D space.                                                                                 |
|[Fixed Timestep](./features/fixed-timestep.md)                        | Run some logic at a fixed time interval. Important for physics and simulations.                                                       |
|[Material](./features/materials.md)                                   | The shading properties for a surface to be drawn by the GPU, such as its color/texture, how shiny it is, ...                          |
|[Mesh](./features/meshes.md)                                          | 3D geometry data, consists of vertices, typically arranged into many triangles. Required for the GPU to draw an object.               |
|[Parent/Child Hierarchy](./programming/parent-child.md)               | Entities in a hierarchical relationship.                                                                                              |
|Procedural Generation                                                 | Creating game assets using code/math/algorithms (often done at runtime, instead of loading asset files).                              |
|Raycast                                                               | Calculating a simulated "ray" in the game world. Used, for example, for checking what object is under the mouse cursor.               |
|[Scenes](./features/scenes.md)                                        | Collection of preconfigured entities that you can spawn into the world. Similar to "prefabs" in other game engines.                   |
|Shaders                                                               | Code that runs on the GPU. Typically for rendering graphics, but also for general computations.                                       |
|[Sprites](./features/sprites.md)                                      | Game object that is a 2D rectangle displaying an image. Most 2D games are made from these.                                            |
|[Texture](./features/textures.md)                                     | Typically an image (but more generally any data) that can be sampled by the GPU during rendering (in a shader).                       |
|[Transforms](./features/transforms.md)                                | Position and orientation of an object. May be relative to a parent object.                                                            |
|UI                                                                    | User Interfaces like menus and in-game HUDs, typically displayed as overlays on top.                                                  |

## Rendering Jargon

Concepts that come up when working with computer graphics and the GPU, as applicable to Bevy.

|Topic                                                                 | Definition                                                                                                                            |
|----------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
|Anisotropic Filtering                                                 | Method of sampling a texture that accounts for the angle between the surface and the camera, resulting in better visual quality.      |
|Anti-Aliasing                                                         | Techniques to reduce aliasing artifacts (shimmering or jagged edges, when there is too much detail for the display resolution).       |
|Batching                                                              | Combining compatible data from many meshes/entities together, so that it can be drawn by the GPU all at once.                         |
|[Camera](./features/camera.md)                                        | Entity representing a "view" into the game world, to be rendered.                                                                     |
|Compute Shaders / GPU Compute                                         | A way to use the GPU for general data processing.                                                                                     |
|[Coordinate System](./features/transforms.md#bevys-coordinate-system) | Orientation of the X/Y/Z axes, defining the 3D space.                                                                                 |
|Culling                                                               | Figuring out which parts of the scene don't need to be drawn and skipping them to improve performance.                                |
|Directional Light                                                     | Global light source that illuminates the whole world, at a specified angle/direction. Typically models the sun in outdoor scenes.     |
|Draw Call                                                             | Telling the GPU to render something. Expensive; for best performance, aim to draw everything you need with the fewest draw calls.     |
|Frustum                                                               | The space/volume that is visible from a camera. Everything that the camera can see.                                                   |
|[Indices / Index Buffer](./features/meshes.md)                        | A way to make mesh data more compact (to save memory) by deduplicating vertices.                                                      |
|Instancing                                                            | A way to tell the GPU to draw many copies of the same mesh efficiently. Useful for things like vegetation (lots of grass / trees).    |
|[Material](./features/materials.md)                                   | The shading properties for a surface to be drawn by the GPU, such as its color/texture, how shiny it is, ...                          |
|[Mesh](./features/meshes.md)                                          | 3D geometry data, consists of vertices, typically arranged into many triangles. Required for the GPU to draw an object.               |
|Mipmaps                                                               | Textures with data available in many sizes (like 64x64, 32x32, 16x16, 8x8, ...). The GPU automatically uses the most appropriate.     |
|[MSAA](./cookbook/msaa.md)                                            | Multi-Sample Anti-Aliasing: hardware AA method: for pixels along edges of meshes, the GPU will render multiple samples.               |
|Normals / Normal Vectors                                              | The direction perpendicular to a surface. Useful for shading, to compute how light interacts with the surface.                        |
|Normal Maps                                                           | A way to add extra detail to a 3D object, using a texture that changes the normals to affect how light interacts with it.             |
|[Physically-Based Rendering](./features/pbr.md)                       | Modern realistic 3D graphics technique that uses materials that model physical properties, like how rough or metallic a surface is.   |
|Point Light                                                           | A light source that radiates in all directions from a given position, like a lamp or candle.                                          |
|Post-Processing                                                       | After rendering (but before displaying), apply some visual effects to the whole image.                                                |
|Render Graph                                                          | Bevy's abstraction for modular configurable rendering. Connects "nodes" together to enable various graphical features.                |
|Render Pipeline                                                       | The procedure to be executed by the GPU to draw something. Includes the shaders to run, as well as setting fixed hardware parameters. |
|Sampler / Sampling (textures)                                         | How the GPU picks a value (like a color) from a specific position on a texture.                                                       |
|Shaders                                                               | Code that runs on the GPU. Typically for rendering graphics, but also for general computations.                                       |
|[Texture](./features/textures.md)                                     | Typically an image (but more generally any data) that can be sampled by the GPU during rendering (in a shader).                       |
|Texel                                                                 | A pixel that is part of a texture.                                                                                                    |
|[Transforms](./features/transforms.md)                                | Position and orientation of an object. May be relative to a parent object.                                                            |
|Uniforms                                                              | Shader input data that is "global" for the whole mesh (unlike Vertex Attributes, which are per-vertex).                               |
|UVs / UV coordinates                                                  | Texture coordinates, used to map a texture to a mesh. The position on the texture that each vertex corresponds to.                    |
|[Vertex / Vertices](./features/meshes.md)                             | Building block of Meshes, typically a point in 3D space, with associated vertex attributes.                                           |
|[Vertex Attributes](./features/meshes.md)                             | Data associated with each vertex in a mesh, such as its position, color, UVs, normals, etc.                                           |
