# Resources

Akin to "global variables" or "singletons", used to hold data independent of entities.

Any Rust type (`struct` or `enum`) can be used as a resource.

Types must be unique; there can only be one resource of a given type.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:resource}}
```

Resources can be accessed from systems, using `Res`/`ResMut`.

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

If a resource of this type already exists, it will be overwritten.
