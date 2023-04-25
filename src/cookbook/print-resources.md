{{#include ../include/header09.md}}

# List All Resource Types

[Click here for the full example code.][cbexample::print-resources]

---

This example shows how to print a list of all types that have been added as
[resources][cb::res].

```rust,no_run,noplayground
{{#include ../code/examples/print-resources.rs:example}}
```

Note that this does *not* give you a comprehensive list of every Bevy-provided
type that is useful as a resource. It lists the types of all the resources
*currently added* to the app (by all registered plugins, your own, etc.).

[See here for a more useful list types provided in Bevy.][chapter::builtins]
