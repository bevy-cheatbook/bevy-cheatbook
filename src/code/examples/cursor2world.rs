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
    primary_window: Query<(&Window, &PrimaryWindow)>,
    all_windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window: &Window = if let RenderTarget::Window(window_ref) = camera.target {
        match window_ref {
            bevy::window::WindowRef::Entity(id) => all_windows.get(id).unwrap(),
            bevy::window::WindowRef::Primary => primary_window.single().0,
        }
    } else {
        primary_window.single().0
    };

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
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
