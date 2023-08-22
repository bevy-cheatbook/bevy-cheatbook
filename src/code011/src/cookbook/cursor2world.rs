use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// ANCHOR: example
fn my_cursor_system(
    // need to get cursor coordinates on the primary window
    window_query: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_query.single();
    let cursor_world_position = window_query
        .single()
        // get the cursor position on the window
        .cursor_position()
        // convert the cursor position into world coordinates
        .and_then(|cursor_position| camera.viewport_to_world(camera_transform, cursor_position))
        .map(|ray| ray.origin.truncate());

    if let Some(pos) = cursor_world_position {
        println!("World coords: {}/{}", pos.x, pos.y);
    }
}
// ANCHOR_END: example

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, my_cursor_system)
        .run();
}
