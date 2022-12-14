# Plugins

{{#include ../include/links.md}}

Relevant official examples:
[`plugin`][example::plugin],
[`plugin_group`][example::plugin_group].

---

As your project grows, it can be useful to make it more modular. You can
split it into "plugins".

Plugins are simply collections of things to be added to the [App
Builder][cb::app]. Think of this as a way to add things to the app from
multiple places, like different Rust files/modules or crates.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:plugins}}
```

Note how you get `&mut` access to the [`App`][bevy::App], so you can
add whatever you want to it, just like you can do from your `fn main`.

For internal organization in your own project, the main value of plugins
comes from not having to declare all your Rust types and functions as
`pub`, just so they can be accessible from `fn main` to be added to the
app builder. Plugins let you add things to your [app][cb::app] from multiple
different places, like separate Rust files / modules.

You can decide how plugins fit into the architecture of your game.

Some suggestions:
 - Create plugins for different [states][cb::state].
 - Create plugins for various sub-systems, like physics or input handling.

## Plugin groups

Plugin groups register multiple plugins at once.
Bevy's [`DefaultPlugins`][bevy::DefaultPlugins] and
[`MinimalPlugins`][bevy::MinimalPlugins] are examples of this.
To create your own plugin group:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:plugin-groups}}
```

When adding a plugin group to the [app][cb::app], you can disable some
plugins while keeping the rest.

For example, if you want to manually set up logging (with your own `tracing`
subscriber), you can disable Bevy's [`LogPlugin`][bevy::LogPlugin]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:plugin-groups-disable}}
```

Note that this simply disables the functionality, but it cannot actually
remove the code to avoid binary bloat. The disabled plugins still have to
be compiled into your program.

If you want to slim down your build, you should look at disabling Bevy's
default [cargo features][cb::features], or depending on the various Bevy
sub-crates individually.

## Plugin Configuration

Plugins are also a convenient place to store settings/configuration that are
used during initialization/startup. For settings that can be changed at runtime,
it is recommended that you put them in [resources][cb::res] instead.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:plugin-config}}
```

Plugins that are added using [Plugin Groups][cb::plugingroup] can also be
configured. Many of Bevy's [`DefaultPlugins`][bevy::DefaultPlugins] work
this way.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:defaultplugins-config}}
```

## Publishing Crates

Plugins give you a nice way to publish Bevy-based libraries for other people
to easily include into their projects.

If you intend to publish plugins as crates for public use, you should read
[the official guidelines for plugin authors][bevy::plugins_guidelines].

Don't forget to submit an entry to [Bevy Assets][bevyassets] on the official
website, so that people can find your plugin more easily. You can do this
by making a PR in [the Github repo][project::bevyassets].

If you are interested in supporting bleeding-edge Bevy (main), [see here
for advice][cb::git-plugins].
