{{#include ../include/header012.md}}

# Visibility

Relevant official examples:
[`parenting`][example::parenting].

---

Visibility is used to control if something is to be rendered or not. If you
want an entity to exist in the world, just not be displayed, you can hide it.

```rust,no_run,noplayground
{{#include ../code012/src/fundamentals/visibility.rs:visibility}}
```

## Visibility Components

In Bevy, visibility is represented by **multiple** [components][cb::component]:
 - [`Visibility`][bevy::Visibility]: the user-facing toggle (here is where you set what you want)
 - [`InheritedVisibility`][bevy::InheritedVisibility]: used by Bevy to keep track of the state from any [parent entities][cb::hierarchy]
 - [`ViewVisibility`][bevy::ViewVisibility]: used by Bevy to track if the entity should actually be displayed

Any [Entity][cb::ecs-intro] that represents a renderable object in
the game world needs to have them all. All of Bevy's [built-in bundle
types][builtins::bundle] include them.

If you are creating a custom entity without using those [bundles][cb::bundle],
you can use one of the following to ensure you don't miss them:
 - [`SpatialBundle`][bevy::SpatialBundle] for [transforms][cb::transform] + visibility
 - [`VisibilityBundle`][bevy::VisibilityBundle] for just visibility

```rust,no_run,noplayground
{{#include ../code012/src/fundamentals/visibility.rs:spatialbundle}}
```

If you don't do this correctly (say, you manually add just the `Visibility`
component and forget the others, because you don't use a bundle), your
entities will not render!

### `Visibility`

[`Visibility`][bevy::Visibility] is the "user-facing toggle". This is where
you specify what you want for the current entity:
 - `Inherited` (default): show/hide depending on [parent][cb::hierarchy]
 - `Visible`: always show the entity, regardless of parent
 - `Hidden`: always hide the entity, regardless of parent

If the current entity has any [children][cb::hierarchy] that have `Inherited`,
their visibility will be affected if you set the current entity to `Visible`
or `Hidden`.

### `InheritedVisibility`

[`InheritedVisibility`][bevy::InheritedVisibility] represents the state the
current entity would have based on its [parent][cb::hierarchy]'s visibility.

The value of [`InheritedVisibility`][bevy::InheritedVisibility] should
be considered read-only. It is managed internally by Bevy, in a manner
similar to [transform propagation][cb::transform-propagate]. A "visibility
propagation" [system][cb::system] runs in the [`PostUpdate`][bevy::PostUpdate]
[schedule][cb::schedule].

If you want to read the up-to-date value for the current frame, you should
[add][cb::app] your [system][cb::system] to the [`PostUpdate`][bevy::PostUpdate]
[schedule][cb::schedule] and [order it after][cb::system-order]
[`VisibilitySystems::VisibilityPropagate`][bevy::VisibilitySystems].

```rust,no_run,noplayground
{{#include ../code012/src/fundamentals/visibility.rs:inheritedvisibility}}
```
```rust,no_run,noplayground
{{#include ../code012/src/fundamentals/visibility.rs:inheritedvisibility-app}}
```

### `ViewVisibility`

[`ViewVisibility`][bevy::ViewVisibility] represents the actual final
decision made by Bevy about whether this entity needs to be rendered.

The value of [`ViewVisibility`][bevy::ViewVisibility] is read-only. It
is managed internally by Bevy.

It is used for "culling": if the entity is not in the range of
any Camera or Light, it does not need to be rendered, so Bevy will hide it
to improve performance.

Every frame, after "visibility propagation", Bevy will check what entities
can be seen by what view (camera or light), and store the outcome in these
components.

If you want to read the up-to-date value for the current frame, you should
[add][cb::app] your [system][cb::system] to the [`PostUpdate`][bevy::PostUpdate]
[schedule][cb::schedule] and [order it after][cb::system-order]
[`VisibilitySystems::CheckVisibility`][bevy::VisibilitySystems].

```rust,no_run,noplayground
{{#include ../code012/src/fundamentals/visibility.rs:viewvisibility}}
```
```rust,no_run,noplayground
{{#include ../code012/src/fundamentals/visibility.rs:viewvisibility-app}}
```
