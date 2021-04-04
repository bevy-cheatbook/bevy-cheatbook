# Bevy Cheatsheet

Condensed single-page listing of common Bevy features and syntax.

Bookmark this page in your browser, for a quick reference when working with Bevy!

Intended for people who are familiar with the concepts.

If you are new to bevy or need a refresher, click on the "[explain]" links near
each feature, to go to the full page about the feature, in the
[Bevy Programming](./programming/_index.md) chapter!

---

## Table of Contents

Alphabetical index of Bevy features covered on this page.

|Topic | Definition |
|------|------------|
|[App Builder](#app-builder) | Bevy entry point; setup all the things to run. |
|[Assets](#assets) | Load external files into the game. |
|[Change Detection](#change-detection) | Write logic that responds to data being changed. |
|[Commands](#commands) | Spawn/despawn entities, manage components and resources. |
|[Components](#entities-and-components) | Basic data primitive in Bevy. |
|[Entities](#entities-and-components) | ID for a set of component values. |
|[Events](#events) | Communicate between systems. Send/receive data. |
|[Labels](#labels) | Names for systems, stages, and other things. |
|[Local Resources](#local-resources) | Per-system data. |
|[Parent/Child Hierarchy](#parentchild-hierarchy) | Entities in a hierarchy. |
|[Plugins](#plugins) | Use the App Builder in a modular way. |
|[Queries](#queries) | Access component data; find matching entities. |
|[Query Sets](#query-sets) | Resolve query conflicts. |
|[Resources](#resources) | Global data for the whole app. |
|[Stages](#app-builder) | Hard synchronization points for runtime scheduling. |
|[States](#states) | Multiple "modes" for your application. |
|[Systems](#systems) | The functions that contain your game logic. |
|[System Chaining](#system-chaining) | Combine multiple Rust functions into one big system. |
|[System Order](#system-order) | Control the runtime order of execution of systems. |
|[Transforms](#transforms) | Position and orientation of an object in the game world. |

---

## Systems

[(back to top)](#table-of-contents) [[explain](./programming/systems.md)]

Regular Rust functions, but can only accept parameters that implement [trait `SystemParam`](https://docs.rs/bevy/0.5.0/bevy/ecs/trait.SystemParam.html).

The standard system parameter types provided by Bevy:

{{#include ./include/systemparams.md}}

## Entities and Components

[(back to top)](#table-of-contents) [[explain](./programming/ec.md)]

Any Rust type (`struct` or `enum`) can be used as a component.

An entity is a simple integer ID for a set of components.

Type must be unique; only one instance per type per entity can exist. Use newtypes.

See [`Commands`](#commands) for how to add components to entities and spawn entities.

Component bundles:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:bundle}}
```

## Resources

[(back to top)](#table-of-contents) [[explain](./programming/res.md)]

Any Rust type (`struct` or `enum`) can be used as a resource.

Type must be unique; only one instance per type can exist. Use newtypes.

Access from system:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:res}}
```

Add when [building the app](#app-builder-main-function):

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:res-app-builder}}
```

Manage using [`Commands`](#commands):

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:res-commands}}
```

For complex resource initialization, implement `FromWorld`:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:fromworld}}
```

## Commands

[(back to top)](#table-of-contents) [[explain](./programming/commands.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:commands}}
```

These actions are applied at the end of the stage.

## Queries

[(back to top)](#table-of-contents) [[explain](./programming/queries.md)]

Iterate all matching entities:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:query}}
```

Or, if expecting the query to match only one entity (returns `Result`):

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:query-single}}
```

To operate on a specific entity:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:query-one}}
```

Add query filters:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:query-filter}}
```

## Change Detection

[(back to top)](#table-of-contents) [[explain](./programming/change-detection.md)]

Query filters:
 - `Added<T>`: detect adding new components to existing entities
 - `Changed<T>`: detect when a component is modified

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:change-detection}}
```

Change detection is triggered by `DerefMut`. Accessing components via a mutable
query without it actually being a `&mut` access, will *not* trigger it.

Beware of [frame delay / 1-frame-lag](./pitfalls/frame-delay.md). You may want
to use [explicit system ordering](#system-order).

## Query Sets

[(back to top)](#table-of-contents) [[explain](./programming/query-sets.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:query-set}}
```

## Events

[(back to top)](#table-of-contents) [[explain](./programming/events.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:events}}
```

Events don't persist. They are stored until the end of the next frame, after
which they are lost. If your systems do not handle events every frame, you could
miss some.

Beware of [frame delay / 1-frame-lag](./pitfalls/frame-delay.md). You may want
to use [explicit system ordering](#system-order).

## Local Resources

[(back to top)](#table-of-contents) [[explain](./programming/local.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:local}}
```

The type must implement `Default` or `FromWorld`. It is automatically initialized.

## Labels

[(back to top)](#table-of-contents) [[explain](./programming/labels.md)]

Labels can be strings, or any other type that implements the relevant traits.

For example, a custom enum type:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:labels}}
```

## Plugins

[(back to top)](#table-of-contents) [[explain](./programming/plugins.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:plugin}}
```

## App Builder

[(back to top)](#table-of-contents) [[explain](./programming/app-builder.md), [stages](./programming/stages.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:app-builder}}
```

## System Order

[(back to top)](#table-of-contents) [[explain](./programming/system-order.md)]

Execution order is *nondeterministic* and may change every frame!

Use [labels](#labels) to specify order explicitly:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:system-labels}}
```

## States

[(back to top)](#table-of-contents) [[explain](./programming/states.md)]

App builder with States:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:states}}
```

Change or check States:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:res-state}}
```

## Transforms

[(back to top)](#table-of-contents) [[explain](./features/transforms.md)]

Coordinate system: +X is right, +Y is up, +Z is out of the screen. Right-handed. 3D matches 2D.

Transforms consist of: translation (coordinates), rotation, scale.

The `Transform` component is the relative/local transform. You may modify this directly.

The `GlobalTransform` component is the absolute/global transform. Do not modify; internally managed by Bevy.

## Parent/Child Hierarchy

[(back to top)](#table-of-contents) [[explain](./programming/parent-child.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:parent-child}}
```

Reparent an existing entity:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:add-parent}}
```

Transforms: Ensure both parent and child have both components: `Transform`, `GlobalTransform`.

## System Chaining

[(back to top)](#table-of-contents) [[explain](./programming/system-chaining.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:system-chaining}}
```

## Assets

[(back to top)](#table-of-contents) [[explain](./features/assets.md)]

Load asset:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:asset-load}}
```

Access assets from systems:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:asset-use}}
```

Create assets from code:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:asset-new}}
```

Asset events:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:asset-event}}
```
