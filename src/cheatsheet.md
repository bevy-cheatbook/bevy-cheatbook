# Bevy Cheatsheet

One-page overview of common bevy syntax and features.

Concise listing of the syntax + important usage notes, no explanations.

Intended for people who are familiar with the core concepts.

If you are new to bevy or need a refresher, see
[Bevy Programming](./programming/_index.md)
for brief explanations of each feature.

---

Click on "[explain]" links to go to the relevant [Bevy Programming](./programming/_index.md) page!

---

## Systems

[[explain](./programming/systems.md)]

Regular Rust functions, but can only accept parameters that implement [trait `SystemParam`](TODO DOCS.RS).

The standard system parameter types provided by Bevy:

{{#include ./include/systemparams.md}}

## Entities and Components

[[explain](./programming/ec.md)]

Any Rust type (`struct` or `enum`) can be used as a component.

Type must be unique; only one instance per type per entity can exist. Use newtypes.

See [`Commands`](#commands) for how to add components to entities and spawn entities.

Component bundles:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:bundle}}
```

## Resources

[[explain](./programming/res.md)]

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

## Queries

[[explain](./programming/queries.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:query}}
```

To operate on a specific entity:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:query-one}}
```

Add query filters:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:query-filter}}
```

Query sets (to resolve component access conflicts):

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:query-set}}
```

## Change Detection

[[explain](./programming/change-detection.md)]

Query filters:
 - `Added<T>`: detect adding new components to existing entities
 - `Changed<T>`: detect when a component is modified

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:change-detection}}
```

Change detection is triggered by `DerefMut`. Accessing components via a mutable
query without it actually being a `&mut` access, will *not* trigger it.

Beware of [frame delay / 1-frame-lag](./pitfalls/frame-delay.md). You may want
to use [explicit system ordering](#system-order-of-execution).

## Commands

[[explain](./programming/commands.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:commands}}
```

These actions are applied at the end of the stage.

## Local (per-system) Resources

[[explain](./programming/local.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:local}}
```

The type must implement `Default` or `FromWorld`. It is automatically initialized.

## Events

[[explain](./programming/events.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:events}}
```

Events don't persist. They are stored until the end of the next frame, after
which they are lost. If your systems do not handle events every frame, you could
miss some.

Beware of [frame delay / 1-frame-lag](./pitfalls/frame-delay.md). You may want
to use [explicit system ordering](#system-order-of-execution).

## Entity Hierarchies (Parent/Child)

[[explain](./programming/parent-child.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:parent-child}}
```

Reparent an existing entity:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:add-parent}}
```

If you are using `Transform`s, ensure both the parent and the child also have a `GlobalTransform`.

Child `Transform` is relative to the parent. `GlobalTransform` is internally managed by bevy.

## System Chaining

[[explain](./programming/system-chaining.md)]

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:system-chaining}}
```

## App Builder (main function)

[[explain](./programming/app-builder.md), [stages](./programming/stages.md), [plugins](./programming/plugins.md)]

App builder:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:app-builder}}
```

Custom plugin:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:plugin}}
```

## System Order of Execution

[[explain](./programming/system-order.md)]

## States

[[explain](./programming/states.md)]

App builder with States:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:states}}
```

Change or check States:

```rust,no_run,noplayground
{{#include ./code/src/cheatsheet.rs:res-state}}
```

## Assets

[[explain](./features/assets.md)]

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