use bevy::prelude::*;

fn setup_camera_3d(
    mut commands: Commands,
) {
// ANCHOR: camera
use bevy::render::camera::ClearColorConfig;

// configure the background color (if any), for a specific camera (3D)
commands.spawn(Camera3dBundle {
    camera: Camera {
        // clear the whole viewport with the given color
        clear_color: ClearColorConfig::Custom(Color::rgb(0.8, 0.4, 0.2)),
        ..Default::default()
    },
    ..Default::default()
});

// configure the background color (if any), for a specific camera (2D)
commands.spawn(Camera2dBundle {
    camera: Camera {
        // disable clearing completely (pixels stay as they are)
        // (preserves output from previous frame or camera/pass)
        clear_color: ClearColorConfig::None,
        ..Default::default()
    },
    ..Default::default()
});
// ANCHOR_END: camera
}

// ANCHOR: main
fn setup(
    mut commands: Commands,
) {
    // this camera will use the default color
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // set the global default clear color
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_systems(Startup, setup)
        .run();
}
// ANCHOR_END: main

fn _main() {
    App::new()
        .add_systems(Startup, setup_camera_3d)
        .run();
}

