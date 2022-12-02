use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

// ANCHOR: example
/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn my_cursor_system(
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_position) = window.cursor_position() {
        // check if it is possible to create a ray from screen coordinates to world coordinates
        if let Some(ray) = camera.viewport_to_world(camera_transform, screen_position) {
            // get 2d world mouse coordinates from the ray
            let world_position: Vec2 = ray.origin.truncate();

            eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        }
    }
}
// ANCHOR_END: example

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(my_cursor_system)
        .run();
}
