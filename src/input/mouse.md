Warning: this page has not been updated for Bevy 0.10 yet!

# Mouse

{{#include ../include/links.md}}

Relevant official examples:
[`mouse_input`][example::mouse_input],
[`mouse_input_events`][example::mouse_input_events].

---

## Mouse Buttons

Similar to [keyboard input][input::keyboard], mouse buttons are available
as an [`Input`][bevy::Input] state [resource][cb::res], as well as
[events][cb::event].

You can check the state of specific mouse buttons using
[`Input<MouseButton>`][bevy::MouseButton]:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:mouse-button-input}}
```

To get all press/release activity, use
[`MouseButtonInput`][bevy::MouseButtonInput] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:mouse-button-events}}
```

## Mouse Scrolling / Wheel

To detect scrolling input, use [`MouseWheel`][bevy::MouseWheel] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:scroll-events}}
```

The [`MouseScrollUnit`][bevy::MouseScrollUnit] enum is important: it tells
you the type of scroll input. `Line` is for hardware with fixed steps, like
the wheel on desktop mice. `Pixel` is for hardware with smooth (fine-grained)
scrolling, like laptop touchpads.

You should probably handle each of these differently (with different
sensitivity settings), to provide a good experience on both types of hardware.

**Note:** the `Line` unit is not guaranteed to have whole number values/steps!
At least macOS does non-linear scaling / acceleration of scrolling at the OS
level, meaning your app will get a fractional number of lines even when using a
regular PC mouse with a scroll wheel.

## Mouse Motion

Use this if you don't care about the exact position of the mouse cursor,
but rather you just want to see how much it moved from frame to frame. This
is useful for things like controlling a 3D camera.

Use [`MouseMotion`][bevy::MouseMotion] [events][cb::event]. Whenever the
mouse is moved, you will get an event with the delta.

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:mouse-motion}}
```

You might want to [grab/lock the mouse inside the game
window][cookbook::mouse-grab].

## Mouse Cursor Position

Use this if you want to accurately track the position pointer / cursor. This is
useful for things like clicking and hovering over objects in your game or UI.

You can get the current coordinates of the mouse pointer, from the respective
[`Window`][bevy::Window] (if the mouse is currently inside that window):

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:cursor-position}}
```

To detect when the pointer is moved, use [`CursorMoved`][bevy::CursorMoved]
[events][cb::event] to get the updated coordinates:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:cursor-events}}
```

Note that you can only get the position of the mouse inside a window;
you cannot get the global position of the mouse in the whole OS Desktop /
on the screen as a whole.

The coordinates you get are in "window space". They represent window
pixels, and the origin is the bottom left corner of the window. They do not
relate to your camera or in-game coordinates in any way. [See this cookbook
example][cookbook::cursor2world] for converting these window cursor coordinates
into world-space coordinates.

To track when the mouse cursor enters and leaves your window(s), use
[`CursorEntered`][bevy::CursorEntered] and [`CursorLeft`][bevy::CursorLeft]
[events][cb::event].
