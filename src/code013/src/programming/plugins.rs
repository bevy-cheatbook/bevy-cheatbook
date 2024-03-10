mod plugins {

use bevy::prelude::*;

#[derive(Resource, Default)]
struct MyCustomResource;

#[derive(Resource, Default)]
struct MyOtherResource;

#[derive(Event)]
struct MyEvent;

fn plugin_init() {}
fn my_system() {}
fn do_some_things() {}
fn do_other_things() {}

// ANCHOR: plugin-fn
fn my_plugin(app: &mut App) {
    app.init_resource::<MyCustomResource>();
    app.add_systems(Update, (
        do_some_things,
        do_other_things,
    ));
}
// ANCHOR_END: plugin-fn

// ANCHOR: plugin-struct
struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyOtherResource>();
        app.add_event::<MyEvent>();
        app.add_systems(Startup, plugin_init);
        app.add_systems(Update, my_system);
    }
}
// ANCHOR_END: plugin-struct

// ANCHOR: plugin-app
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            my_plugin, // the `fn`-based plugin
            MyPlugin,  // the `struct`-based plugin
        ))
        .run();
}
// ANCHOR_END: plugin-app
}

mod plugin_groups {

use bevy::prelude::*;

struct FooPlugin;

impl Plugin for FooPlugin {
    fn build(&self, app: &mut App) {}
}

struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {}
}

// ANCHOR: plugin-groups
use bevy::app::PluginGroupBuilder;

struct MyPluginGroup;

impl PluginGroup for MyPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FooPlugin)
            .add(BarPlugin)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyPluginGroup)
        .run();
}
// ANCHOR_END: plugin-groups

fn disable_plugins() {
// ANCHOR: plugin-groups-disable
App::new()
    .add_plugins(
        DefaultPlugins.build()
            .disable::<bevy::log::LogPlugin>()
    )
    .run();
// ANCHOR_END: plugin-groups-disable
}

}

mod pluginconfig {
use bevy::prelude::*;

fn health_system() {}
fn movement_system() {}
fn player_invincibility() {}
fn free_camera() {}

// ANCHOR: plugin-config
struct MyGameplayPlugin {
    /// Should we enable dev hacks?
    enable_dev_hacks: bool,
}

impl Plugin for MyGameplayPlugin {
    fn build(&self, app: &mut App) {
        // add our gameplay systems
        app.add_systems(Update, (
            health_system,
            movement_system,
        ));
        // ...

        // if "dev mode" is enabled, add some hacks
        if self.enable_dev_hacks {
            app.add_systems(Update, (
                player_invincibility,
                free_camera,
            ));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyGameplayPlugin {
            // change to true for dev testing builds
            enable_dev_hacks: false,
        })
        .run();
}
// ANCHOR_END: plugin-config

fn _main2() {
// ANCHOR: defaultplugins-config
use bevy::window::WindowResolution;

App::new()
    .add_plugins(DefaultPlugins.set(
        // here we configure the main window
        WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800.0, 600.0),
                // ...
                ..Default::default()
            }),
            ..Default::default()
        }
    ))
    .run();
// ANCHOR_END: defaultplugins-config
}
}
