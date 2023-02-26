use bevy::prelude::*;

#[derive(Component)]
struct MyGameCamera;

// ANCHOR: zoom
fn zoom_2d(
    mut q: Query<&mut OrthographicProjection, With<MyGameCamera>>,
) {
    let mut projection = q.single_mut();

    // example: zoom in
    projection.scale *= 1.25;
    // example: zoom out
    projection.scale *= 0.75;

    // always ensure you end up with sane values
    // (pick an upper and lower bound for your application)
    projection.scale = projection.scale.clamp(0.5, 5.0);
}
// ANCHOR_END: zoom

fn main() {
    App::new()
        .add_system(zoom_2d);
}
