Warning: this page has not been updated for Bevy 0.10 yet!

# Community Plugins Ecosystem

{{#include ../include/links.md}}

There is a growing ecosystem of unofficial community-made plugins for Bevy.
They provide a lot of functionality that is not officially included with the
engine. You might greatly benefit from using some of these in your projects.

To find such plugins, you should search the [Bevy Assets][bevyassets]
page on the official Bevy website. This is the official registry of known
community-made things for Bevy. If you publish your own plugins for Bevy,
you should [contribute a link to be added to that page][project::bevyassets].

Beware that some 3rd-party plugins may use unusual licenses! Be sure to
check the license before using a plugin in your project.

---

Other pages in this book with valuable information when using 3rd-party plugins:

  - Some plugins may require you to [configure Bevy in some specific way][cb::features].
  - If you are [using bleeding-edge unreleased Bevy (main)][cb::git], you may encounter difficulties with plugin compatibility.

## Plugin Recommendations

This here is my personal, curated, opinionated list of recommendations,
featuring the most important plugins (in my opinion) in the Bevy ecosystem.

My goal here is to help direct new Bevy users to some known-good resources,
so you can start working on the kinds of games you want to make. :)

The plugins listed here use permissive licenses (like Bevy itself).

This page is limited. I can only recommend plugins I know enough about. Please
also check the [Bevy Assets][bevyassets] page to find even more things. :)

### Development Tools and Editors

[These are listed on a separate page.][cb::tools]

### Code Helpers

[`bevy_asset_loader`][project::bevy_asset_loader] is a more flexible and
opinionated helper for managing and loading [assets][cb::asset]. Uses custom
syntax to let you declare your assets more conveniently. It can track loading
progress and perform a state transition for you.

[`bevy_debug_lines`][project::bevy_debug_lines] allows you to quickly draw
lines on the screen, to help you visualize things during development.

[`iyes_loopless`][project::iyes_loopless] provides alternative improved
implementations of [states][cb::state], [run criteria][cb::runcriteria],
and [fixed timestep][cb::fixedtimestep], that do not suffer from the major
usability limitations of the ones provided with Bevy.

[`iyes_progress`][project::iyes_progress] is a general helper for tracking
progress and performing a state transition when everything is ready. A common
use case are loading screens, where you might want to do more than just load
assets, like world generation, connecting to a server, …

### Input Mappings

To help with your game's [input handling][chapter::input] needs, try the
[Input Manager plugin by Leafwing Studios][project::lwim]. It is a very
flexible way to handle your game's bindings / mappings.

### Audio

Use [`bevy_kira_audio`][project::bevy_kira_audio] instead of the built-in `bevy_audio`.

The built-in audio is very limited in features, and you are likely going
to need this plugin for pretty much any game with audio needs beyond simply
playing sounds with volume control. Kira provides advanced mixing, playback
controls, some simple effects, streaming functionality…

See [this page][cb::audio] for help on how to set it up.

### Camera

[`smooth-bevy-cameras`][project::smooth-bevy-cameras] is a nice plugin for easily
adding camera controls to your Bevy 3D project.

Cameras are something that can be very game-specific. As you progress with your
project, you would probably want to implement your own custom camera control
logic for your game. However, this plugin can be useful when you are starting
out on a new project.

## Graphics

If you need particle effects, try [`bevy_hanabi`][project::bevy_hanabi].

Experimental 3D Path Traced Global Illumination: [`bevy_hikari`][project::bevy_hikari].

### Tilemap

If you are making a 2D game based on a tile-map, there are plugins to help do it
efficiently with high performance. It is much better than just spawning lots of
individual Bevy sprites for each tile.

[`bevy_ecs_tilemap`][project::bevy_ecs_tilemap]:
  - Uses one ECS Entity per tile, lets you work with the tilemap in an ECS-idiomatic way.
  - Efficient rendering, using techniques like texture arrays, chunks, morton encoding, …
  - Lots of features: Square/Hexagon/Isometric grids, animation, layers, chunks, …

### 2D Maps / Levels

[`bevy_ecs_ldtk`][project::bevy_ecs_ldtk] implements loading of entire
maps/levels created with the [LDTK editor][project::ldtk], into Bevy. Based on
`bevy_ecs_tilemap` internally, for efficient performance.

### Shapes / Vector Graphics / Canvas

If you want to draw 2D shapes, use the [`bevy_prototype_lyon`][project::bevy_prototype_lyon] plugin.

For SDFs (signed-distance-functions), see [`bevy_smud`][project::bevy_smud].

### Game AI

[`big-brain`][project::big-brain] is a plugin for game AI behaviors (Utility AI).

### GUI

There are a few alternatives to Bevy UI available.

[`bevy_egui`][project::bevy_egui] integrates the [`egui`
toolkit][project::egui] into Bevy. It is a mature immediate-mode GUI library
(like the popular Dear Imgui, but in Rust). It is very feature-rich and
provides lots of widgets. It was not really designed for making flashy
gamey UIs (though it may very well be fine for your game). It's great for
editor-like UIs, debug UIs, or non-game applications.

[`kayak_ui`][project::kayak_ui] is a new experimental game-centric UI library
for Bevy, which uses a XML-like declarative syntax for constructing UIs.

#### UI Navigation

If you are using the builtin Bevy UI, there is a nice plugin available
for navigation (moving between buttons and other focusable UI elements):
[`bevy-ui-navigation`][project::bevy-ui-navigation].

### Physics

Bevy can integrate with the [Rapier physics engine][project::rapier].

Use the [`bevy_rapier`][project::bevy_rapier] plugin. It provides a nice API to
bridge between Bevy and Rapier.

### Animation

Starting from Bevy 0.7, there is built-in support for playing predefined
asset-driven animations, including 3D skeletal animation.

However, for "programmatic" / code-driven animation, you may need something
else. Try [`bevy_tweening`][project::bevy_tweening]. This might be good
enough for moving objects around, moving the camera, smoothly changing colors,
or other such transitions.

For animated 2D sprites, try [`benimator`][project::benimator]. This is
for using sprite-sheet assets with many frames of animation.

### Selecting 3D Objects

[`bevy_mod_picking`][project::bevy_mod_picking] is a plugin for selecting
3D meshes using pointer inputs like the mouse. Use this if you need to let
the user click on things in your 3D world.

### File Formats

Additional asset loaders, for loading assets from file formats other than
[those that Bevy officially supports][builtins::file-formats].

 - Wavefront OBJ 3D models: [`bevy_obj`][project::bevy_obj]
 - STL 3D models: [`bevy_stl`][project::bevy_stl]
 - MagicaVoxel VOX: [`bevy_vox`][project::bevy_vox]

[`bevy_common_assets`][project::bevy_common_assets] allows you to easily
create custom asset types to be loaded from general structured data formats
like RON, JSON, YAML, TOML, and MessagePack. Useful for representing data
specific to your game (like item/weapon/enemy/etc parameters") as asset files.

### Distribution

Plugins to help with releasing your game.

[`bevy_embedded_assets`][project::bevy_embedded_assets] allows bundling
your asset files inside of the game executable, instead of having a separate
`assets` folder.

[`bevy_steamworks`][project::bevy_steamworks] allows you to integrate with
the Steam SDK by Valve Software, if you intend to publish on that platform.

[`bevy_discord_presence`][project::bevy_discord_presence] allows you to
integrate with the Discord social platform.
