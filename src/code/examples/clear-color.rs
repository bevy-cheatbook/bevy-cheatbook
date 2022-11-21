#![allow(dead_code)]
use bevy::prelude::*;

// ANCHOR: camera
use bevy::core_pipeline::clear_color::ClearColorConfig;

fn setup_camera_2d(
    mut commands: Commands,
) {
    // set the color for a specific camera (2D)
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.8, 0.4, 0.2)),
        },
        ..Default::default()
    });
}

fn setup_camera_3d(
    mut commands: Commands,
) {
    // set the color for a specific camera (3D)
    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.8, 0.4, 0.2)),
            ..Default::default()
        },
        ..Default::default()
    });
}
// ANCHOR_END: camera

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
