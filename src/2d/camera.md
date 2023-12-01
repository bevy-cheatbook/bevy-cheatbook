{{#include ../include/header012.md}}

# 2D Camera Setup

[Cameras][cb::camera] in Bevy are mandatory to see anything: they configure the
rendering.

This page will teach you about the specifics of 2D cameras. If you want to learn about
general non-2D specific functionality, see the [general page on cameras][cb::camera].

## Creating a 2D Camera

Bevy provides a [bundle][cb::bundle] ([`Camera2dBundle`][bevy::Camera2dBundle])
that you can use to [spawn][cb::commands] a camera [entity][cb::entity]. It
has reasonable defaults to set up everything correctly.

You might want to set the [transform][cb::transform], to position the camera.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:basic-setup}}
```

## Projection

The [projection][cb::camera-projection] is what determines how coordinates map to the
[viewport][cb::camera-viewport] (commonly, the screen/window).

2D cameras always use an Orthographic projection.

When you spawn a 2D camera using [`Camera2dBundle`][bevy::Camera2dBundle],
it adds the [`OrthographicProjection`][bevy::OrthographicProjection]
[component][cb::component] to your [entity][cb::entity]. When
you are working with 2D cameras and you want to access
the projection, you should [query][cb::query] for
[`OrthographicProjection`][bevy::OrthographicProjection].

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:query-projection}}
```

Note that this is different from [3D][cb::camera-3d::projection]. If you are
making a library or some other code that should be able to handle both 2D and
3D, you cannot make a single [query][cb::query] to access both 2D and 3D
cameras. You should create separate [systems][cb::system], or at least two
separate queries, to handle each kind of camera. This makes sense, as you will
likely need different logic for 2D vs. 3D anyway.

### Caveat: near/far values

The projection contains the `near` and `far` values, which indicate the minimum
and maximum Z coordinate (depth) that can be rendered, relative to the position
([transform][cb::transform]) of the camera.

[`Camera2dBundle`][bevy::Camera2dBundle] sets them appropriately for 2D:
`-1000.0` to `1000.0`, allowing entities to be displayed on both positive and
negative Z coordinates. However, if you create the
[`OrthographicProjection`][bevy::OrthographicProjection] yourself, to change any
other settings, you need to set these values yourself. The default value of the
[`OrthographicProjection`][bevy::OrthographicProjection] struct is designed for
3D and has a `near` value of `0.0`, which means you might not be able to see
your 2D entities.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:projection-near-far}}
```

A more foolproof way to go about this is to use a temporary variable, to let the
bundle do its thing, and then mutate whatever you want. This way, you don't have
to worry about the exact values or getting anything wrong:

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:projection-mut}}
```

### Scaling Mode

You can set the [`ScalingMode`][bevy::ScalingMode] according to how you want to
handle window size / resolution.

The default for Bevy 2D cameras is to have 1 screen pixel correspond to 1 world
unit, thus allowing you to think of everything in "pixels". When the window is
resized, that causes more or less content to be seen.

If you want to keep this window resizing behavior, but change the mapping of screen
pixels to world units, use `ScalingMode::WindowSize(x)` with a value other than `1.0`.
The value represents the number of screen pixels for one world unit.

If, instead, you want to always fit the same amount of content
on-screen, regardless of resolution, you should use something like
`ScalingMode::FixedVertical` or `ScalingMode::AutoMax`. Then, you can directly
specify how many units you want to display on-screen, and your content will
be upscaled/downscaled as appropriate to fit the window size.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:scalingmode}}
```

### Zooming

To "zoom" in 2D, you can change the orthographic projection's `scale`. This
allows you to just scale everything by some factor, regardless of the
[`ScalingMode`][bevy::ScalingMode] behavior.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:zoom-scale}}
```

Alternatively, you can reconfigure the [`ScalingMode`][bevy::ScalingMode]. This
way you can be confident about how exactly coordinates/units map to the
screen. This also helps avoid scaling artifacts with 2D assets, especially
pixel art.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:zoom-scalingmode}}
```

Consider having a list of predefined "zoom levels" / scale values, so that you
can make sure your game always looks good.

If you are making a pixel-art game, you want to make sure the default texture
filtering mode is set to Nearest (and not Linear), if you want your pixels
to appear crisp instead of blurry:

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:default-nearest}}
```

However, when *downscaling*, Linear (the default) filtering is preferred
for higher quality. So, for games with high-res assets, you want to leave
it unchanged.
