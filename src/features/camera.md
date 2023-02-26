# Cameras

{{#include ../include/links.md}}

Cameras drive all rendering in Bevy. They are responsible for configuring what
to draw, how to draw it, and where to draw it.

You must have at least one camera entity, in order for anything to be displayed
at all! If you forget to spawn a camera, you will get an empty black screen.

In the simplest case, you can create a camera with the default settings. Just
spawn an entity using [`Camera2dBundle`][bevy::Camera2dBundle] or
[`Camera3dBundle`][bevy::Camera3dBundle]. It will simply draw all renderable
entities that are [visible][cb::visibility].

This page gives a general overview of cameras in Bevy. Also see the dedicated
pages for [2D cameras][cb::camera-2d] and [3D cameras][cb::camera-3d].

Practical advice: always create [marker components][cb::component-marker] for
your camera entities, so that you can [query][cb::query] your cameras easily!

```rust,no_run,noplayground
{{#include ../code/src/features/camera/general.rs:simple}}
```

## The Camera Transform

Cameras have [transforms][cb::transform], which can be used to position or
rotate the camera. This is how you move the camera around.

For examples, see these [cookbook][chapter::cookbook] pages:
 - [3D pan-orbit camera][cookbook::pan-orbit-camera], like in 3D editor apps

If you are making a game, you should implement your own custom camera controls
that feel appropriate to your game's genre and gameplay.

### Zooming the camera

Do not use the transform scale to "zoom" a camera! It just stretches the image,
which is not "zooming". It might also cause other issues and incompatibilities.
Use the [projection][cb::camera-projection] to zoom.

For orthographic projections, change the projection's scale. This way you can be
confident about how exactly coordinate/units map to the screen. This also helps
avoid scaling artifacts with 2D assets.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/d2.rs:zoom}}
```

For 3D perspective projections, change the FOV. This achieves the desired 3D
effect of zooming with a lens, while keeping the camera at the same distance
from what it is looking at. Decrease the FOV to "zoom in" (make objects appear
closer). Increase the FOV to "zoom out" (make objects appear further away,
increase the stretching due to the perspective effect).

```rust,no_run,noplayground
{{#include ../code/src/features/camera/d3.rs:zoom}}
```

In some applications (like 3D editors), "zooming" might mean moving the camera
closer or farther away, instead of changing the FOV.

## Projection

The camera projection is responsible for mapping the coordinate system to the
viewport. This effectively determines the "coordinate space" you are working in.

Bevy provides two kinds of projections:
[`OrthographicProjection`][bevy::OrthographicProjection] and
[`PerspectiveProjection`][bevy::PerspectiveProjection]. They are configurable,
to be able to serve a variety of different use cases. See the dedicated pages
for [2D cameras][cb::camera-2d] and [3D cameras][cb::camera-3d] to learn more
about what you can do with them.

It is possible to implement your own [custom camera
projections][cb::camera-custom-projection]. This can give you full control over
the coordinate system. However, beware that things might behave in unexpected
ways if you violate Bevy's [coordinate system conventions][cb::coords]!

Note that Bevy uses a an [infinite reversed
Z][nvidia::infinite-reverse-z] configuration for 3D.

## HDR and Tonemapping

[See here!][cb::hdr]

## Render Target

The render target of a camera determines where the GPU will draw things to. It
could be a window (for outputting directly to the screen) or an
[`Image`][bevy::Image] [asset][cb::asset] (render-to-texture).

By default, cameras output to the primary window.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/general.rs:render-target}}
```

## Viewport

The viewport is an (optional) way to restrict a camera to a sub-area of its
render target, defined as a rectangle. That rectangle is effectively treated as
the "window" to draw in.

An obvious use-case are split-screen games, where you want a camera to only draw
to one half of the screen.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/general.rs:set-viewport}}
```

If you need to find out the area a camera renders to (the viewport, if
configured, or the entire window, if not):

```rust,no_run,noplayground
{{#include ../code/src/features/camera/general.rs:get-viewport}}
```

## Coordinate Conversion

[`Camera`][bevy::Camera] provides methods to help with coordinate conversion
between on-screen coordinates and world-space coordinates. For an example, see
the ["cursor to world"][cookbook::cursor2world] cookbook page.

## Clear Color

This is the "background color" that the whole viewport will be cleared to,
before a camera renders anything.

You can also disable clearing on a camera, if you want to preserve all the
pixels as they were before.

[See this page for more info.][cb::clearcolor]

## Render Layers

[`RenderLayers`][bevy::RenderLayers] is a way to filter what entities should be
drawn by what cameras. Insert this [component][cb::component] onto your entities
to place them in specific "layers". The layers are integers from 0 to 31 (32
total available).

Inserting this component onto a camera entity selects what layers that camera
should render. Inserting this component onto renderable entities selects what
cameras should render those entities. An entity will be rendered if there is any
overlap between the camera's layers and the entity's layers (they have at least
one layer in common).

If an entity does not have the [`RenderLayers`][bevy::RenderLayers] component,
it is assumed to belong to layer 0 (only).

```rust,no_run,noplayground
{{#include ../code/src/features/camera/general.rs:renderlayers}}
```

You can also modify the render layers of entities after they are spawned.

## Camera Ordering

A camera's priority is a simple integer value that controls the order relative
to any other cameras with the same render target.

For example, if you have multiple cameras that all render to the primary window,
they will behave as multiple "layers". Cameras with higher priority will render
"on top of" cameras with lower priority.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/general.rs:overlay}}
```

## UI Rendering

Bevy UI rendering is integrated into the cameras! Every camera will, by default,
also draw UI.

However, if you are working with multiple cameras, you probably only want your
UI to be drawn once (probably by the main camera). You can disable UI rendering
on your other cameras.

Also, UI on multiple cameras is currently broken in Bevy. Even if you want
multiple UI cameras (say, to display UI in an app with multiple windows), it
does not work correctly.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/general.rs:no-ui}}
```

## Disabling Cameras

You can deactivate a camera without despawning it. This is useful when you want
to preserve the camera entity and all the configuration it carries, so you can
easily re-enable it later.

Some example use cases: toggling an overlay, switching between a 2D and 3D view.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/general.rs:is_active}}
```

## Multiple Cameras

This is an overview of different scenarios where you would need more than one
camera entity.

### Multiple Windows

Official example: [`multiple_windows`][example::multiple_windows].

If you want to create a Bevy app with multiple windows, you need to spawn
multiple cameras, one for each window, and set their render targets
respectively. Then, you can use your cameras to control what to display in each
window.

### Split-Screen

Official example: [`split_screen`][example::split_screen].

You can set the camera [viewport](#viewport) to only render to a part of the
render target. This way, a camera can be made to render one half of the screen
(or any other area). Use a separate camera for each view in a split-screen game.

### Overlays

Official example: [`two_passes`][example::two_passes].

You might want to render multiple "layers" (passes) to the same render target.
An example of this might be an overlay/HUD to be displayed as an overlay on top
of the main game.

The overlay camera could be completely different from the main camera. For
example, the main camera might draw a 3D scene, and the overlay camera might
draw 2D shapes. Such use cases are possible!

Use a separate camera to create the overlay. Set the [priority](#priority)
higher, to tell Bevy to render it after (on top of) the main camera. Make sure
to disable [clearing](#clear-color)!

Think about which camera you want to be responsible for [rendering the
UI](#ui-rendering). Use the overlay camera if you want it to be unaffected,
or use the main camera if you want the overlay to be on top of the UI. Disable
it on the other camera.

Use [Render Layers](#render-layers) to control what entities should be rendered
by each camera.

### Render to Image

(aka Render to Texture)

Official example: [`render_to_texture`][example::render_to_texture].

If you want to generate an image in memory, you can output to an `Image` asset.

This is useful for intermediate steps in games, such as rendering a minimap or
the gun in a shooter game. You can then use that image as part of the final
scene to render to the screen. Item previews are a similar use case.

Another use case is window-less applications that want to generate image files.
For example, you could use Bevy to render something, and then export it to a PNG
file.

