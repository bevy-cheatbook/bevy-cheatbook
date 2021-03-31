# Labels

You need labels to name various things in your app, such as [stages](./stages.md),
[systems](./system-order.md), [run criteria](./run-criteria.md), and [ambiguity sets](TODO).

Bevy uses some clever Rust type system magic, to allow you to use strings
as well as your own custom types for labels, and even mix them!

You may use a value of any type as a label, as long as it has the following
standard Rust traits: `Clone + Eq + Hash + Debug` (and the implied `+ Send +
Sync + 'static`).

You need to derive the appropriate trait: `StageLabel`, `SystemLabel`, `RunCriteriaLabel`, or `AmbiguitySetLabel`.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:labels}}
```

For quick prototyping, it is convenient to just use strings as labels.

However, by defining your labels as a custom `enum` type, the Rust compiler can
check them for you, and your IDE can auto-complete them. It is the recommended
way, as it helps you stay organized in larger projects.
