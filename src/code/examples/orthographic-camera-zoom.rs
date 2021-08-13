use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};

fn main() {
    App::build().add_system(zoom_system.system()).run();
}

fn zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    window: Res<WindowDescriptor>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection, &mut Camera)>,
) {
    for event in mouse_wheel_events.iter() {
        for (mut transform, mut projection, mut camera) in query.iter_mut() {
            projection.scale -= event.y;
            if projection.scale <= 0.1 {
                projection.scale = 0.1;
            }
            projection.update(window.width, window.height);
            camera.projection_matrix = projection.get_projection_matrix();
        }
    }
}
