{{#include ../include/header014.md}}

# Gamepad (Controller, Joystick)

Relevant official examples:
[`gamepad_input`][example::gamepad_input],
[`gamepad_input_events`][example::gamepad_input_events].

---

Bevy has support for gamepad input hardware, using [gilrs][project::gilrs]:
console controllers, joysticks, etc. Many different kinds of hardware should
work, but if your device is not supported, you should file an issue with the
[gilrs][project::gilrs] project.

## Gamepad IDs

Bevy assigns a unique ID ([`Gamepad`]) to each connected gamepad. For local
multiplayer, this lets you associate each device with a specific player and
distinguish which one your inputs are coming from.

You can use the [`Gamepads`] [resource][cb::res] to list the IDs of all the
currently connected gamepad devices, or to check the status of a specific one.

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepads}}
```

### Handling Connections / Disconnections

To detect when gamepads are connected or disconnected, you can use
[`GamepadEvent`] [events][cb::event].

Example showing how to remember the first connected gamepad ID:

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-connect-disconnect}}
```

## Handling Gamepad Inputs

The `Axis<GamepadAxis>` ([`Axis`], [`GamepadAxis`]) [resource][cb::res]
keeps track of the current value of the different axes: X/Y for each thumb
stick, and the Z axes (the analog triggers).

Buttons can be handled with the `ButtonInput<GamepadButton>`
([`ButtonInput`], [`GamepadButton`]) [resource][cb::res], similar to [mouse
buttons][input::mouse-button] or [keyboard keys][input::keyboard].

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-input}}
```

Notice that the names of buttons in the [`GamepadButton`] `enum` are
vendor-neutral (like `South` and `East` instead of X/O or A/B).

Some game controllers have additional buttons and axes beyond what is available
on a standard controller, for example:
 - HOTAS (stick for flight sim)
 - steering wheel + pedals (for car driving games)

These are represented by the `Other(u8)` variant in [`GamepadButton`]/[`GamepadAxis`].
The `u8` value is hardware-specific, so if you want to support such devices,
your game needs to have a way for your users to configure their input bindings.

### Events

Alternatively, if you want to detect all activity as it comes in, you
can also handle gamepad inputs using [`GamepadEvent`] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-input-events}}
```

## Gamepad Settings

You can use the [`GamepadSettings`] [resource][cb::res] to configure dead-zones
and other parameters of the various axes and buttons. You can set the global
defaults, as well as individually per-axis/button.

Here is an example showing how to configure gamepads with custom settings
(not necessarily *good* settings, please don't copy these blindly):

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-settings}}
```

To tie the examples together: if you have the [system][cb::system] from the
[connect/disconnect example](#handling-connections--disconnections) earlier
above on this page, to update our `MyGamepad` resource, we can configure
the system from the above example with a [run condition][cb::rc], so that
the gamepad settings are updated whenever a new gamepad is connected and
selected to be used:

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-settings-app}}
```

## Gamepad Rumble

To cause rumble/vibration, use the [`GamepadRumbleRequest`] event. Every
event you send will add a "rumble" with a given intensity that lasts for
a given duration of time. As you send multiple events, each requested rumble
will be tracked independently, and the actual hardware vibration intensity
will be the sum of all the rumbles currently in progress.

You can also send a `Stop` event to immediately cancel any ongoing rumbling.

The intensity of each rumble is represented as two values: the "strong"
motor and the "weak" motor. These might produce different-feeling vibrations
on different hardware.

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-rumble}}
```
