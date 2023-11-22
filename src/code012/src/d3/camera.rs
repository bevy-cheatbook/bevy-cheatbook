use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

fn setup(mut commands: Commands) {
// ANCHOR: basic-setup
commands.spawn((
    Camera3dBundle {
        transform: Transform::from_xyz(10.0, 12.0, 16.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
    MyCameraMarker,
));
// ANCHOR_END: basic-setup
// ANCHOR: fov
commands.spawn((
    Camera3dBundle {
        projection: PerspectiveProjection {
            // We must specify the FOV in radians.
            // Rust can convert degrees to radians for us.
            fov: 60.0_f32.to_radians(),
            ..default()
        }.into(),
        transform: Transform::from_xyz(10.0, 12.0, 16.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
    MyCameraMarker,
));
// ANCHOR_END: fov
// ANCHOR: ortho
use bevy::render::camera::ScalingMode;

commands.spawn((
    Camera3dBundle {
        projection: OrthographicProjection {
            // For this example, let's make the screen/window height
            // correspond to 16.0 world units.
            scaling_mode: ScalingMode::FixedVertical(16.0),
            ..default()
        }.into(),
        // the distance doesn't really matter for orthographic,
        // it should look the same (though it might affect
        // shadows and clipping / culling)
        transform: Transform::from_xyz(10.0, 12.0, 16.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
    MyCameraMarker,
));
// ANCHOR_END: ortho
}

// ANCHOR: query-projection
fn debug_projection(
    query_camera: Query<&Projection, With<MyCameraMarker>>,
) {
    let projection = query_camera.single();
    match projection {
        Projection::Perspective(persp) => {
            // we have a perspective projection
        }
        Projection::Orthographic(ortho) => {
            // we have an orthographic projection
        }
    }
}
// ANCHOR_END: query-projection

// ANCHOR: fov-zoom
fn zoom_perspective(
    mut query_camera: Query<&mut Projection, With<MyCameraMarker>>,
) {
    // assume perspective. do nothing if orthographic.
    let Projection::Perspective(persp) = query_camera.single_mut().into_inner() else {
        return;
    };
    // zoom in
    persp.fov /= 1.25;
    // zoom out
    persp.fov *= 1.25;
}
// ANCHOR_END: fov-zoom

// ANCHOR: orthographic-zoom
fn zoom_orthographic(
    mut query_camera: Query<&mut Projection, With<MyCameraMarker>>,
) {
    // assume orthographic. do nothing if perspective.
    let Projection::Orthographic(ortho) = query_camera.single_mut().into_inner() else {
        return;
    };
    // zoom in
    ortho.scale /= 1.25;
    // zoom out
    ortho.scale *= 1.25;
}
// ANCHOR_END: orthographic-zoom

fn _main() {
    App::new().add_systems(Startup, (
        setup,
        debug_projection,
        zoom_orthographic,
        zoom_perspective,
    ));
}
