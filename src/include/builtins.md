[]:#(ANCHOR: bundles)

[(List in API Docs)][bevy::impl::Bundle]

Any tuples of up to 15 [`Component`][bevy::Component] types are valid bundles.

Bevy 3D:
 - [`PerspectiveCameraBundle`][bevy::PerspectiveCameraBundle]:
   3D camera with a perspective projection
 - [`OrthographicCameraBundle`][bevy::OrthographicCameraBundle]:
   Camera with an orthographic projection, 2D or 3D
 - [`MaterialMeshBundle`][bevy::MaterialMeshBundle]:
   3D Object/Primitive: a Mesh and the Material to draw it with
 - [`PbrBundle`][bevy::PbrBundle]:
   3D object with the standard Physically-Based Material (`MaterialMeshBundle<StandardMaterial>`)
 - [`DirectionalLightBundle`][bevy::DirectionalLightBundle]: 
   3D directional light (like the sun)
 - [`PointLightBundle`][bevy::PointLightBundle]: 
   3D point light (like a lamp or candle)

Bevy 2D:
 - [`OrthographicCameraBundle`][bevy::OrthographicCameraBundle]:
   Camera with an orthographic projection, 2D or 3D
 - [`SpriteBundle`][bevy::SpriteBundle]: 
   2D sprite, using a whole image ([`Image`][bevy::Image] asset)
 - [`SpriteSheetBundle`][bevy::SpriteSheetBundle]:
   2D sprite, using a sub-rectangle in a larger image ([`TextureAtlas`][bevy::TextureAtlas] asset)
 - [`MaterialMesh2dBundle`][bevy::MaterialMesh2dBundle]:
   2D shape, with custom Mesh and Material (similar to 3D objects)
 - [`Text2dBundle`][bevy::Text2dBundle]:
   Text to be drawn in the 2D world (not the UI)

Bevy UI:
 - [`UiCameraBundle`][bevy::UiCameraBundle]:
   The UI Camera
 - [`NodeBundle`][bevy::NodeBundle]:
   Empty node element (like HTML `<div>`)
 - [`ButtonBundle`][bevy::ButtonBundle]:
   Button element
 - [`ImageBundle`][bevy::ImageBundle]:
   Image element
 - [`TextBundle`][bevy::TextBundle]:
   Text element
[]:#(ANCHOR_END: bundles)

[]:#(ANCHOR: assets)
 - [`Image`][bevy::Image]:
   Pixel data, used as a texture for 2D and 3D rendering;
   also contains the [`SamplerDescriptor`][bevy::SamplerDescriptor] for texture filtering settings
 - [`TextureAtlas`][bevy::TextureAtlas]:
   2D "Sprite Sheet" defining sub-images within a single larger image
 - [`Mesh`][bevy::Mesh]:
   3D Mesh (geometry data), contains vertex attributes (like position, UVs, normals)
 - [`Shader`][bevy::Shader]:
   Raw GPU shader source code, in one of the supported languages (WGSL or SPIR-V)
 - [`ColorMaterial`][bevy::ColorMaterial]:
   Basic "2D material": contains color, optionally an image
 - [`StandardMaterial`][bevy::StandardMaterial]:
   "3D material" with support for Physically-Based Rendering
 - [`Font`][bevy::Font]:
   Font data used for text rendering
 - [`Scene`][bevy::Scene]:
   Scene composed of literal ECS entities to instantiate
 - [`DynamicScene`][bevy::DynamicScene]:
   Scene composed with dynamic typing and reflection
 - [`Gltf`][bevy::Gltf]:
   [GLTF Master Asset][cb::gltf-master]: index of the entire contents of a GLTF file
 - [`GltfNode`][bevy::GltfNode]:
   Logical GLTF object in a scene
 - [`GltfMesh`][bevy::GltfMesh]:
   Logical GLTF 3D model, consisting of multiple `GltfPrimitive`s
 - [`GltfPrimitive`][bevy::GltfPrimitive]:
   Single unit to be rendered, contains the Mesh and Material to use
 - [`AudioSource`][bevy::AudioSource]:
   Raw audio data for `bevy_audio`
 - [`FontAtlasSet`][bevy::FontAtlasSet]:
   (internal use for text rendering)
[]:#(ANCHOR_END: assets)

[]:#(ANCHOR: file-formats)
Image formats (loaded as [`Image`][bevy::Image] assets):

|Format|Cargo feature|Default?|Filename extensions|
|------|-------------|--------|-------------------|
|PNG   |`"png"`      |Yes     |`.png`             |
|HDR   |`"hdr"`      |Yes     |`.hdr`             |
|JPEG  |`"jpeg"`     |No      |`.jpg`, `.jpeg`    |
|TGA   |`"tga"`      |No      |`.tga`             |
|BMP   |`"bmp"`      |No      |`.bmp`             |
|DDS   |`"dds"`      |No      |`.dds`             |

Audio formats (loaded as [`AudioSource`][bevy::AudioSource] assets):

|Format    |Cargo feature|Default?|Filename extensions|
|----------|-------------|--------|-------------------|
|OGG Vorbis|`"vorbis"`   |Yes     |`.ogg`             |
|FLAC      |`"flac"`     |No      |`.flac`            |
|WAV       |`"wav"`      |No      |`.wav`             |
|MP3       |`"mp3"`      |No      |`.mp3`             |

3D asset (model or scene) formats:

|Format|Cargo feature|Default?|Filename extensions|
|------|-------------|--------|-------------------|
|GLTF  |`"bevy_gltf"`|Yes     |`.gltf`, `.glb`    |

[]:#(ANCHOR_END: file-formats)

[]:#(ANCHOR: resources-config-init)
 - [`LogSettings`][bevy::LogSettings]:
   Configure what messages get logged to the console
 - [`WindowDescriptor`][bevy::WindowDescriptor]:
   Settings for the primary application window
 - [`WgpuOptions`][bevy::WgpuOptions]:
   Low-level settings for the GPU API and backends
 - [`AssetServerSettings`][bevy::AssetServerSettings]:
   Advanced configuration of the [`AssetServer`][bevy::AssetServer]
[]:#(ANCHOR_END: resources-config-init)

[]:#(ANCHOR: resources-config)
 - [`ClearColor`][bevy::ClearColor]:
   Global renderer background color to clear the window at the start of each frame
 - [`AmbientLight`][bevy::AmbientLight]:
   Global renderer "fake lighting", so that shadows don't look too dark / black
 - [`Msaa`][bevy::Msaa]:
   Global renderer setting for Multi-Sample Anti-Aliasing (some platforms might only support the values 1 and 4)
 - [`GamepadSettings`][bevy::GamepadSettings]:
   Gamepad input device settings, like joystick deadzones and button sensitivities
[]:#(ANCHOR_END: resources-config)

[]:#(ANCHOR: resources-engine)
 - [`FixedTimesteps`][bevy::FixedTimesteps]:
   The state of all registered [`FixedTimestep`][bevy::FixedTimestep] drivers
 - [`Time`][bevy::Time]:
   Global time-related information (current frame delta time, time since startup, etc.)
 - [`AssetServer`][bevy::AssetServer]:
   Control the asset system: Load assets, check load status, etc.
 - [`Gamepads`][bevy::Gamepads]:
   List of IDs for all currently-detected (connected) gamepad devices
 - [`Windows`][bevy::Windows]:
   All the open windows (the primary window + any additional windows in a multi-window gui app)
 - [`WinitWindows`][bevy::WinitWindows]:
   Raw state of the `winit` backend for each window
 - [`Audio`][bevy::Audio]:
   Use this to play sounds via `bevy_audio`
 - [`AsyncComputeTaskPool`][bevy::AsyncComputeTaskPool]:
   Task pool for running background CPU tasks
 - [`ComputeTaskPool`][bevy::ComputeTaskPool]:
   Task pool where the main app schedule (all the systems) runs
 - [`IoTaskPool`][bevy::IoTaskPool]:
   Task pool where background i/o tasks run (like asset loading)
 - [`Diagnostics`][bevy::Diagnostics]:
   Diagnostic data collected by the engine (like frame times)
 - [`SceneSpawner`][bevy::SceneSpawner]:
   Direct control over spawning Scenes into the main app World
[]:#(ANCHOR_END: resources-engine)

[]:#(ANCHOR: resources-input)
 - [`Input<KeyCode>`][bevy::KeyCode]:
   Keyboard key state, as a binary [Input][bevy::Input] value
 - [`Input<MouseButton>`][bevy::MouseButton]:
   Mouse button state, as a binary [Input][bevy::Input] value
 - [`Input<GamepadButton>`][bevy::GamepadButton]:
   Gamepad buttons, as a binary [Input][bevy::Input] value
 - [`Axis<GamepadAxis>`][bevy::GamepadAxis]:
   Analog [Axis][bevy::Axis] gamepad inputs (joysticks and triggers)
 - [`Axis<GamepadButton>`][bevy::GamepadButton]:
   Gamepad buttons, represented as an analog [Axis][bevy::Axis] value
 - [`Touches`][bevy::Touches]:
   The state of all fingers currently touching the touchscreen
[]:#(ANCHOR_END: resources-input)

[]:#(ANCHOR: events-input)
 - [`MouseButtonInput`][bevy::MouseButtonInput]:
   Changes in the state of mouse buttons
 - [`MouseWheel`][bevy::MouseWheel]:
   Scrolling by a number of pixels or lines ([`MouseScrollUnit`][bevy::MouseScrollUnit])
 - [`MouseMotion`][bevy::MouseMotion]:
   Relative movement of the mouse (pixels from previous frame), regardless of the OS pointer/cursor
 - [`CursorMoved`][bevy::CursorMoved]:
   New position of the OS mouse pointer/cursor
 - [`KeyboardInput`][bevy::KeyboardInput]:
   Changes in the state of keyboard keys (keypresses, not text)
 - [`ReceivedCharacter`][bevy::ReceivedCharacter]:
   Unicode text input from the OS (correct handling of the user's language and layout)
 - [`TouchInput`][bevy::TouchInput]:
   Change in the state of a finger touching the touchscreen
 - [`GamepadEvent`][bevy::GamepadEvent]:
   Changes in the state of a gamepad or any of its buttons or axes
 - [`GamepadEventRaw`][bevy::GamepadEventRaw]:
   Gamepad events unaffected by [`GamepadSettings`][bevy::GamepadSettings]
[]:#(ANCHOR_END: events-input)

[]:#(ANCHOR: events-system)
 - [`AppExit`][bevy::AppExit]:
   Tell Bevy to shut down
 - [`CloseWindow`][bevy::CloseWindow]:
   Tell Bevy to close a window
 - [`CreateWindow`][bevy::CreateWindow]:
   Tell Bevy to open a new window
 - [`FileDragAndDrop`][bevy::FileDragAndDrop]:
   The user drag-and-dropped a file into our app
 - [`CursorEntered`][bevy::CursorEntered]:
   OS mouse pointer/cursor entered one of our windows
 - [`CursorLeft`][bevy::CursorLeft]:
   OS mouse pointer/cursor exited one of our windows
 - [`WindowCloseRequested`][bevy::WindowCloseRequested]:
   OS wants to close one of our windows
 - [`WindowCreated`][bevy::WindowCreated]:
   New application window opened
 - [`WindowFocused`][bevy::WindowFocused]:
   One of our windows is now focused
 - [`WindowMoved`][bevy::WindowMoved]:
   OS/user moved one of our windows
 - [`WindowResized`][bevy::WindowResized]:
   OS/user resized one of our windows
 - [`WindowScaleFactorChanged`][bevy::WindowScaleFactorChanged]:
   One of our windows has changed its DPI scaling factor
 - [`WindowBackendScaleFactorChanged`][bevy::WindowBackendScaleFactorChanged]:
   OS reports change in DPI scaling factor for a window
[]:#(ANCHOR_END: events-system)

[]:#(ANCHOR: stages)
Internally, Bevy has at least these built-in [stages][cb::stage]:
 - In the [main app][cb::app] ([`CoreStage`][bevy::CoreStage]):
`First`, `PreUpdate`, `Update`, `PostUpdate`, `Last`
 - In the render [sub-app][cb::subapp] ([`RenderStage`][bevy::RenderStage]):
`Extract`, `Prepare`, `Queue`, `PhaseSort`, `Render`, `Cleanup`
[]:#(ANCHOR_END: stages)

[]:#(ANCHOR: systemparams)

[(List in API Docs)][bevy::impl::SystemParam]

 - [`Commands`][bevy::Commands]:
   Manipulate the ECS using [commands][cb::commands]
 - [`Res<T>`][bevy::Res]:
   Shared access to a [resource][cb::res]
 - [`ResMut<T>`][bevy::ResMut]:
   Exclusive (mutable) access to a [resource][cb::res]
 - `Option<Res<T>>`:
   Shared access to a resource that may not exist
 - `Option<ResMut<T>>`:
   Exclusive (mutable) access to a resource that may not exist
 - [`Query<T, F = ()>`][bevy::Query] (can contain tuples of up to 15 types):
   Access to [entities and components][cb::ec]
 - [`QuerySet`][bevy::QuerySet] (with up to 4 queries):
   Resolve [conflicting queries][cb::queryset]
 - [`Local<T>`][bevy::Local]:
   Data [local][cb::local] to the system
 - [`EventReader<T>`][bevy::EventReader]:
   Receive [events][cb::event]
 - [`EventWriter<T>`][bevy::EventWriter]:
   Send [events][cb::event]
 - [`RemovedComponents<T>`][bevy::RemovedComponents]:
   [Removal detection][cb::removal-detection]
 - [`NonSend<T>`][bevy::NonSend]:
   Shared access to [Non-`Send`][cb::nonsend] (main thread only) data
 - [`NonSendMut<T>`][bevy::NonSendMut]:
   Mut access to [Non-`Send`][cb::nonsend] (main thread only) data
 - [`Entities`][bevy::Entities]:
   Low-level ECS metadata: All entities
 - [`Components`][bevy::Components]:
   Low-level ECS metadata: All components
 - [`Bundles`][bevy::Bundles]:
   Low-level ECS metadata: All bundles
 - [`Archetypes`][bevy::Archetypes]:
   Low-level ECS metadata: All archetypes
 - [`SystemChangeTick`][bevy::SystemChangeTick]:
   Low-level ECS metadata: Tick used for change detection
 - tuples containing any of these types, with up to 16 members
 
[]:#(ANCHOR: systemparam-limits)

Your function can have a maximum of 16 total parameters. If you need more,
group them into tuples to work around the limit. Tuples can contain up to
16 members, but can be nested indefinitely.

[]:#(ANCHOR_END: systemparam-limits)

[]:#(ANCHOR_END: systemparams)

[]:#(ANCHOR: components)
 - [`Transform`][bevy::Transform]:
   Local transform (relative to parent, if any)
 - [`GlobalTransform`][bevy::GlobalTransform]:
   Global transform (in the world)
 - [`Parent`][bevy::Parent]:
   Entity's parent, if in a hierarchy
 - [`Children`][bevy::Children]:
   Entity's children, if in a hierarchy
 - [`Timer`][bevy::Timer]:
   Track if a time interval has elapsed
 - [`Stopwatch`][bevy::Stopwatch]:
   Track how much time has passed
 - [`Handle<T>`][bevy::Handle]:
   Reference to an asset of specific type
 - [`Visibility`][bevy::Visibility]:
   Manually control visibility, whether to display the entity (hide/show)
 - [`RenderLayers`][bevy::RenderLayers]:
   Group entities into "layers" and control which "layers" a camera should display
 - [`Camera`][bevy::Camera]:
   Camera used for rendering
 - [`OrthographicProjection`][bevy::OrthographicProjection]:
   Orthographic projection for a camera
 - [`PerspectiveProjection`][bevy::PerspectiveProjection]:
   Perspective projection for a camera
 - [`Sprite`][bevy::Sprite]:
   (2D) Properties of a sprite, using a whole image
 - [`TextureAtlasSprite`][bevy::TextureAtlasSprite]:
   (2D) Properties of a sprite, using a sprite sheet
 - [`PointLight`][bevy::PointLight]:
   (3D) Properties of a point light
 - [`DirectionalLight`][bevy::DirectionalLight]:
   (3D) Properties of a directional light
 - [`NotShadowCaster`][bevy::NotShadowCaster]:
   (3D) Disable entity from producing dynamic shadows
 - [`NotShadowReceiver`][bevy::NotShadowReceiver]:
   (3D) Disable entity from having dynamic shadows of other entities
 - [`Wireframe`][bevy::Wireframe]:
   (3D) Draw object in wireframe mode
 - [`Node`][bevy::Node]:
   (UI) Mark entity as being controlled by the UI layout system
 - [`Style`][bevy::Style]:
   (UI) Layout properties of the node
 - [`Button`][bevy::Button]:
   (UI) Marker for a pressable button
 - [`Interaction`][bevy::Interaction]:
   (UI) Track interaction/selection state: if the node is clicked or hovered over
 - [`Text`][bevy::Text]:
   Text to be displayed

[]:#(ANCHOR_END: components)

[]:#(ANCHOR: gltf-asset-labels)
The following asset labels are supported (`{}` is the numerical index):
  - `Scene{}`: GLTF Scene as Bevy [`Scene`][bevy::Scene]
  - `Node{}`: GLTF Node as [`GltfNode`][bevy::GltfNode]
  - `Mesh{}`: GLTF Mesh as [`GltfMesh`][bevy::GltfMesh]
  - `Mesh{}/Primitive{}`: GLTF Primitive as Bevy [`Mesh`][bevy::Mesh]
  - `Texture{}`: GLTF Texture as Bevy [`Image`][bevy::Image]
  - `Material{}`: GLTF Material as Bevy [`StandardMaterial`][bevy::StandardMaterial]
  - `DefaultMaterial`: as above, if the GLTF file contains a default material with no index
[]:#(ANCHOR_END: gltf-asset-labels)
