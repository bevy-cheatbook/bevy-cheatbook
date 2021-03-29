# Resources

Resources allow you to store a single global instance of some data type, independently of entities.

Use them for data that is truly global for your app, such configuration / settings.

Even if there is only one of a certain thing (such as the player in a
single-player game), it is still preferable to use an entity instead of a
resource, because entities are composed of multiple components, some of which
can be shared with other entities. Resources are monolithic.

Similar to components, any Rust type (`struct` or `enum`) can be used as a resource.

Types must be unique; there can only be one instance of a given type.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:resource}}
```

Resources can be accessed from [systems](./systems.md), using `Res`/`ResMut`.

## Resource Initialization

Implement `Default` for simple resources:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:resource-default}}
```

For resources that need complex initialization, implement `FromWorld`:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:fromworld}}
```

You can create your resources at [`App` creation](./app-builder.md):

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:appinit-resource}}
```

Or using [`Commands`](./commands.md) from inside a system:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:commands-resource}}
```

If you insert a resource of a type that already exists, it will be overwritten.
