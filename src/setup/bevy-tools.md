{{#include ../include/header011.md}}

# Dev Tools and Editors for Bevy

Bevy does not yet have an official editor or other such tools. An official
editor is planned as a long-term future goal. In the meantime, here are
some community-made tools to help you.

---

## Editor

[`bevy_inspector_egui`][project::bevy_inspector_egui] gives you a simple
editor-like property inspector window in-game. It lets you modify the values of
your components and resources in real-time as the game is running.

[`bevy_editor_pls`][project::bevy_editor_pls] is an editor-like interface that
you can embed into your game. It has even more features, like switching app
states, fly camera, performance diagnostics, and inspector panels.

[`space_editor`][project::space_editor] is another such editor that can be
embedded into your game. It seems to be designed for a Unity-inspired prefab
workflow.

You can also use [Blender][project::blender] as a level/scene editor,
by exporting your scenes to [GLTF][cb::gltf]. The [Blender Bevy Components
Workflow][project::blender_bevy_components_workflow] project improves on this
experience, by allowing you to setup your Bevy ECS [Components][cb::component]
in Blender, include them in the exported GLTF, and use them in Bevy.

## Diagnostics

[`bevy_mod_debugdump`][project::bevy_mod_debugdump] is a tool to help visualize
your [App Schedules](../programming/app-builder.md) (all of the registered
[systems](../programming/systems.md) with their [ordering
dependencies](../programming/system-order.md)), and the Bevy Render Graph.
