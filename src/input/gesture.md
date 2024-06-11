{{#include ../include/header014.md}}

# Gestures

Multi-finger gestures on a Touchpad or Touchscreen are a very common
way to implement various operations, like panning, zooming, and rotating.

## Platform Gesture Events

Bevy offers [events][cb::event] that allow you to handle gestures as they
are detected / implemented by the OS.

Currently, only macOS and iOS are supported. Other platforms may be supported
in the future.

The supported gestures are:

 - [`RotationGesture`]: rotating with two fingers
 - [`PinchGesture`]: pinch-to-zoom with two fingers
 - [`PanGesture`]: panning gesture
 - [`DoubleTapGesture`]: double-tap gesture

```rust,no_run,noplayground
{{#include ../code014/src/input/gesture.rs:platform-gesture-events}}
```

## Custom Touchpad Gestures

It is not currently possible to implement your own gestures on a touchpad,
because there is no API to detect the individual fingers that are touching
the touchpad.

## Custom Touchscreen Gestures

You can (and probably should) implement your own touchscreen gestures. Bevy
offers multi-touch detection, tracking each finger that is currently on the
screen. Implementing your own gestures is be a good way to make touchscreen
input behave appropriately to your application.

[See here for more info on touchscreen input in Bevy.][input::touch]
