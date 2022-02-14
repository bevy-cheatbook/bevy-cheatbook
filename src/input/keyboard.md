# Keyboard Input

{{#include ../include/links.md}}

Relevant official examples:
[`keyboard_input`][example::keyboard_input],
[`keyboard_input_events`][example::keyboard_input_events].

---

This page shows how to handle keyboard keys being pressed and released.

If you are interested in text input, see the [Character Input][input::char] page instead.

Note: Command Key on Mac corresponds to the Super/Windows Key on PC.

## Checking Key State

Checking the state of specific keys can currently only be done by Key Code,
using the `Input<KeyCode>` ([`Input`][bevy::Input], [`KeyCode`][bevy::KeyCode])
[resource][cb::res]:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:keyboard-input}}
```

## Keyboard Events

To get all keyboard activity, you can use
[`KeyboardInput`][bevy::KeyboardInput] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:keyboard-events}}
```

These events give you both the Key Code and Scan Code.
The Scan Code is represented as an arbitrary `u32` integer ID.

## Key Codes and Scan Codes

Keyboard keys can be identified by Key Code or Scan Code.

Key Codes represent the symbol/letter on each key and are dependent on the
keyboard layout currently active in the user's OS. Bevy represents them with
the [`KeyCode`][bevy::KeyCode] enum.

Scan Codes represent the physical key on the keyboard, regardless of the
system layout. Unfortunately, they are just arbitrary integer IDs and
platform-dependent. There is no easy way to know what to display in the
game's UI for the user, from the scan code.

Additionally, support for using Scan Codes in Bevy is limited. This can be
annoying for people with multiple or non-QWERTY keyboard layouts.

See [Bevy Issue #2052][bevy::2052] for efforts to improve this situation.

### Layout-Agnostic Key Bindings

You could try to provide a better experience for players, regardless of these
limitations, by:
 - Internally recording and storing key bindings as Scan Codes
 - Handling input using [events](#keyboard-events) and using Scan Codes to identify the key
 - Storing Key Codes only for displaying the name of a key in the UI

Doing things this way means that users with multiple keyboard layouts in the
OS will not have their keybindings break if they accidentally switch their
layout mid-game, or start the game with the wrong layout.

Unfortunately, this also means your game UI will display the symbol from the
layout that was used when registering the key bindings. This may be wrong
or confusing if the user has changed the currently active layout.
