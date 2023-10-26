{{#include ../include/header012.md}}

# macOS Desktop

If you have any additional macOS-specific knowledge,
please help improve this page!

Create Issues or PRs on [GitHub][project::cb].

---

[See here if you also want to build Windows EXEs from macOS][cross::macos-windows].

## Known Pitfalls

### Input Peculiarities

[Mouse wheel scrolling][input::mouse-wheel] behaves in a peculiar manner,
because macOS does "scroll acceleration" at the OS level. Other OSs, with
regular PC mice, provide `Line` scroll events with whole number values, where
1.0 corresponds to one step on the scroll wheel. macOS scales the value
depending on how fast the user is spinning the wheel. You do not get whole
numbers. They can range anywhere from tiny values <0.1 (for the starting event,
before the scroll speed ramps up), up to values as big as >10.0 (say, for a fast
flick of the wheel), per event.

macOS provides [special events for touchpad gestures][input::touchpad-gesture]
for zooming and rotation, which you can handle in Bevy.

Some keyboard keys have a somewhat-unintuitive mapping:
 - The Command (⌘) key is `KeyCode::{SuperLeft, SuperRight}`.
 - The Option (⌥) key is `KeyCode::{AltLeft, AltRight}`.

Other key codes have their intuitive names.

### Window Management Apps Compatability

Bevy apps can encounter performance issues (such as lag when dragging the window
around the screen) when window management apps like "Magnet" are used. This is a
bug in `winit` (the OS window management library that Bevy uses). This issue can
be tracked [here][winit::1737].

Until that bug is fixed, advise closing the window management apps, if
encountering performance issues.

## Creating an Application Bundle

When you build your Bevy project normally, cargo/Rust will produce a bare
executable file, similar to other operating systems. However, this is not how
"normal" macOS apps look and behave. You probably want to create a proper
native-feeling Mac app for distribution to your users.

You need to do this, to have your app play nicely with the Mac desktop GUI, such
as to have a nice icon appear in the dock.

macOS applications are typically stored on the filesystem as "bundles" – special
directories/folders that end in `.app`, that the OS displays to the user as one
item. macOS expects to find a special hieararchy of subfolders and files inside.

A minimal app bundle might have the following files:

 - `MyGame.app/Contents/MacOS/MyGame`: the actual executable file
 - `MyGame.app/Contents/MacOS/assets/`: your Bevy assets folder
 - `MyGame.app/Contents/Info.plist`: metadata (see below)
 - `MyGame.app/Contents/Resources/AppIcon.icns`: the app's icon

Only the executable file is technically mandatory. If you have nothing else, the
app will run, as long as the executable file name matches the app bundle file
name. You should, however, follow the below instructions, if you want to make a
proper nice Mac app. :)

### Executable File

The executable file produced by the Rust compiler (in the `target` directory) is
a single-architecture binary for your current development machine. You could
just copy this file into the app bundle, but then you will not support all Mac
hardware natively.

If you want to support both machines with Intel CPUs and with Apple Silicon
(Arm) CPUs, you need to compile for both of them, and then combine them into a
single executable using Apple's `lipo` tool.

First, make sure you have Rust toolchain support for both architectures installed:

```sh
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

Now, you can compile for both architectures:

```sh
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

Now, you can combine the two executables into one, for your app bundle.

```sh
lipo "target/x86_64-apple-darwin/release/my_game" \
     "target/aarch64-apple-darwin/release/my_game" \
     -create -output "MyGame.app/Contents/MacOS/MyGame"
```

Note: please ensure the Bevy `dynamic_linking` cargo feature is ***not*** enabled.

### Game Assets

Your Bevy `assets` folder needs to be placed alongside the executable file,
for Bevy to find it and be able to load your assets. Just copy it into
`Contents/MacOS` in your app bundle.

Note: This is not the standard conventional location as prescribed by Apple.
Typically, macOS apps store their data files in `Contents/Resources`. However,
Bevy will not find them there. Thankfully, Apple does not enforce this, so we
are free to do something unusual when we have to.

### `Info.plist`

This file contains all the metadata that macOS wants.

If you do not create this file, or if it is missing some of the fields, macOS
will try to guess them, so your app can still run. Ideally, you want to create a
proper `Info.plist` file, to prevent issues.

[Download an example file as a starting point.][cbdl::macos-info-plist]

You can edit this file using Apple XCode or a text editor. Check that all the
values make sense for your app. Pay special attention to these values:
 - `CFBundleName` (Bundle name)
   - Short user-visible name of your app
 - `CFBundleDisplayName` (Bundle display name)
   - Optional: You can set a longer user-visible name here, if you want
 - `CFBundleExecutable` (Executable file)
   - The name of the executable file
 - `CFIconFile` (Icon file)
   - The name of the icon file
 - `CFBundleIdentifier` (Bundle identifier)
   - Apple wants an ID for your app, in domain format, like: `com.mycompany.mygame`
 - `CFBundleShortVersionString` (Bundle version string (short))
   - The version of your app, like `0.1.0`.

### App Icon

The icon file needs to be in a special Apple format.

Such a file can be created from a collection of PNGs of different standard sizes
(powers of two). If you want your app to look nice at all sizes, you can
hand-craft an image for each size, following [Apple Design
Guidelines][apple::design-app-icon]. If you don't care, you can just take one image
(ideally 1024x1024, the biggest size used by macOS) and scale it to different sizes.

Here is a script that does that:

```sh
SOURCE_IMAGE="myicon1024.png"
mkdir -p AppIcon.iconset
sips -z 16 16     "${SOURCE_IMAGE}" --out AppIcon.iconset/icon_16x16.png
sips -z 32 32     "${SOURCE_IMAGE}" --out AppIcon.iconset/icon_16x16@2x.png
sips -z 32 32     "${SOURCE_IMAGE}" --out AppIcon.iconset/icon_32x32.png
sips -z 64 64     "${SOURCE_IMAGE}" --out AppIcon.iconset/icon_32x32@2x.png
sips -z 128 128   "${SOURCE_IMAGE}" --out AppIcon.iconset/icon_128x128.png
sips -z 256 256   "${SOURCE_IMAGE}" --out AppIcon.iconset/icon_128x128@2x.png
sips -z 256 256   "${SOURCE_IMAGE}" --out AppIcon.iconset/icon_256x256.png
sips -z 512 512   "${SOURCE_IMAGE}" --out AppIcon.iconset/icon_256x256@2x.png
sips -z 512 512   "${SOURCE_IMAGE}" --out AppIcon.iconset/icon_512x512.png
cp "${SOURCE_IMAGE}" AppIcon.iconset/icon_512x512@2x.png
iconutil -c icns AppIcon.iconset
## move it into the app bundle
mv AppIcon.icns MyGame.app/Contents/Resources
```

It works by creating a special `iconset` folder, with all the PNG files at different
sizes, created by resizing your source image. Then, it uses `iconutil` to produce
the final Apple ICNS file for your app bundle.

If you want hand-crafted icons for each size, you could use a similar process.
Create an `iconset` folder with your PNGs, and run `iconutil -c icns` on it.

Alternatively, Apple XCode has GUI tools for creating and editing app icons.

### Putting Everything Together

Here is a simple shell script to build a Mac app. It follows the recommendations
on this page. Adjust everything as necessary for your project.

```sh
# set the name of the Mac App
APP_NAME="MyGame"
# set the name of your rust crate
RUST_CRATE_NAME="my_game"
# create the folder structure
mkdir -p "${APP_NAME}.app/Contents/MacOS"
mkdir -p "${APP_NAME}.app/Contents/Resources"
# copy Info.plist
cp Info.plist "${APP_NAME}.app/Contents/Info.plist"
# copy the icon (assuming you already have it in Apple ICNS format)
cp AppIcon.icns "${APP_NAME}.app/Contents/Resources/AppIcon.icns"
# copy your Bevy game assets
cp -a assets "${APP_NAME}.app/Contents/MacOS/"
# compile the executables for each architecture
cargo build --release --target x86_64-apple-darwin # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon
# combine the executables into a single file and put it in the bundle
lipo "target/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "target/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "${APP_NAME}.app/Contents/MacOS/${APP_NAME}"
```

Note: please ensure the Bevy `dynamic_linking` cargo feature is ***not*** enabled.

## Creating a DMG file

It is common for Mac apps downloadable from the internet to be distributed as
DMG files – Apple's "disk image" format. Users can drag-and-drop the app bundle
inside into their `Applications` folder on their system.

### `create-dmg`

If you want to create a fancy DMG file, you can install and use the
[`create-dmg` tool][project::create-dmg].

If you are using Homebrew, you can install it easily from there:

```sh
brew install create-dmg
```

Then, you can use it as follows:

```sh
create-dmg \
  --volname "My Bevy Game" \
  --volicon "AppIcon.icns" \
  --background "DMG-background.png" \
  --window-size 800 400 \
  --icon-size 128 \
  --icon "MyGame.app" 200 200 \
  --hide-extension "MyGame.app" \
  --app-drop-link 600 200 \
  "mybevygame_release_mac.dmg" \
  "build/mac/"
```

The options are:
 - `--volname`: the name of the device when the user opens the DMG file
 - `--volicon`: the icon of the device when the user opens the DMG file
 - `--background`: the background image for the Finder window
 - `--window-size`: the size of the Finder window
 - `--icon-size`: the default zoom level (how big the icons should look)
 - `--icon`: specify the X/Y coordinates where to display a specific file
 - `--hide-extension`: do not display the file extension for this file
 - `--app-drop-link`: create a shortcut to Applications for easy drag-and-drop; place at given X/Y coordinates
 - the name of the DMG file to create
 - the name of the folder where you have the files to be added to the DMG (your app + anything else you want to add)

### `hdiutil`

If you don't want to install any special tools, you can create a very simple
DMG file using `hdiutil`, which comes with macOS:

```sh
hdiutil create -fs HFS+ \
  -volname "My Bevy Game" \
  -srcfolder "MyGame.app" \
  "mybevygame_release_mac.dmg"
```

Specify the Volume Name (how it appears when opened), the name of your app
bundle, and the name of the output DMG file, respectively. You can use
`-srcfolder` multiple times, if you want to add more files and folders to the
DMG image.

### GUI

If you want to create a DMG file using a GUI, you can use Apple's "Disk
Utility" app that comes preinstalled with macOS. Then, just use Finder to
set up everything inside how you like it.
