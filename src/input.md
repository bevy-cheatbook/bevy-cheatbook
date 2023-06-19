{{#include ./include/header011.md}}

# Input Handling

Bevy supports the following inputs:
  - [Keyboard][input::keyboard] (detect when keys are pressed or released)
  - [Character][input::char] (for text input; keyboard layout handled by the OS)
  - [Mouse][input::mouse] (relative motion, buttons, scrolling)
    - [Motion][input::mouse-motion] (moving the mouse, not tied to OS cursor)
    - [Cursor][input::cursor] (absolute pointer position)
    - [Buttons][input::mouse-button]
    - [Scrolling][input::mouse-wheel] (mouse wheel or touchpad gesture)
    - [Zoom/Rotate touchpad gestures][input::touchpad-gesture]
  - [Touchscreen][input::touch] (with multi-touch)
  - [Gamepad (Controller, Joystick)][input::gamepad] (via the [gilrs][project::gilrs] library)

The following notable input devices are ***not*** supported:
 - Accelerometers and gyroscopes for device tilt
 - Other sensors, like temperature sensors
 - Tracking individual fingers on a multi-touch trackpad, like on a touchscreen
 - Microphones and other audio input devices
 - MIDI (musical instruments), but there is an unofficial plugin: [`bevy_midi`][project::bevy_midi].

---

For most input types (where it makes sense), Bevy provides two ways of
dealing with them:
  - by checking the current state via [resources][bevy::res] ([input resources][builtins::res-input]),
  - or via [events][cb::event] ([input events][builtins::event-input]).

Some inputs are only provided as events.

Checking state is done using [resources][cb::res] such as
[`Input`][bevy::Input] (for binary inputs like keys or buttons),
[`Axis`][bevy::Axis] (for analog inputs), [`Touches`][bevy::Touches]
(for fingers on a touchscreen), etc. This way of handling input is very
convenient for implementing game logic. In these scenarios, you typically
only care about the specific inputs mapped to actions in your game. You can
check specific buttons/keys to see when they get pressed/released, or what
their current state is.

[Events][cb::event] ([input events][builtins::event-input]) are a lower-level,
more all-encompassing approach. Use them if you want to get all activity
from that class of input device, rather than only checking for specific inputs.

## Input Mapping

Bevy does not yet offer a built-in way to do input mapping (configure key
bindings, etc). You need to come up with your own way of translating the
inputs into logical actions in your game/app.

There are some community-made plugins that may help with that: [see the
input-section on bevy-assets][bevyassets::input]. My personal recommendation:
[Input Manager plugin by Leafwing Studios][project::lwim].

It may be a good idea to build your own abstractions specific to your
game. For example, if you need to handle player movement, you might want to
have a system for reading inputs and converting them to your own internal
"movement intent/action events", and then another system acting on those
internal events, to actually move the player. Make sure to use [explicit
system ordering][cb::system-order] to avoid lag / frame delays.

## Run Conditions

Bevy also provides [run conditions][cb::rc] ([see all of them
here][bevy::input::common_conditions]) that you can attach to your systems, if
you want a specific system to only run when a specific key or button is pressed.

This way, you can do input handling as part of the
[scheduling/configuration][cb::ecs-intro-code] of your [systems][cb::system], and
avoid running unnecessary code on the CPU.

Using these in real games is not recommended, because you have to hard-code the
keys, which makes it impossible to make user-configurable keybindings.

To support configurable keybindings, you can implement your own run conditions
that check your keybindings from the user settings.

If you are using the [LWIM plugin][project::lwim], it also provides support for
[a similar run-condition-based workflow][lwim::common_conditions].
