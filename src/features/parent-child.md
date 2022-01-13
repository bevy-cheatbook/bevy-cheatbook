# Hierarchical (Parent/Child) Entities

{{#include ../include/navlinks.md}}
{{#include ../include/docsrs.md}}
{{#include ../include/bevy-examples.md}}

Relevant official examples:
[`hierarchy`][example::hierarchy],
[`parenting`][example::parenting].

---

Technically, the [Entities/Components][cb::ec] themselves cannot form a
hierarchy (the [ECS][cb::ecs-intro] is a flat data structure). However,
logical hierarchies are a common pattern in games.

Bevy supports creating such a logical link between entities, to form
a virtual "hierarchy", by simply adding [`Parent`][bevy::Parent] and
[`Children`][bevy::Children] components on the respective entities.

When using [Commands][cb::commands] to spawn entities,
[`Commands`][bevy::Commands] has methods for adding children to entities,
which automatically add the correct components:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:parenting}}
```

You can despawn an entire hierarchy with a single [command][cb::commands]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:despawn-recursive}}
```

## Accessing the Parent or Children

To make a system that works with the hierarchy, you typically need two [queries][cb::query]:
 - one with the components you need from the child entities
 - one with the components you need from the parent entities

One of the two queries should include the appropriate component, to obtain the
entity ids to use with the other one:
 - [`Parent`][bevy::Parent] in the child query, if you want to iterate entities
   and look up their parents, or
 - [`Children`][bevy::Children] in the parent query, if you want to iterate entities
   and look up their children

For example, if we want to get the [`Transform`][bevy::Transform]
of cameras ([`Camera`][bevy::Camera]) that have a parent, and the
[`GlobalTransform`][bevy::GlobalTransform] of their parent:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-parent}}
```

As another example, say we are making a strategy game, and we have Units
that are children of a Squad. Say we need to make a system that works on
each Squad, and it needs some information about the children:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-child}}
```

## Relative Transforms

If your entities represent "objects in the game world", you probably expect
the child to be positioned relative to the parent and move with it.

All Bundles that come with Bevy provide this behavior automatically.

If you are not using such a bundle, you need to make sure
to add these components to both the parent and the child entities:
[`GlobalTransform`][bevy::GlobalTransform] and [`Transform`][bevy::Transform].

The [`Transform`][bevy::Transform] represents the relative position.
You can manipulate it directly.

The [`GlobalTransform`][bevy::GlobalTransform] represents the absolute position.
It is managed by bevy internally; do not manipulate it yourself.

For more info, see the [dedicated page about transforms][cb::transform].
