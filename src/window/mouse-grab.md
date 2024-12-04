{{#include ../include/header014.md}}

# Grabbing the Mouse

Relevant official examples:
[`mouse_grab`][example::mouse_grab].

---

For some genres of games, you want to the mouse to be restricted to the window,
to prevent it from leaving the window during gameplay.

There are two variations on this behavior ([`CursorGrabMode`]):
 - `Confined` allows the cursor to be moved, but only within the bounds of the window.
 - `Locked` fixes the cursor in place and does not allow it to move.
   - Relative [mouse motion][input::mouse-motion] [events][cb::event] still work.

To grab the cursor:

```rust,no_run,noplayground
{{#include ../code014/src/window/mouse_grab.rs:grab}}
```

To release the cursor:

```rust,no_run,noplayground
{{#include ../code014/src/window/mouse_grab.rs:ungrab}}
```

You should grab the cursor during active gameplay and release it when
the player pauses the game / exits to menu / etc.

For relative mouse movement, you should use [mouse motion][input::mouse-motion]
instead of [cursor input][input::cursor] to implement your gameplay.

## Platform Differences

macOS does not natively support `Confined` mode. Bevy will fallback to `Locked`.
If you want to support macOS and you want to use [cursor input][input::cursor],
you might want to implement a "virtual cursor" instead.

Windows does not natively support `Locked` mode. Bevy will fallback to `Confined`.
You could emulate the locked behavior by re-centering the cursor every frame:

```rust,no_run,noplayground
{{#include ../code014/src/window/mouse_grab.rs:recenter}}
```

```rust,no_run,noplayground
{{#include ../code014/src/window/mouse_grab.rs:recenter-app}}
```
