# macOS Desktop

{{#include ../include/links.md}}

If you have any additional macOS-specific knowledge,
please help improve this page!

Create Issues or PRs on [GitHub][project::cb].

---

## Known Pitfalls

### Window Management Apps Compatability

Bevy applications making use of `winit` encounter performance issues where the window is slow to drag around the screen when apps like `Magnet` are used to orginize the user's workspace. Closing the workspace manager apps should significantly ehance the performance of app.

This issue can be tracked here: https://github.com/rust-windowing/winit/issues/1737
