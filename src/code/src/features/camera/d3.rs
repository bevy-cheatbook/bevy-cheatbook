use bevy::prelude::*;

#[derive(Component)]
struct MyGameCamera;

// ANCHOR: zoom
fn zoom_3d(
    mut q: Query<&mut PerspectiveProjection, With<MyGameCamera>>,
) {
    let mut projection = q.single_mut();

    // example: zoom in
    projection.fov *= 1.25;
    // example: zoom out
    projection.fov *= 0.75;

    // always ensure you end up with sane values
    // (pick an upper and lower bound for your application)
    projection.fov = projection.fov.clamp(30.0f32.to_radians(), 90.0f32.to_radians());
}
// ANCHOR_END: zoom

fn main() {
    App::new()
        .add_system(zoom_3d);
}
