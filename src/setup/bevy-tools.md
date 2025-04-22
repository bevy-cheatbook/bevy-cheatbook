{{#include ../include/header016.md}}

# Dev Tools and Editors for Bevy

Bevy does not yet have an official editor or other such tools. An official
editor is planned as a long-term future goal. In the meantime, here are
some community-made tools to help you.

---

## Editor

You can use [Blender][project::blender] as a level/scene
editor, by exporting your scenes to [GLTF][cb::gltf]. The
[Blenvy][project::blender_bevy_components_workflow] project provides
a nice experience / workflow, by allowing you to setup your Bevy ECS
[Components][cb::component] in Blender, include them in the exported GLTF,
and use them in Bevy.

[`bevy_inspector_egui`][project::bevy_inspector_egui] gives you a simple
editor-like property inspector window in-game. It lets you modify the values of
your components and resources in real-time as the game is running.

There is a repo for WIP development of Bevy's official editor:
[`bevy_editor_prototypes`][project::bevy_editor_prototypes]. Not really ready for use yet.

## Diagnostics

[`bevy_mod_debugdump`][project::bevy_mod_debugdump] is a tool to help visualize
your [App Schedules](../programming/app-builder.md) (all of the registered
[systems](../programming/systems.md) with their [ordering
dependencies](../programming/system-order.md)), and the Bevy [Render Graph][cb::render::graph].
