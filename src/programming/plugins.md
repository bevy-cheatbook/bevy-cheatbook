{{#include ../include/header013.md}}

# Plugins

Relevant official examples:
[`plugin`][example::plugin],
[`plugin_group`][example::plugin_group].

---

As your project grows, it can be useful to make it more modular. You can
split it into "plugins".

Plugins are simply collections of things to be added to the [App
Builder][cb::app]. Think of this as a way to add things to the app from
multiple places, like different Rust files/modules or crates.

The simplest way to create a plugin is by just writing a Rust function
that takes [`&mut App`][`App`]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/plugins.rs:plugin-fn}}
```

An alternative way is by creating a `struct` and implementing the [`Plugin`] trait:

```rust,no_run,noplayground
{{#include ../code013/src/programming/plugins.rs:plugin-struct}}
```

The benefit of using a `struct` is that you could extend it with configuration
parameters or generics if you want to make your plugin configurable.

Either way, you get `&mut` access to the [`App`], so you can add whatever
you want to it, just like you can do from your `fn main()`.

You can now add your plugins to your [`App`] from elsewhere (most commonly
`fn main()`). Bevy will just call your plugin implementation above. In effect,
everything the plugin adds will be flattened into your [`App`] alongside
everything that is already there.

```rust,no_run,noplayground
{{#include ../code013/src/programming/plugins.rs:plugin-app}}
```

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

Plugin groups register multiple plugins at once.  Bevy's [`DefaultPlugins`]
and [`MinimalPlugins`] are examples of this.

To create your own plugin group:

```rust,no_run,noplayground
{{#include ../code013/src/programming/plugins.rs:plugin-groups}}
```

When adding a plugin group to the [app][cb::app], you can disable some
plugins while keeping the rest.

For example, if you want to manually set up logging (with your own `tracing`
subscriber), you can disable Bevy's [`LogPlugin`]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/plugins.rs:plugin-groups-disable}}
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
{{#include ../code013/src/programming/plugins.rs:plugin-config}}
```

Plugins that are added using [Plugin Groups][cb::plugingroup] can also be
configured. Many of Bevy's [`DefaultPlugins`] work this way.

```rust,no_run,noplayground
{{#include ../code013/src/programming/plugins.rs:defaultplugins-config}}
```

## Publishing Crates

Plugins give you a nice way to publish Bevy-based libraries for other people
to easily include into their projects.

Bevy offers some official guidance for good practices when you develop plugins
you want to publish for other people to use. [You can read it here.][bevy::plugin-guidelines]

Don't forget to submit an entry to [Bevy Assets][bevyassets] on the official
website, so that people can find your plugin more easily. You can do this
by making a PR in [the Github repo][project::bevyassets].

If you are interested in supporting bleeding-edge Bevy (main), [see here
for advice][cb::git-plugins].
