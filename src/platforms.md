{{#include ./include/header012.md}}

# Bevy on Different Platforms

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

All Bevy features are fully supported on each of the above.

You can also build Windows EXEs for your Windows users, if you are working
in [Linux][cross::linux-windows] or [macOS][cross::macos-windows].

### Web

Bevy works quite well on the [web (using WebAssembly)][platform::wasm],
but with some limitations.

Multithreading is not supported, so you will have limited performance and
possible audio glitches. Rendering is limited to the features of the WebGL2
API, meaning worse performance and limitations like only supporting a maximum
of 256 lights in 3D scenes. These limitations can be lifted by enabling the
new WebGPU support, but then you will have limited browser compatibility.

For inspiration, check out the entries in the Bevy Game Jams
([third][bevy::jam-03], [second][bevy::jam-02], [first][bevy::jam-01]). Many
of them have web builds you can play in your browser.

### Mobile

Apple iOS is well-supported and most features work well. There are developers
in the Bevy community that have successfully shipped Bevy-based apps to the
App Store.

Android support is not as good as iOS, but very usable (as of Bevy 0.12). If
you find bugs, broken features, or other issues, please report them.

Bevy has been known to have issues with emulator devices. It is recommended
you test your app on real hardware.

### Game Consoles

Unfortunately, due to NDA requirements, developing for consoles is inaccessible
to most community developers who work in the open, and Bevy support is still
mostly nonexistent.

At some point, there was someone in the community working on PlayStation
support. I do not know if they are still around, or anything about the
status of that work. If you are interested, join [Discord][bevy::discord]
and ask around. Maybe you can find each other and work together.

The Rust Programming Language aims to make Nintendo Switch a supported target,
but that work is in its early days and has not progressed enough to be useful
for Bevy yet. It should be possible to work on Nintendo Switch support in
the open, without NDAs, using emulators.

The Steam Deck, and other such "handheld PCs", are well supported. Such
devices run special versions of standard Desktop OSs (Linux, Windows) and are
designed to support PC games out of the box. To develop for these devices,
just make regular Linux/Windows builds of your game and ideally try them on
an actual device, so you can see how the handheld experience is like and make
sure your game feels good on such a device.
