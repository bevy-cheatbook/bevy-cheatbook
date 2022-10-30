# Labels

{{#include ../include/links.md}}

You need labels to name various things in your [app][cb::app], such as
[systems][cb::system] (for [order control][cb::system-order]), [run
criteria][cb::runcriteria], [stages][cb::stage], and ambiguity sets.

The most common use for labels is for [system ordering][cb::system-label]
and adding [stages][cb::stage].

Bevy uses some clever Rust type system magic, to allow you to use strings
as well as your own custom types for labels, and even mix them!

Using strings for labels is quick and easy for prototyping. However, they
are easy to mistype and are unstructured. The compiler cannot validate them
for you, to catch mistakes.

You can also use custom types (usually `enum`s) to define your labels. This
allows the compiler to check them, and helps you stay organized in larger
projects.

You need to derive the appropriate trait, depending on what they will be
used for: `StageLabel`, `SystemLabel`, `RunCriteriaLabel`, or `AmbiguitySetLabel`.

Any Rust type is suitable, as long as it has the following standard Rust
traits: [`Clone`][std::Clone] (and the implied [`Send`][std::Send] +
[`Sync`][std::Sync] + `'static`).

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:labels}}
```

However, by defining your labels as custom types, the Rust compiler can check
them for you, and your IDE can auto-complete them. It is the recommended way,
as it prevents mistakes, and helps you stay organized in larger projects.
