# Input Handling

Relevant official examples: [everything in the "Input" category](https://github.com/bevyengine/bevy/tree/latest/examples#input).

---

[Click here to download the example code.](../code/examples/input.rs)

This is a complete example that you can run. It will print all input activity
to the console.

---

Bevy supports the following inputs:
  - Keyboard
  - Mouse (relative motion, buttons, scrolling)
  - Cursor (absolute pointer position)
  - Touchscreen (with multi-touch)
  - Most controllers/gamepads/joysticks (via the [gilrs](https://gitlab.com/gilrs-project/gilrs) library)

Sensors like accelerometers, gyroscopes, VR head tracking, are not supported yet.

For most input types (where it makes sense), Bevy provides two ways of
dealing with them:
  - by checking the current state via [resources](../programming/res.md),
  - or via [events](../programming/events.md).

Some inputs are only provided as events.

Checking state is done using [resources](../programming/res.md) such as
`Input` (for binary inputs like keys or buttons), `Axis` (for analog inputs),
`Touches` (for fingers on a touchscreen), etc. This way of handling input is
very convenient for implementing game logic. In these scenarios, you typically
only care about the specific inputs mapped to actions in your game. You can
check specific buttons/keys to see when they get pressed/released, or what
their current state is.

Events are a lower-level, more all-encompassing approach. Use them if you
want to get all activity from that class of input device, rather than only
checking for specific inputs. For example, if you are implementing text
input / typing, you probably want to receive all keyboard activity.

Since bevy resources and events are global and accessible from any system,
you don't need to centralize your input handling code in one place. You can
check any inputs you care about, from any system.

## Input Mapping

Bevy does not yet offer a built-in way to do input mapping (configure key
bindings, etc). You need to come up with your own way of translating the
inputs into logical actions in your game/app.

There are some community-made plugins that may help with that: [see the
input-section on bevy-assets](https://bevyengine.org/assets/#input).

It may be a good idea to build your own abstractions specific to your
game. For example, if you need to handle player movement, you might want to
have a system for reading inputs and converting them to your own internal
"movement intent/action events", and then another system acting on those
internal events, to actually move the player. Make sure to use [explicit
system ordering](../programming/system-order.md) to avoid [lag / frame
delays](../pitfalls/frame-delay.md).

## How to work with different input devices

### Keyboard

Keyboard keys can be identified by Scan Code or Key Code.

Note: Command Key on Mac corresponds to the Super/Windows Key on PC.

Scan Codes represent the physical key on the keyboard, regardless of the
system layout. Key Codes represent the symbol/letter on each key and are
dependent on the keyboard layout.

Unfortunately, support for using Scan Codes in Bevy is limited. This can be
annoying for people with non-QWERTY keyboard layouts.

Checking the state of specific keys can currently only be done by Key Code,
using the `Input<KeyCode>` resource:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:keyboard-input}}
```

To get all keyboard activity, you can use `KeyboardInput` events:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:keyboard-events}}
```

These events give you both the Key Code and Scan Code, but unfortunately,
the Scan Code is just represented as an arbitrary `u32` integer ID (which
means it is difficult to figure out what actual physical key it represents).

### Mouse Buttons

Similar to keyboard input, mouse buttons are available as an `Input` state
resource, as well as events.

You can check the state of specific mouse buttons using `Input<MouseButton>`:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:mouse-button-input}}
```

To get all press/release activity, use `MouseButtonInput` events:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:mouse-button-events}}
```

### Mouse Motion

If you need to detect relative mouse motion, you can use `MouseMotion` events.
Whenever the mouse is moved, you will get an event with the delta.

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:mouse-motion}}
```

You might want to [grab/lock the mouse inside the game
window](../cookbook/mouse-grab.md).

### Mouse Cursor

You can get the current coordinates of the mouse pointer, from the respective
`Window`:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:cursor-position}}
```

To detect when the pointer is moved, use `CursorMoved` events to get the
updated coordinates:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:cursor-events}}
```

Note that you can only get the position of the mouse inside a window;
you cannot get the global position of the mouse in the whole OS Desktop /
on the screen as a whole.

### Scrolling / Mouse Wheel

To detect scrolling input, use `MouseWheel` events:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:scroll-events}}
```

The `MouseScrollUnit` enum is important: it tells you the type of scroll
input.  `Line` is for hardware with fixed steps, like the wheel on desktop
mice. `Pixel` is for hardware with smooth (fine-grained) scrolling, like
laptop touchpads.

You should probably handle each of these differently (with different
sensitivity settings), to provide a good experience on both types of hardware.

### Controller / Gamepad / Joystick

You can detect when controllers/gamepads are connected or disconnected using
`GamepadEvent`. Do this to get the ID of the device and assign it to a
"player" in your game. You can easily support local multiplayer.

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:gamepad-connect-disconnect}}
```

The other event types are for changes to axes and buttons. They are not
shown above, because handling gamepad input via events is inconvenient.

You can handle the analog sticks with `Axis<GamepadAxis>`. Buttons can be
handled with `Input<GamepadButton>` (similar to mouse buttons or keyboard
keys).

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:gamepad-input}}
```

Notice that the names of buttons in the `enum` are vendor-neutral (like
`South` and `East` instead of X/O or A/B). Many different kinds of hardware
should work, but if your device is not supported, you should file an issue
with the [gilrs](https://gitlab.com/gilrs-project/gilrs) project.

### Touchscreen

Multi-touch touchscreens are supported. You can track multiple fingers on
the screen, with position and pressure/force information. Bevy does not
offer gesture recognition.

The `Touches` resource allows you to track any fingers currently on the screen:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:touches}}
```

Alternatively, you can use `TouchInput` events, but this is often harder
to use:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:touch-events}}
```
