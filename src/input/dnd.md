{{#include ../include/header012.md}}

# Drag-and-Drop (Files)

Relevant official examples:
[`drag_and_drop`][example::drag_and_drop].

---

Bevy supports the Drag-and-Drop gesture common on most desktop operating
systems, but only for files, not arbitrary data / objects.

If you drag a file (say, from the file manager app) into a Bevy app, Bevy
will produce a [`FileDragAndDrop`][bevy::FileDragAndDrop] [event][cb::event],
containing the path of the file that was dropped in.

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:dnd-file}}
```

## Detecting the Position of the Drop

You may want to do different things depending on where the cursor was when the
drop gesture ended. For example, add the file to some collection, if it was
dropped over a specific UI element/panel.

Unfortunately, this is currently somewhat tricky to implement, due to [`winit`
bug #1550][winit::1550]. Bevy does not get [`CursorMoved`][bevy::CursorMoved]
[events][cb::event] while the drag gesture is ongoing, and therefore does not
respond to the mouse cursor. Bevy completely loses track of the cursor position.

Checking the cursor position from the [`Window`][bevy::Window] will also not work.

Systems that use cursor events to respond to cursor movements will not work
during a drag gesture. This includes Bevy UI's [`Interaction`][bevy::Interaction]
detection, which is the usual way of detecting when a UI element is hovered over.

### Workaround

The only way to workaround this issue is to store the file path somewhere
temporarily after receiving the drop event. Then, wait until the next
[`CursorMoved`][bevy::CursorMoved] event, and then process the file.

Note that this might not even be on the next frame update. The next cursor
update will happen whenever the user moves the cursor. If the user does not
immediately move the mouse after dropping the file and leaves the cursor in the
same place for a while, there will be no events and your app will have no way of
knowing the cursor position.
