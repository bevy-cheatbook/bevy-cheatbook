# Local Resources

You can have per-system data, using `Local<T>`.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:local-resource}}
```

The type must implement `Default` or `FromWorld`. It is automatically initialized.

These are similar to Resources, but each system will have its own instance.

A system can also have multiple `Local`s of the same type.
