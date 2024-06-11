# Summary

[Introduction](./introduction.md)
[Chapter Overview](./overview.md)

---

[List of Bevy Builtins](./builtins.md)

---

- [Bevy Tutorials](./tutorial.md)
  - [Guided Tour](./tutorial/guide.md)

- [Bevy Cookbook](./cookbook.md)
  - [Show Framerate](./cookbook/print-framerate.md)
  - [Convert cursor to world coordinates](./cookbook/cursor2world.md)
  - [Custom Camera Projection](./cookbook/custom-projection.md)
  - [3D Pan+Orbit Camera](./cookbook/pan-orbit-camera.md)
  - [List All Resource Types](./cookbook/print-resources.md)

---

- [Bevy Setup Tips](./setup.md)
  - [Getting Started](./setup/getting-started.md)
  - [Text Editor / IDE](./setup/editor.md)
    - [Visual Studio Code](./setup/editor/vscode.md)
    - [JetBrains (RustRover, IntelliJ, CLion)](./setup/editor/jetbrains.md)
    - [Kakoune](./setup/editor/kak.md)
    - [Vim](./setup/editor/vim.md)
    - [Emacs](./setup/editor/emacs.md)
  - [Customizing Bevy (features, modularity)](./setup/bevy-config.md)
  - [Community Plugin Ecosystem](./setup/unofficial-plugins.md)
  - [Dev Tools and Editors for Bevy](./setup/bevy-tools.md)
  - [Performance Tunables](./setup/perf.md)
  - [Using bleeding-edge Bevy (main)](./setup/bevy-git.md)

- [Common Pitfalls](./pitfalls.md)
  - [Strange compile errors from Bevy or dependencies](./pitfalls/build-errors.md)
  - [Slow Performance](./pitfalls/performance.md)
  - [Error adding function as system](./pitfalls/into-system.md)
  - [3D objects not displaying](./pitfalls/3d-not-rendering.md)
  - [Borrow multiple fields from struct](./pitfalls/split-borrows.md)
  - [Jittering Time (choppy movement/animation)](./pitfalls/time.md)
  - [Textures/Images are flipped](./pitfalls/uv-coordinates.md)

- [Game Engine Fundamentals](./fundamentals.md)
  - [Coordinate System](./fundamentals/coords.md)
  - [Transforms](./fundamentals/transforms.md)
  - [Visibility](./fundamentals/visibility.md)
  - [Time and Timers](./fundamentals/time.md)
  - [Logging, Console Messages](./fundamentals/log.md)
  - [Parent/Child Hierarchies](./fundamentals/hierarchy.md)
  - [Fixed Timestep](./fundamentals/fixed-timestep.md)
  - [Gizmos](./fundamentals/gizmos.md)

- [General Graphics Features](./graphics.md)
  - [Cameras](./graphics/camera.md)
  - [HDR and Tonemapping](./graphics/hdr-tonemap.md)
  - [Bloom](./graphics/bloom.md)

- [Working with 2D](./2d.md)
  - [2D Camera Setup](./2d/camera.md)
  - [Sprites and Atlases](./2d/sprites.md)

- [Working with 3D](./3d.md)
  - [3D Camera Setup](./3d/camera.md)
  - [3D Models and Scenes (GLTF)](./3d/gltf.md)

- [Input Handling](./input.md)
  - [Keyboard](./input/keyboard.md)
  - [Mouse](./input/mouse.md)
  - [Gamepad (Controller, Joystick)](./input/gamepad.md)
  - [Touchscreen](./input/touch.md)
  - [Gestures](./input/gesture.md)
  - [Drag-and-Drop (Files)](./input/dnd.md)
  - [IME (Advanced Text)](./input/ime.md)

- [Window Management](./window.md)
  - [Window Properties](./window/props.md)
  - [Change the Background Color](./window/clear-color.md)
  - [Grab/Capture the Mouse Cursor](./window/mouse-grab.md)
  - [Set the Window Icon](./window/icon.md)

- [Asset Management](./assets.md)
  - [Handles](./assets/handles.md)
  - [Load Assets from Files](./assets/assetserver.md)
  - [Access the Asset Data](./assets/data.md)
  - [React to Changes with Asset Events](./assets/assetevent.md)
  - [Track Loading Progress](./assets/ready.md)
  - [Hot-Reloading Assets](./assets/hot-reload.md)
  - [Processing Assets](./assets/processing.md)

- [Audio](./audio.md)
  - [Playing Sounds](./audio/basic.md)
  - [Spatial Audio](./audio/spatial.md)
  - [Custom Audio Streams](./audio/custom.md)

- [Bevy UI Framework](./ui.md)

- [Bevy Core Programming Framework](./programming.md)
  - [Intro to ECS](./programming/ecs-intro.md)
  - [Intro: Your Data](./programming/intro-data.md)
  - [Intro: Your Code](./programming/intro-code.md)
  - [The App](./programming/app-builder.md)
  - [Systems](./programming/systems.md)
  - [Resources](./programming/res.md)
  - [Entities, Components](./programming/ec.md)
  - [Bundles](./programming/bundle.md)
  - [Queries](./programming/queries.md)
  - [Commands](./programming/commands.md)
  - [Events](./programming/events.md)
  - [Plugins](./programming/plugins.md)
  - [Local Resources](./programming/local.md)
  - [Exclusive Systems](./programming/exclusive.md)
  - [Direct ECS World Access](./programming/world.md)
  - [Schedules](./programming/schedules.md)
  - [System Order of Execution](./programming/system-order.md)
  - [Run Conditions](./programming/run-conditions.md)
  - [System Sets](./programming/system-sets.md)
  - [States](./programming/states.md)
  - [Change Detection](./programming/change-detection.md)
  - [One-Shot Systems](./programming/one-shot-systems.md)
  - [System Piping](./programming/system-piping.md)
  - [ParamSet](./programming/paramset.md)
  - [Non-Send](./programming/non-send.md)

- [Bevy Render (GPU) Framework](./gpu.md)
  - [Render Architecture Overview](./gpu/intro.md)
  - [Render Stages](./gpu/stages.md)

- [Programming Patterns](./patterns.md)
  - [Generic Systems](./patterns/generic-systems.md)
  - [Component Storage (Table/Sparse-Set)](./patterns/component-storage.md)
  - [Manual Event Clearing](./patterns/manual-event-clear.md)
  - [Writing Tests for Systems](./patterns/system-tests.md)

- [Bevy on Different Platforms](./platforms.md)
  - [Linux Desktop](./platforms/linux.md)
  - [macOS Desktop](./platforms/macos.md)
  - [Windows Desktop](./platforms/windows.md)
    - [Working in WSL2](./platforms/windows/wsl2.md)
  - [Browser (WebAssembly)](./platforms/wasm.md)
    - [Optimize for Size](./platforms/wasm/size-opt.md)
    - [Create a Custom Web Page](./platforms/wasm/webpage.md)
    - [Hosting on GitHub Pages](./platforms/wasm/gh-pages.md)
  - [Cross-Compilation](./setup/cross.md)
    - [From Linux to Windows](./setup/cross/linux-windows.md)
    - [From macOS to Windows](./setup/cross/macos-windows.md)

---

[Credits](./credits.md)

---

[Contact Me](./contact.md)

---

[Contribute to Bevy](./contributing-bevy.md)
[Contribute to this Book](./contributing.md)
