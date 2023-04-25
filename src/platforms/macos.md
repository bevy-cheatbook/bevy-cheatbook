{{#include ../include/header09.md}}

# macOS Desktop

If you have any additional macOS-specific knowledge,
please help improve this page!

Create Issues or PRs on [GitHub][project::cb].

---

## Known Pitfalls

### Window Management Apps Compatability

Bevy apps can encounter performance issues (such as lag when dragging the window
around the screen) when window management apps like "Magnet" are used. This is a
bug in `winit` (the OS window management library that Bevy uses). This issue can
be tracked [here][winit::1737].

Until that bug is fixed, advise closing the window management apps, if
encountering performance issues.
