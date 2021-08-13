use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    render::camera::{Camera, CameraProjection, OrthographicProjection},
};

fn main() {
    App::build()
        .add_startup_system(startup.system())
        .add_system(zoom_system.system())
        .run();
}

fn startup(mut commands: Commands) {
    let camera_bundle = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera_bundle);
}

fn zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    window: Res<WindowDescriptor>,
    mut query: Query<(&mut OrthographicProjection, &mut Camera)>,
) {
    for event in mouse_wheel_events.iter() {
        for (mut projection, mut camera) in query.iter_mut() {
            projection.scale -= event.y;
            if projection.scale <= 1.0 {
                // The scale can also be between 0-1, but it's easier to
                // demonstrate when limited to 1.
                projection.scale = 1.0;
            }
            projection.update(window.width, window.height);
            camera.projection_matrix = projection.get_projection_matrix();
        }
    }
}
