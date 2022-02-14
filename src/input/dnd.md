# Drag-and-Drop (Files)

{{#include ../include/links.md}}

Relevant official examples:
[`drag_and_drop`][example::drag_and_drop].

---

Bevy supports the Drag-and-Drop gesture common on most desktop operating
systems, but only for files, not arbitrary data / objects.

If you drag a file (say, from the file manager app) into a Bevy app, Bevy
will produce a [`FileDragAndDrop`][bevy::FileDragAndDrop] [event][cb::event],
containing the path of the file that was dropped in.

Usually, in a graphical app, you may want to do different things depending
on where it was dropped. For this, you can [check the mouse cursor
position][input::cursor], or use a Bevy UI [`Interaction`][bevy::Interaction].

For example, here is how to detect if a file was dropped onto a
special UI widget/element (which we identify with a custom marker
[component][cb::component]):

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:dnd-file}}
```
