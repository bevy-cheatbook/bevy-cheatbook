use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

fn setup(mut commands: Commands) {
// ANCHOR: basic-setup
commands.spawn((
    Camera2dBundle {
        transform: Transform::from_xyz(100.0, 200.0, 0.0),
        ..default()
    },
    MyCameraMarker,
));
// ANCHOR_END: basic-setup
// ANCHOR: scalingmode
use bevy::render::camera::ScalingMode;

commands.spawn((
    Camera2dBundle {
        projection: OrthographicProjection {
            // For this example, let's make the screen/window height
            // correspond to 1600.0 world units. The width will
            // depend on the aspect ratio.
            scaling_mode: ScalingMode::FixedVertical(1600.0),
            ..default()
        },
        transform: Transform::from_xyz(100.0, 200.0, 0.0),
        ..default()
    },
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
