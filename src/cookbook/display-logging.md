# Display logging

Bevy comes with logging out of the box based on `tracing`. We will not see any logging in the console by default, but you can enable it:

```rust
.insert_resource(bevy::log::LogSettings {
    level: bevy::log::Level::DEBUG,
    ..Default::default()
})
```

This must come **before** `DefaultPlugins`/`LogPlugin` in your App.

Notably, asset load issues will silently fail unless logging is enabled with at least `WARN` level.