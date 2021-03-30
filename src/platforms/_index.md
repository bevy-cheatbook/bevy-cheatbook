# Bevy on Different Platforms

Bevy trivially works out-of-the-box on the major desktop operating systems: Linux, macOS, Windows.

However, Bevy aims to also make it easy to target other platforms, such as web
browsers (via WebAssembly), mobile (Android and iOS), and game consoles.

Bevy makes it easy to be multi-platform. Your Bevy code can be the same for all
platforms; the differences are only in the build process and environment setup.

Currently, support for these non-desktop platforms is limited:

 - Most Bevy features work OK on the Web. You can make a browser game using
   Bevy, albeit with limited performance (compared to a native desktop version),
   due to lack of multithreading, and some other possible caveats.
 - Mobile support is minimal. It's not hard to get something to run, but it will
   be broken in various major ways.
 - Game console support is still completely non-existent yet.
 
If you are interested in these other platforms and you'd like to help improve
Bevy's cross-platform support, your contributions would be greatly welcomed!

---

This section of the book is planned to be expanded into a full chapter soon!

It will cover the relevant topics and considerations to help you use Bevy with
different non-desktop platforms.
