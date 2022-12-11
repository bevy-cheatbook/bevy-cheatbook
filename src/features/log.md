# Logging, Console Messages

{{#include ../include/links.md}}

Relevant official examples:
[`logs`][example::logs].

---

You may have noticed how, when you run your Bevy project, you get messages
in your console window. For example:

```
2022-06-12T13:28:25.445644Z  WARN wgpu_hal::vulkan::instance: Unable to find layer: VK_LAYER_KHRONOS_validation
2022-06-12T13:28:25.565795Z  INFO bevy_render::renderer: AdapterInfo { name: "AMD Radeon RX 6600 XT", vendor: 4098, device: 29695, device_type: DiscreteGpu, backend: Vulkan }
2022-06-12T13:28:25.565795Z  INFO mygame: Entered new map area.
```

Log messages like this can come from Bevy, dependencies (like wgpu), and
also from your own code.

Bevy offers a logging framework that is much more advanced than simply using
`println`/`eprintln` from Rust. Log messages can have metadata, like the
level, timestamp, and Rust module where it came from. You can see that this
metadata is printed alongside the contents of the message.

## Levels

Levels determine how important a message is, and allow messages to be filtered.

The available levels are: `error`, `warn`, `info`, `debug`, `trace`.

A rough guideline for when to use each level, could be:
 - `error`: something happened that prevents things from working correctly
 - `warn`: something unusual happened, but things can continue to work
 - `info`: general informational messages
 - `debug`: for development, messages about what your code is doing
 - `trace`: for very verbose debug data, like dumping values

## Printing your own log messages

To display a message, just use the macro named after the level of the
message. The syntax is exactly the same as with Rust's `println`. See the
[`std::fmt`][std::fmt] documentation for more details.

```rust
error!("Unknown condition!");
warn!("Something unusual happened!");
info!("Entered game level: {}", level_id);
debug!("x: {}, state: {:?}", x, state);
trace!("entity transform: {:?}", transform);
```

## Filtering messages

To control what messages you would like to see, you can configure Bevy's
[`LogPlugin`][bevy::LogPlugin]:

```rust
{{#include ../code/examples/log-settings.rs:log-settings}}
```

The `filter` field is a string specifying a list of rules for what level to
enable for different Rust modules/crates. In the example above, the string
means: show up to `info` by default, limit `wgpu_core` and `wgpu_hal`
to `warn` level, for `mygame` show `debug`.

All levels higher than the one specified are also enabled. All levels lower
than the one specified are disabled, and those messages will not be displayed.

The `level` filter is a global limit on the lowest level to use. Messages
below that level will be ignored and most of the performance overhead avoided.

### Environment Variable

You can override the filter string when running your app, using the `RUST_LOG`
environment variable.

```sh
RUST_LOG="warn,mygame=debug" ./mygame
```

Note that other Rust projects, such as `cargo`, also use the same
environment variable to control their logging. This can lead to unexpected
consequences. For example, doing:

```sh
RUST_LOG="debug" cargo run
```

will cause your console to also be filled with debug messages from `cargo`.

### Different settings for debug and release builds

If you want to do different things in your Rust code for debug/release
builds, an easy way to achieve it is using conditional compilation on
"debug assertions".

```rust
{{#include ../code/examples/log-settings.rs:log-settings-debugrelease}}
```

This is a good reason why [you should not use release mode during development
just for performance reasons][pitfall::perf].

On Microsoft Windows, your game EXE will also launch with a console window for
displaying log messages by default. You might not want that in release builds.
[See here.][platform::windows::noconsole]

## Performance Implications

Printing messages to the console is a relatively slow operation.

However, if you are not printing a large volume of messages, don't worry
about it. Just avoid spamming lots of messages from performance-sensitive
parts of your code like inner loops.

You can disable log levels like `trace` and `debug` in release builds.
