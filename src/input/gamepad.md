# Gamepad (Controller, Joystick)

{{#include ../include/links.md}}

Relevant official examples:
[`gamepad_input`][example::gamepad_input],
[`gamepad_input_events`][example::gamepad_input_events].

---

Bevy has support for gamepad input hardware: console controllers,
joysticks, etc. Many different kinds of hardware should work, but
if your device is not supported, you should file an issue with the
[gilrs](https://gitlab.com/gilrs-project/gilrs) project.

## Gamepad IDs

Bevy assigns a unique ID ([`Gamepad`][bevy::Gamepad]) to each connected
gamepad. This lets you associate the device with a specific player and
distinguish which one your inputs are coming from.

You can use the [`Gamepads`][bevy::Gamepads] [resource][cb::res] to list
the IDs of all the currently connected gamepad devices, or to check the
status of a specific one.

To detect when gamepads are connected or disconnected, you can use
[`GamepadEvent`][bevy::GamepadEvent] [events][cb::event].

Example showing how to remember the first connected gamepad ID:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:gamepad-connect-disconnect}}
```

## Handling Gamepad Inputs

You can handle the analog sticks and triggers with `Axis<GamepadAxis>`
([`Axis`][bevy::Axis], [`GamepadAxis`][bevy::GamepadAxis]). Buttons
can be handled with `Input<GamepadButton>` ([`Input`][bevy::Input],
[`GamepadButton`][bevy::GamepadButton]), similar to [mouse
buttons][input::mouse-button] or [keyboard keys][input::keyboard].

Notice that the names of buttons in the [`GamepadButton`][bevy::GamepadButton]
are vendor-neutral (like `South` and `East` instead of X/O or A/B).

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:gamepad-input}}
```

You can also handle gamepad inputs using [`GamepadEvent`][bevy::GamepadEvent] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:gamepad-input-events}}
```

## Gamepad Settings

You can use the [`GamepadSettings`][bevy::GamepadSettings] [resource][cb::res]
to configure dead-zones and other parameters of the various axes and
buttons. You can set the global defaults, as well as individually
per-axis/button.

Here is an example showing how to configure gamepads with custom settings
(not necessarily *good* settings, please don't copy these blindly):

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:gamepad-settings}}
```
