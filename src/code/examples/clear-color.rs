#![allow(dead_code)]
use bevy::prelude::*;

fn setup_camera_3d(
    mut commands: Commands,
) {
// ANCHOR: camera
use bevy::core_pipeline::clear_color::ClearColorConfig;

// configure the background color (if any), for a specific camera (3D)
commands.spawn(Camera3dBundle {
    camera_3d: Camera3d {
        // clear the whole viewport with the given color
        clear_color: ClearColorConfig::Custom(Color::rgb(0.8, 0.4, 0.2)),
        ..Default::default()
    },
    ..Default::default()
});

// configure the background color (if any), for a specific camera (2D)
commands.spawn(Camera2dBundle {
    camera_2d: Camera2d {
        // disable clearing completely (pixels stay as they are)
        // (preserves output from previous frame or camera/pass)
        clear_color: ClearColorConfig::None,
    },
    ..Default::default()
});
// ANCHOR_END: camera
}

// ANCHOR: main
fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        // set the global default
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
// ANCHOR_END: main
