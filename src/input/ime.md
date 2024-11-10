{{#include ../include/header014.md}}

# IME Input

Bevy has support for IMEs (Input Method Editors), which is how people perform
text input in languages with more complex scripts, like East Asian languages, and
how non-keyboard text input methods (such as handwriting recognition) work. It
requires some special handling from you, however.

If you'd like all international users to be able to input text in their
language, the way they usually do in other GUI apps on their OS, you should
support IMEs. If you want good accessibility for disabled users or users
who prefer alternative text input methods like handwriting recognition, you
should support IMEs. This should be in addition to supporting [text input via
the keyboard][input::keyboard-text], which is how most users will input text.

## How IMEs Work

IMEs work by using a special "buffer", which shows the current in-progress
text suggestions and allows users to preview and compose the next part of
their text before confirming it. The text suggestions are provided by the OS,
but your app needs to display them for the user.

For example, imagine you have a text input box in your UI. You show the text
that the user has already inputted, with a cursor at the end.

If IME is enabled, you will get [`Ime::Preedit`][`Ime`] [events][cb::event]
for "pending" text.  You should show that "unconfirmed" text in the text
input box, but with different formatting to be visually distinct.

When the user confirms their desired input, you will get an
[`Ime::Commit`][`Ime`] [event][cb::event] with the final text. You should
then discard any previous "uncofirmed" text and append the new text to your
actual text input string.

## How to support IMEs in your Bevy app

First, you need to inform the OS when your application is expecting text input.
You don't want the IME to accidentally activate during gameplay, etc.

Whenever you want the user to input text, you enable "IME mode" on the [`Window`].
When you are done, disable it.

If the user is not using an IME, nothing happens when you enable "IME mode". You
will still get [keyboard][input::keyboard] [events][cb::event] as usual and you
can [accept text input that way][input::keyboard-text].

If the user has an IME, you will get an [`Ime::Enabled`][`Ime`] event. At that point,
your application will no longer receive any [`KeyboardInput`] [events][cb::event].

You can then handle [`Ime::Preedit`][`Ime`] [events][cb::event] for pending/unconfirmed
text, and [`Ime::Commit`][`Ime`] for final/confirmed text.

```rust,no_run,noplayground
{{#include ../code014/src/input/ime.rs:ime}}
```

For the sake of brevity, this example just prints the events to the console.

In a real app, you will want to display the "pre-edit" text on-screen, and use
different formatting to show the cursor. On "commit", you can append the
provided text to the actual string where you normally accept text input.
