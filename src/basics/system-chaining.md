# System Chaining

Systems can take an input and produce an output.

One useful application of this is error handling:

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:system-io}}
```

Such systems cannot be registered individually (bevy doesn't know what to do
with the input/output). You have to connect them in a chain:

```rust,no_run,noplayground
{{#include ../code_bevy_release/src/basics.rs:system-chain}}
```

In general, this pattern is useful for any scenario where you have one system
that requires data produced from another system.

