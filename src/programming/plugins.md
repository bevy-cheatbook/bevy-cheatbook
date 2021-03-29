# Plugins

As your project grows, it can be useful to make it more modular. You can split it into plugins.

Plugins are simply collections of things to be added to the App Builder.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:plugins}}
```

---

If you intend to publish plugins as crates for public use, you should read
[the official guidelines for plugin authors](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md).
