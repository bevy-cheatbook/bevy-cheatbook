{{#include ../include/header014.md}}

# Keyboard Input

Relevant official examples:
[`keyboard_input`][example::keyboard_input],
[`keyboard_input_events`][example::keyboard_input_events].

---

This page shows how to handle keyboard keys being pressed and released.

Note: Command Key on Mac corresponds to the Super/Windows Key on PC.

## Checking Key State

Most commonly for games, you might be interested in specific known keys and
detecting when they are pressed or released. You can check specific keys
using the [`ButtonInput<KeyCode>`][`ButtonInput`] [resource][cb::res].

 - Use `.pressed(…)`/`.released(…)` to check if a key is being held down
 - Use `.just_pressed(…)`/`.just_released(…)` to detect the actual press/release

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:res}}
```

## Keyboard Events

To get all keyboard activity, you can use [`KeyboardInput`] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:events}}
```

### Physical [`KeyCode`] vs. Logical [`Key`]

When a key is pressed, the [event][cb::event] contains two important pieces of information:
 - The [`KeyCode`], which always represents a specific key on the keyboard,
   regardless of the OS layout or language settings.
 - The [`Key`], which contains the logical meaning of the key as interpreted by the OS.

When you want to implement gameplay mechanics, you want to use the [`KeyCode`].
This will give you reliable keybindings that always work, including for multilingual
users with multiple keyboard layouts configured in their OS.

When you want to implement text/character input, you want to use the [`Key`].
This can give you Unicode characters that you can append to your text string and
will allow your users to type just like they do in other applications.

If you'd like to handle special function keys or media keys on keyboards that
have them, that can also be done via the logical [`Key`].

## Text Input

Here is a simple example of how to implement text input into a string (here
stored as a [local][cb::local]).

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:char}}
```

Note how we implement special handling for keys like `Backspace` and `Enter`.
You can easily add special handling for other keys that make sense in your
application, like arrow keys or the `Escape` key.

Keys that produce useful characters for our text come in as small Unicode
strings. It is possible that there might be more than one `char` per keypress
in some languages.

Note: To support text input for international users who use languages
with complex scripts (such as East Asian languages), or users who use
assistive methods like handwriting recognition, you also need to support
[IME input][input::ime], in addition to keyboard input.

## Keyboard Focus

If you are doing advanced things like caching state to detect multi-key
sequences or combinations of keys, you might end up in an inconsistent
state if the Bevy OS window loses focus in the middle of keyboard input,
such as with Alt-Tab or similar OS window switching mechanisms.

If you are doing such things and you think your algorithm might be getting
stuck, Bevy offers a [`KeyboardFocusLost`] [event][cb::event] to let you
know when you should reset your state.

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:focus-lost}}
```
