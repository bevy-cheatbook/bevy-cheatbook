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

## Relative Transforms

If your entities represent "objects in the game world", you probably expect the
child to be positioned relative to the parent and move with it.

All Bundles that come with Bevy provide this behavior automatically.

If you are not using such a bundle, you need to make sure to add these components to both the parent and the child: `GlobalTransform` and `Transform`.

The `Transform` represents the relative position. You can manipulate it directly.

The `GlobalTransform` represents the absolute position. It is managed by bevy internally; do not manipulate it yourself.

For more info, see the [dedicated page about transforms](../features/transforms.md).
