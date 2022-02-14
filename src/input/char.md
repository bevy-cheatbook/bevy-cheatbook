# Text / Character Input

{{#include ../include/links.md}}

Relevant official examples:
[`char_input_events`][example::char_input_events].

---

Use this (*not* [keyboard input][input::keyboard]) if you want to implement
text input in a Bevy app. This way, everything works as the user expects
from their operating system, including Unicode support.

Bevy will produce a [`ReceivedCharacter`][bevy::ReceivedCharacter]
[event][cb::event] for every Unicode code point coming from the OS.

This example shows how to let the user input text into a string (here stored
as a [local resource][cb::local]).

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:text}}
```
