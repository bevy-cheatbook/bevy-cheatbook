# HDR, Tonemapping, Bloom

{{#include ../include/links.md}}

HDR (High Dynamic Range) refers to the ability of the game engine to handle very
bright lights or colors. Bevy's rendering is HDR. This means you can have
objects with colors that go above `1.0`, very bright lights, or bright emissive
materials. All of this is supported for both 3D and 2D.

This is not to be confused with HDR display output, which is the ability to
produce a HDR image to be displayed by a modern monitor or TV with HDR
capabilities. Bevy has no support for this yet.

## Camera HDR configuration

There is a per-camera toggle that lets you decide whether you want Bevy to
preserve the HDR data internally during rendering.

```rust,no_run,noplayground
{{#include ../code/src/features/camera/hdr.rs:hdr-config}}
```

If it is enabled, Bevy's intermediate textures will be in HDR format. The
shaders output HDR values and Bevy will store them, so they can be used in later
rendering passes. This allows you to enable post-processing effects like Bloom,
that make use of the HDR data. [Tonemapping][cb::tonemap] will happen as a
post-processing step.

If it is disabled, the shaders are expected to output standard RGB colors in the
0.0 to 1.0 range. [Tonemapping][cb::tonemap] happens in the shader. The HDR information
is not preserved. Effects that require HDR data, like Bloom, will not work.

It is disabled by default. If enabling it, make sure to also have
[tonemapping][cb::tonemap] enabled, ideally with deband dithering.

If you have both HDR and MSAA enabled, it is possible you might encounter
issues. There might be visual artifacts in some cases. It is also unsupported on
Web/WASM, crashing at runtime. Disable MSAA if you experience any such issues.

## Bloom

The "Bloom" effect creates a glow around bright lights. It is not a
physically-accurate effect, but it does a good job of helping the perception of
very bright light, especially when outputting HDR to the display hardware is not
supported.

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

## Deband Dithering

Deband dithering helps color gradients or other areas with subtle changes in
color to appear higher-quality, without a "color banding" effect.

It is enabled by default, and can be disabled per-camera.

Here is an example image without dithering (top) and with dithering (bottom).
Pay attention to the quality/smoothness of the green color gradient on the
ground plane. In games with photorealistic graphics, similar situations can
arise in the sky, in dark rooms, or lights glowing with a bloom effect.

![Visual comparison of a scene simple cube on a flat green plane, with dithering disabled/enabled.](/img/dithering.png)
