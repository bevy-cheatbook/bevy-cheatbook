use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

mod basic_setup {
use super::*;
// ANCHOR: basic-setup
#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}
// ANCHOR_END: basic-setup
}

fn setup(mut commands: Commands) {
// ANCHOR: projection-near-far
commands.spawn((
    Camera2dBundle {
        projection: OrthographicProjection {
            // don't forget to set `near` and `far`
            near: -1000.0,
            far: 1000.0,
            // ... any other settings you want to change ...
            ..default()
        },
        ..default()
    },
    MyCameraMarker,
));
// ANCHOR_END: projection-near-far

// ANCHOR: projection-mut
let mut camera_bundle = Camera2dBundle::default();
// change the settings we want to change:
camera_bundle.projection.scale = 2.0;
camera_bundle.transform.rotate_z(30f32.to_radians());
// ...

commands.spawn((
    camera_bundle,
    MyCameraMarker,
));
// ANCHOR_END: projection-mut

// ANCHOR: scalingmode
use bevy::render::camera::ScalingMode;

let mut my_2d_camera_bundle = Camera2dBundle::default();
// For this example, let's make the screen/window height correspond to
// 1600.0 world units. The width will depend on the aspect ratio.
my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1600.0);
my_2d_camera_bundle.transform = Transform::from_xyz(100.0, 200.0, 0.0);

commands.spawn((
    my_2d_camera_bundle,
    MyCameraMarker,
));
// ANCHOR_END: scalingmode
}

// ANCHOR: query-projection
fn debug_projection(
    query_camera: Query<&OrthographicProjection, With<MyCameraMarker>>,
) {
    let projection = query_camera.single();
    // ... do something with the projection
}
// ANCHOR_END: query-projection

// ANCHOR: zoom-scale
fn zoom_scale(
    mut query_camera: Query<&mut OrthographicProjection, With<MyCameraMarker>>,
) {
    let mut projection = query_camera.single_mut();
    // zoom in
    projection.scale /= 1.25;
    // zoom out
    projection.scale *= 1.25;
}
// ANCHOR_END: zoom-scale

// ANCHOR: zoom-scalingmode
fn zoom_scalingmode(
    mut query_camera: Query<&mut OrthographicProjection, With<MyCameraMarker>>,
) {
    use bevy::render::camera::ScalingMode;

    let mut projection = query_camera.single_mut();
    // 4 screen pixels to world/game pixel
    projection.scaling_mode = ScalingMode::WindowSize(4.0);
    // 6 screen pixels to world/game pixel
    projection.scaling_mode = ScalingMode::WindowSize(6.0);
}
// ANCHOR_END: zoom-scalingmode

fn _main() {
    App::new().add_systems(Startup, (
        setup,
        debug_projection,
        zoom_scale,
        zoom_scalingmode,
    ));
}

// ANCHOR: default-nearest
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
        )
        // ...
        .run();
}
// ANCHOR_END: default-nearest
