# Community Plugins Ecosystem

{{#include ../include/links.md}}

There is a growing ecosystem of unofficial community-made plugins for Bevy.
They provide a lot of functionality that is not officially included with the
engine. You might greatly benefit from using some of these in your projects.

To find such plugins, you should search the [Bevy Assets][bevyassets]
page on the official Bevy website. This is the official registry of known
community-made things for Bevy. If you publish your own plugins for Bevy,
you should [contribute a link to be added to that page][project::bevyassets].

Please beware that some 3rd-party plugins may use unusual licenses. Be sure
to check the license before using a plugin in your project.

---

Other pages in this book with valuable information when using 3rd-party plugins:

  - Some plugins may require you to [configure Bevy in some specific way][cb::features].
  - If you are [using bleeding-edge unreleased Bevy (main)][cb::git], you may encounter difficulties with plugin compatibility.

## Plugin Recommendations

This here is my personal, curated, opinionated list of recommendations,
featuring the most important plugins (in my opinion) in the Bevy ecosystem.

My goal here is to help direct new Bevy users to some known-good resources,
so you can start working on the kinds of games you want to make. :)

The plugins listed here are compatible with the current Bevy release and use
permissive licenses (like Bevy itself).

This page is limited. I can only recommend plugins I know enough about. Please
also check the [Bevy Assets][bevyassets] page to find even more things. :)

### Development Tools and Editors

[These are listed on a separate page.][cb::tools]

### Code Helpers

[`bevy_asset_loader`][project::bevy_asset_loader] is a more flexible and
opinionated helper for managing and loading [assets][cb::assets]. Uses custom
syntax to let you declare your assets more conveniently.

[`iyes_loopless`][project::iyes_loopless] provides alternative improved
implementations of [states][cb::state], [run criteria][cb::runcriteria],
and [fixed timestep][cb::fixedtimestep], that do not suffer from the major
usability limitations of the ones provided with Bevy.

### Input Mappings

To help with your game's [input handling][chapter::input] needs, try the
[Input Manager plugin by Leafwing Studios][project::lwim]. It is a very
flexible way to handle your game's bindings / mappings.

### Audio

Use [`bevy_kira_audio`][project::bevy_kira_audio] instead of the built-in `bevy_audio`.

The built-in audio is very limited in features, and you are likely going to
need this plugin for pretty much any game with audio.

See [this page][cb::audio] for help on how to set it up.

### Camera

[`bevy_config_cam`][project::bevy_config_cam] is a nice plugin for easily
adding camera controls to your Bevy 3D project. It gives you a a choice
of various common camera behaviors (like follow, top-view, FPS-style,
free-roaming).

Cameras are something that can be very game-specific. As you progress with
your project, you would probably want to implement your own custom camera
control logic for your game. However, this plugin is amazing when you are
starting out on a new project.

### Tilemap

If you are making a 2D game based on a tile-map, there are plugins to
help do it efficiently with high performance. It is better to use one
of these plugins, instead of just spawning lots of individual Bevy
sprites for each tile.

[`bevy_ecs_tilemap`][project::bevy_ecs_tilemap]:
  - Uses one ECS Entity per tile, lets you work with the tilemap in an ECS-idiomatic way.
  - Very efficient rendering, using techniques like texture arrays, chunks, morton encoding, …
  - Lots of features: Square/Hexagon/Isometric grids, animation, layers, chunks, …

[`bevy_ecs_ldtk`][project::bevy_ecs_ldtk] implements loading of
entire maps/levels created with the LDTK editor, into Bevy. Based on
`bevy_ecs_tilemap` internally, for efficient performance.

### Shapes / Vector Graphics / Canvas

If you want to draw 2D shapes, use the [`bevy_prototype_lyon`][project::bevy_prototype_lyon] plugin.

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

[`ui4`][project::ui4] is another notable experimental UI library for Bevy.

#### UI Navigation

If you are using the builtin Bevy UI, there is a nice plugin available
for navigation (moving between buttons and other focusable UI elements):
[`bevy-ui-navigation`][project::bevy-ui-navigation].

### Physics

Bevy can integrate with the [Rapier physics engine][project::rapier].

There are two plugins you can choose from:

  - [`heron`][project::heron]
    - Idiomatic to Bevy. Nice user-friendly integration and workflow.
    - Likely to be easier to use and more intuitive than `bevy_rapier`.
    - May have more limited functionality.
  - [`bevy_rapier`][project::bevy_rapier]
    - This is a "raw" plugin that gives you direct access to Rapier.
    - Gives you the most control, but may be hard to use and not idiomatic-Bevy.
    - You will probably need to read a lot of documentation, harder to learn.

### Animation

Starting from Bevy 0.7, there is built-in support for playing predefined
asset-driven animations, including 3D skeletal animation.

However, for "programmatic" / code-driven animation, you may need something
else. Try [`bevy_tweening`][project::bevy_tweening]. This might be good
enough for moving objects around, moving the camera, smoothly changing colors,
or other such transitions.

For animated 2D sprites, try [`benimator`][project::benimator]. This is
for using sprite-sheet assets with many frames of animation.

### File Formats

Additional asset loaders, for loading assets from file formats other than
[those that Bevy officially supports][builtins::file-formats].

 - Wavefront OBJ 3D models: [`bevy_obj`][project::bevy_obj]
 - STL 3D models: [`bevy_stl`][project::bevy_stl]
 - MagicaVoxel VOX: [`bevy_vox`][project::bevy_vox]
