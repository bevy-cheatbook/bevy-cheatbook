# List All Resource Types

[Click here for the full example code.](../code/examples/print-resources.rs)

---

We can access the metadata stored inside Bevy ECS to learn about the types of
things currently stored.

This example shows how to print a list of all types that have been added as
[resources](../programming/res.md).

```rust,no_run,noplayground
{{#include ../code/examples/print-resources.rs:example}}
```

Note that this does *not* give you a comprehensive list of every Bevy-provided
type that is useful as a resource. It lists the types of all the resources
*currently added* to the app (by all registered plugins, your own, etc.).
