{{#include ../include/header09.md}}

# Visibility

Relevant official examples:
[`parenting`][example::parenting].

---

Visibility is used to control if something is to be rendered or not. If you
want an entity to exist in the world, just not be displayed, you can hide it.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:visibility}}
```

## Visibility Components

In Bevy, visibility is represented by **two** [components][cb::component]:
[`Visibility`][bevy::Visibility] and [`ComputedVisibility`][bevy::ComputedVisibility].

Any [Entity][cb::ecs-intro] that represents an object in the game world
needs to have both. All of Bevy's [built-in bundle types][builtins::bundle]
include them.

If you are creating a custom entity without using those [bundles][cb::bundle],
you can use one of the following to ensure you don't miss them:
 - [`SpatialBundle`][bevy::SpatialBundle] for [transforms][cb::transform] + visibility
 - [`VisibilityBundle`][bevy::VisibilityBundle] for just visibility

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:spatialbundle}}
```

### `Visibility`

[`Visibility`][bevy::Visibility] is the "user-facing toggle". This is where
you specify if you want this entity to be visible or not.

If you hide an entity that has [children][cb::hierarchy], they will also be
hidden, regardless of what their individual [`Visibility`][bevy::Visibility]
value is set to.

### `ComputedVisibility`

[`ComputedVisibility`][bevy::ComputedVisibility] represents the actual final
decision made by Bevy about whether this entity needs to be displayed.

The value of [`ComputedVisibility`][bevy::ComputedVisibility] is read-only. It
is managed internally by Bevy.

It can be affected by the visibility of [parent][cb::hierarchy] entities, if any.

It is also affected by "culling" systems. If the entity is not in the range of
any Camera or Light, it does not need to be rendered, so Bevy will hide it.

## Checking Actual Visibility

You can use [`ComputedVisibility`][bevy::ComputedVisibility] to check if
the entity is actually visible.

Bevy's internal visibility computations are done in the
[`PostUpdate`][bevy::CoreStage] [stage][cb::stage]. To get the up-to-date
values for the current frame, your [system][cb::system] must be [ordered
after][cb::system-order] these bevy-internal systems. You can use the
[`VisibilitySystems`][bevy::VisibilitySystems] [labels][cb::label].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:computedvisibility}}
```
