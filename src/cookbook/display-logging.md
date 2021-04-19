# Display logging

Bevy comes with logging out of the box based on `tracing`. We will not see any logging in the console by default, but you can enable it:

```rust,no_run,noplayground
{{#include ../code/src/cheatsheet.rs:logging}}
```

The `LogSettings` resource must come **before** `DefaultPlugins`/`LogPlugin` in your App.

Notably, asset load issues will silently fail unless logging is enabled with at least `WARN` level.