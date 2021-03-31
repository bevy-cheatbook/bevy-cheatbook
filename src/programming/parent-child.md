# Hierarchical (Parent/Child) Entities

Technically, the Entities/Components themselves cannot form a hierarchy (it is a
flat data structure). However, logical hierarchies are a common pattern in games.

Bevy supports creating such a logical link between entities, to form a virtual
"hierarchy", by simply adding `Parent` and  `Children` components on the
respective entities.

`Commands` has methods for adding children to entities, which automatically add the correct components:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:parenting}}
```

You can despawn an entire hierarchy with a single command:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:despawn-recursive}}
```

## Accessing the Parent or Children

To make a system that works with the hierarchy, you typically need two queries:
 - one with the components you need from the child entities
 - one with the components you need from the parent entities

One of the two queries should include the appropriate component, to obtain the
entity ids to use with the other one:
 - `Parent` in the child query, if you want to iterate entities and look up
   their parents, or
 - `Children` in the parent query, if you want to iterate entities and look up
   their children

For example, if we want to get the `Transform` of cameras that have a parent,
and the `GlobalTransform` of their parent:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-parent}}
```

As another example, say we are making a strategy game, and we have Units that are
children of a Squad. Say we need to make a system that works on each Squad, and it
needs some information about the children:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-child}}
```

## Relative Transforms

If your entities represent "objects in the game world", you probably expect the
child to be positioned relative to the parent and move with it.

All Bundles that come with Bevy provide this behavior automatically.

If you are not using such a bundle, you need to make sure to add these components to both the parent and the child: `GlobalTransform` and `Transform`.

The `Transform` represents the relative position. You can manipulate it directly.

The `GlobalTransform` represents the absolute position. It is managed by bevy internally; do not manipulate it yourself.

For more info, see the [dedicated page about transforms](../features/transforms.md).
