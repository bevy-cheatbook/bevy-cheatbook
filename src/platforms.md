Warning: this chapter has not been updated for Bevy 0.10 yet!

# Bevy on Different Platforms

{{#include ./include/links.md}}

This chapter is a collection of platform-specific information, about using
Bevy with different operating systems or environments.

Feel free to suggest things to add.

## Platform Support

Bevy aims to also make it easy to target different platforms, such as the
various desktop operating systems, web browsers (via WebAssembly), mobile
(Android and iOS), and game consoles. Your Bevy code can be the same for all
platforms, with differences only in the build process and environment setup.

However, that vision is not fully met yet. Currently, support for non-desktop
platforms is limited, and requires more complex configuration.

### Desktop

Bevy trivially works out-of-the-box on the three major desktop operating
systems: Linux, macOS, Windows. No special configuration is required.

See the following pages for specific tips/advice when developing for the
desktop platforms:
 - [Linux][platform::linux]
 - [macOS][platform::macos]
 - [Windows][platform::windows]

All features are fully supported on each of the above.

If you are working in Linux, [see here for how to build Windows EXEs for
your Windows users][cross::linux-windows].

### Web

Bevy works quite well on the [web (using WebAssembly)][platform::wasm],
but with some limitations.

Multithreading is not supported, so you will have limited performance and
possible audio glitches. Rendering is limited to the features of the WebGL2
API, meaning worse performance and limitations like only supporting a maximum
of 256 lights in 3D scenes.

For inspiration, check out the [entries in the first Bevy Game
Jam][bevy::jam-01]. Many of them have web builds you can play in your browser.

### Mobile

Apple iOS is well-supported and most features work well. There are developers
in the Bevy community that have successfully shipped Bevy-based apps to the
App Store.

Android support is still limited. Your app will build and run. You can get
started. However, some features may be missing or broken, and there are
a few major bugs.  The worst one is that Bevy cannot yet handle being put
into the background, and will crash when the user presses the home button
to leave the app.

Android support is actively being worked on. Please join the development
community on [Discord][bevy::discord] if you are interested.

Bevy has been known to have issues with emulator devices. It is recommended
you test your app on real hardware.

### Game Consoles

Unfortunately, due to NDA requirements, developing for consoles is inaccessible
to most community developers who work in the open, and Bevy support is still
mostly nonexistent.

At least one person is working on PlayStation 5 support. If you are interested,
join [Discord][bevy::discord] and ask around. Maybe you can find each other and
work together.

The Rust Programming Language aims to make Nintendo Switch a supported target,
but that work is in its early days and has not progressed enough to be useful
for Bevy yet. It should be possible to work on Nintendo Switch support in
the open, without NDAs, using emulators.
