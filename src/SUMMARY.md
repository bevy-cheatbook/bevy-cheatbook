# Summary

[Introduction](./introduction.md)
[Chapter Overview](./overview.md)

---

[List of Bevy Builtins](./builtins.md)

---

- [Bevy Tutorials](./tutorial.md)
  - [Guided Tour](./tutorial/guide.md)

- [Bevy Setup Tips](./setup.md)
  - [Getting Started](./setup/getting-started.md)
  - [Using bleeding-edge Bevy (main)](./setup/bevy-git.md)
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

- [Common Pitfalls](./pitfalls.md)
  - [Strange compile errors from Bevy or dependencies](./pitfalls/build-errors.md)
  - [Slow Performance](./pitfalls/performance.md)
  - [Error adding function as system](./pitfalls/into-system.md)
  - [3D objects not displaying](./pitfalls/3d-not-rendering.md)
  - [Borrow multiple fields from struct](./pitfalls/split-borrows.md)
  - [Jittering Time (choppy movement/animation)](./pitfalls/time.md)
  - [Textures/Images are flipped](./pitfalls/uv-coordinates.md)

- [General Game Engine Features](./features.md)
  - [Coordinate System](./features/coords.md)
  - [Transforms](./features/transforms.md)
  - [Visibility](./features/visibility.md)
  - [Cameras](./features/camera.md)
  - [HDR, Tonemapping, Bloom](./features/hdr-tonemap.md)
  - [Time and Timers](./features/time.md)
  - [Logging, Console Messages](./features/log.md)
  - [Parent/Child Hierarchies](./features/parent-child.md)
  - [Fixed Timestep](./features/fixed-timestep.md)
  - [Audio](./features/audio.md)

- [Asset Management](./assets.md)
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

- [Window Management](./window.md)
  - [Window Properties](./window/props.md)
  - [Change the Background Color](./window/clear-color.md)
  - [Grab/Capture the Mouse Cursor](./window/mouse-grab.md)
  - [Set the Window Icon](./window/icon.md)

- [Working with 2D](./2d.md)
  - [2D Camera Setup](./2d/camera.md)
  - [Sprites and Atlases](./2d/sprites.md)

- [Working with 3D](./3d.md)
  - [3D Camera Setup](./3d/camera.md)
  - [3D Models and Scenes (GLTF)](./3d/gltf.md)

- [Bevy Programming Framework](./programming.md)
  - [Intro to ECS](./programming/ecs-intro.md)
  - [Intro: Your Data](./programming/intro-data.md)
  - [Intro: Your Code](./programming/intro-code.md)
  - [The App](./programming/app-builder.md)
  - [Systems](./programming/systems.md)
  - [Resources](./programming/res.md)
  - [Entities, Components, Bundles](./programming/ec.md)
  - [Bundles](./programming/bundle.md)
  - [Queries](./programming/queries.md)
  - [Commands](./programming/commands.md)
  - [Events](./programming/events.md)
  - [Local Resources](./programming/local.md)
  - [Exclusive Systems](./programming/exclusive.md)
  - [Direct ECS World Access](./programming/world.md)
  - [Schedules](./programming/schedules.md)
  - [System Order of Execution](./programming/system-order.md)
  - [Run Criteria](./programming/run-criteria.md)
  - [System Sets](./programming/system-sets.md)
  - [States](./programming/states.md)
  - [Plugins](./programming/plugins.md)
  - [Change Detection](./programming/change-detection.md)
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
  - [Browser (WebAssembly)](./platforms/wasm.md)
    - [Panic Messages](./platforms/wasm/panic-console.md)
    - [Optimize for Size](./platforms/wasm/size-opt.md)
    - [Hosting on GitHub Pages](./platforms/wasm/gh-pages.md)
  - [Cross-Compilation](./setup/cross.md)
    - [From Linux to Windows](./setup/cross/linux-windows.md)
    - [From macOS to Windows](./setup/cross/macos-windows.md)

- [Appendix: General Concepts](./concepts.md)

---

[Credits](./credits.md)

---

[Contact Me](./contact.md)

---

[Contribute to Bevy](./contributing-bevy.md)
[Contribute to this Book](./contributing.md)
