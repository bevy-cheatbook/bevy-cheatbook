# Local Resources

You can have per-system data, using `Local<T>`.

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:local-resource}}
```

The type must implement `Default` or `FromResources`. It is automatically initialized.

If the same type is used in multiple systems, they each get their own instance.
