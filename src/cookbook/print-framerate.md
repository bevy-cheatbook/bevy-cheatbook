{{#include ../include/header013.md}}

# Show Framerate

You can use Bevy's builtin diagnostics to measure framerate (FPS), for
monitoring performance.

To enable it, add Bevy's diagnostic plugin to your [app][cb::app]:

```rust,no_run,noplayground
{{#include ../code013/src/cookbook/print_framerate.rs:setup}}
```

## Print to Console / Log

The simplest way to use it is to print the diagnostics to the console
([log][cb::log]). If you want to only do it in dev builds, you can add
a conditional-compilation attribute.

```rust,no_run,noplayground
{{#include ../code013/src/cookbook/print_framerate.rs:log}}
```

## In-Game / On-Screen FPS counter

UPDATE! I have now released a Bevy plugin which provides a much better
version of the code on this page, ready for you to use! Consider trying
my [`iyes_perf_ui`][project::iyes_perf_ui] plugin!

Bevy maintainers have expressed interest in upstreaming it, and we will
try to make it official in the next Bevy release (0.14)!

For now, I am also keeping the old code example below in the book, for
completeness:

---

You can use Bevy UI to create an in-game FPS counter.

It is recommended that you create a new UI root (entity without
a parent) with absolute positioning, so that you can control the
exact position where the FPS counter appears, and so it doesn't
affect the rest of your UI.

Here is some example code showing you how to make a very nice-looking and
readable FPS counter:

<details>
  <summary>
  <code>Code Example (Long):</code>
  </summary>

```rust,no_run,noplayground
{{#include ../code013/src/cookbook/print_framerate.rs:ui}}
```

```rust,no_run,noplayground
{{#include ../code013/src/cookbook/print_framerate.rs:ui-app}}
```

</details>
