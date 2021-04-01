# Local Resources

`Local<T>` is a system parameter similar to `ResMut<T>`, which gives you full
mutable access to an instance of some data type, that is independent from
entities and components.

`Res<T>`/`ResMut<T>` refer to a single global instance of the type, shared
between all systems. On the other hand, every `Local<T>` parameter is a separate
instance, exclusively for that system.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:local-resource}}
```

The type must implement `Default` or `FromWorld`. It is automatically initialized.

A system can have multiple `Local`s of the same type.
