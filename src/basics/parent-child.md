# Hierarchical (Parent/Child) Entities

Technically, the Entities/Components themselves cannot form a hierarchy (it is a
flat data structure). However, logical hierarchies are a common pattern in games.

Bevy supports creating such a logical link between entities, to form a virtual
"hierarchy", by simply adding a `Parent` or `Children` component on the
respective entities.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:parenting}}
```

Bevy has an internal mechanism to ensure the `Parent` component of the child entity
and the `Children` component of the parent entity are kept in sync.

You can despawn an entire hierarchy with a single command:

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:despawn-recursive}}
```

## Relative Transforms

To use transforms with hierarchical entities, both the parent and the child must have a `GlobalTransform` and a `Transform`.

The `GlobalTransform` is managed by bevy internally; do not mutate it yourself.

The `Transform` is what you can manipulate directly. On the child, it will behave relative to the parent.

All component bundles that come with bevy provide this correctly, but if you are
not using such a bundle, you need to make sure to add both of these components to both entities.
