# HDR, Tonemapping, Bloom, Dithering

{{#include ../include/links.md}}

HDR (High Dynamic Range) refers to the ability of the game engine to handle very
bright lights or colors. Bevy supports HDR rendering. This means you can have
objects with colors that go above `1.0`, very bright lights, or bright emissive
materials. All of this is supported for both 3D and 2D.

This is not to be confused with HDR display output, which is the ability to
produce a HDR image to be displayed by a modern monitor or TV with HDR
capabilities. Bevy has no support for this yet.

## Camera HDR configuration

There is a per-camera toggle that lets you decide whether you want Bevy to
store the HDR data internally during rendering.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/hdr.rs:hdr-config}}
```

If it is enabled, Bevy's intermediate textures will be in HDR format. This
allows you to enable post-processing effects like Bloom, that make use of the
HDR rendering data. Tonemapping will happen as a post-processing step.

If it is disabled, the shaders are expected to do any [tonemapping][cb::tonemap]
internally, and output standard RGB colors. The "physically based" materials can
still handle bright lights and other "HDR" scenarios, but that information will
not be stored, and effects like Bloom will not work.

Make sure to also have [tonemapping][cb::tonemap] enabled, ideally with
dithering.

Bevy's HDR rendering is known to cause issues with MSAA. There might be visual
artifacts in some cases. It is also unsupported on Web/WASM. Disable MSAA if you
experience these issues.

## Bloom

The "Bloom" effect creates a glow around bright lights. It is not a
physically-accurate effect, but it does a good job of helping the perception of
very bright light.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/hdr.rs:bloom-config}}
```

![The Bloom effect on street lamps.](/img/bloom.png)

## Tonemapping

Tonemapping is the step of the rendering process where the colors of pixels are
converted from their in-engine intermediate repesentation into the final values
as they should be displayed on-screen.

This is very important with HDR applications, as in that case the image can
contain very bright pixels (above 1.0) which need to be remapped into a range
that can be displayed.

Tonemapping is enabled by default. Bevy gives you a simple toggle
([`Tonemapping`][bevy::Tonemapping]) to disable it, per-camera. This is not
recommended, unless you know you only have very simple graphics that don't need
it. It can make your graphics look incorrect.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/hdr.rs:tonemap-config}}
```

Bevy does not currently offer any way to customize the tonemapping operation,
only a simple toggle. The tonemapping operation is Luminance-weighted Reinhard.

### Dithering

Bevy's tonemapping settings also control dithering. Dithering helps color gradients or other areas with
subtle changes in color to appear higher-quality, without a "color banding" effect.

Dithering is enabled by default.

Here is an example image without dithering (top) and with dithering (bottom).
Pay attention to the quality/smoothness of the green color gradient on the
ground plane. In games with photorealistic graphics, similar situations can
arise in the sky, in dark rooms, or lights glowing with a bloom effect.

![Visual comparison of a scene simple cube on a flat green plane, with dithering disabled/enabled.](/img/dithering.png)
