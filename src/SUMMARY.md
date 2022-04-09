# Summary

[Introduction](./introduction.md)

---

[List of Bevy Builtins](./builtins.md)

---

- [Bevy Setup Tips](./setup.md)
  - [Using bleeding-edge Bevy (main)](./setup/bevy-git.md)
  - [Text Editor / IDE](./setup/editor.md)
  - [Dev Tools and Editors for Bevy](./setup/bevy-tools.md)
  - [Community Plugin Ecosystem](./setup/unofficial-plugins.md)
  - [Customizing Bevy (features, modularity)](./setup/bevy-config.md)

- [Common Pitfalls](./pitfalls.md)
  - [Strange compile errors from Bevy or dependencies](./pitfalls/build-errors.md)
  - [Slow Performance](./pitfalls/performance.md)
  - [Error adding function as system](./pitfalls/into-system.md)
  - [UI is not displaying](./pitfalls/ui-camera.md)
  - [2D objects not displaying](./pitfalls/2d-camera-z.md)
  - [3D objects not displaying](./pitfalls/3d-not-rendering.md)
  - [Borrow multiple fields from struct](./pitfalls/split-borrows.md)
  - [Jittering Time (choppy movement/animation)](./pitfalls/time.md)
  - [UI layout is inverted](./pitfalls/ui-y-up.md)
  - [Textures/Images are flipped](./pitfalls/uv-coordinates.md)

- [Bevy Game Engine Core](./features.md)
  - [Coordinate System](./features/coords.md)
  - [Transforms](./features/transforms.md)
  - [Time and Timers](./features/time.md)
  - [Parent/Child Hierarchies](./features/parent-child.md)
  - [Fixed Timestep](./features/fixed-timestep.md)
  - [Audio](./features/audio.md)

- [Bevy Asset Management](./assets.md)
  - [Handles](./assets/handles.md)
  - [Load Assets from Files](./assets/assetserver.md)
  - [Access the Asset Data](./assets/data.md)
  - [React to Changes with Asset Events](./assets/assetevent.md)
  - [Track Loading Progress](./assets/ready.md)
  - [Hot-Reloading Assets](./assets/hot-reload.md)

- [Input Handling](./input.md)
  - [Keyboard](./input/keyboard.md)
  - [Mouse](./input/mouse.md)
  - [Text / Character](./input/char.md)
  - [Gamepad (Controller, Joystick)](./input/gamepad.md)
  - [Touchscreen](./input/touch.md)
  - [Drag-and-Drop (Files)](./input/dnd.md)
  - [MIDI (Musical Instrument)](./input/midi.md)

- [Window Management](./window.md)
  - [[WIP] Window Properties](./window/props.md)
  - [Change the Background Color](./window/clear-color.md)
  - [Grab/Capture the Mouse Cursor](./window/mouse-grab.md)
  - [Set the Window Icon](./window/icon.md)

- [Bevy 2D](./2d.md)
  - [[WIP] 2D Camera Setup](./2d/camera.md)
  - [[WIP] Sprites and Atlases](./2d/sprites.md)

- [Bevy 3D](./3d.md)
  - [[WIP] 3D Camera Setup](./3d/camera.md)
  - [3D Models and Scenes (GLTF)](./3d/gltf.md)

- [Bevy Programming Framework](./programming.md)
  - [Intro to ECS](./programming/ecs-intro.md)
  - [Entities and Components](./programming/ec.md)
  - [Resources](./programming/res.md)
  - [Systems](./programming/systems.md)
  - [Queries](./programming/queries.md)
  - [Commands](./programming/commands.md)
  - [Events](./programming/events.md)
  - [App Builder (fn main)](./programming/app-builder.md)
  - [Quit the App](./programming/quit.md)
  - [Local Resources](./programming/local.md)
  - [Plugins](./programming/plugins.md)
  - [System Order of Execution](./programming/system-order.md)
  - [System Sets](./programming/system-sets.md)
  - [Change Detection](./programming/change-detection.md)
  - [States](./programming/states.md)
  - [Run Criteria](./programming/run-criteria.md)
  - [Labels](./programming/labels.md)
  - [Stages](./programming/stages.md)
  - [Removal Detection](./programming/removal-detection.md)
  - [Query Sets](./programming/query-sets.md)
  - [System Chaining](./programming/system-chaining.md)
  - [Direct ECS World Access](./programming/world.md)
  - [Exclusive Systems](./programming/exclusive.md)
  - [[WIP] Sub-Apps](./programming/sub-apps.md)
  - [Non-Send](./programming/non-send.md)
  - [Writing Tests for Systems](./programming/system-tests.md)

- [Programming Patterns](./patterns.md)
  - [Generic Systems](./patterns/generic-systems.md)
  - [Manual Event Clearing](./patterns/manual-event-clear.md)

- [[WIP] Bevy Render (GPU) Framework](./gpu.md)

- [Bevy Cookbook](./cookbook.md)
  - [Show Framerate in Console](./cookbook/print-framerate.md)
  - [Convert cursor to world coordinates](./cookbook/cursor2world.md)
  - [Custom Camera Projection](./cookbook/custom-projection.md)
  - [Pan+Orbit Camera](./cookbook/pan-orbit-camera.md)
  - [List All Resource Types](./cookbook/print-resources.md)

- [Bevy on Different Platforms](./platforms.md)
  - [Linux Desktop](./platforms/linux.md)
  - [macOS Desktop](./platforms/macos.md)
  - [Windows Desktop](./platforms/windows.md)
  - [Cross-Compilation](./setup/cross.md)
    - [From Linux to Windows](./setup/cross/linux-windows.md)
  - [Browser (WebAssembly)](./platforms/wasm.md)
    - [Panic Messages](./platforms/wasm/panic-console.md)
    - [Optimize for Size](./platforms/wasm/size-opt.md)
    - [Hosting on GitHub Pages](./platforms/wasm/gh-pages.md)

---

[Credits](./credits.md)

---

[Contribute to Bevy](./contributing-bevy.md)
[Contribute to this Book](./contributing.md)
