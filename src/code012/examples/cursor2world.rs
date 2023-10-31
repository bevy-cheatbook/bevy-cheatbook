use bevy::prelude::*;

// ANCHOR: simple
use bevy::window::PrimaryWindow;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.init_resource::<MyWorldCoords>();
    // Make sure to add the marker component when you set up your camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
// ANCHOR_END: simple

// ANCHOR: multiple-windows
use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;

/// We will add this to each camera we want to compute cursor position for.
/// Add the component to the camera that renders to each window.
#[derive(Component, Default)]
struct WorldCursorCoords(Vec2);

fn setup_multiwindow(mut commands: Commands) {
    // TODO: set up multiple cameras for multiple windows.
    // See bevy's example code for how to do that.

    // Make sure we add our component to each camera
    commands.spawn((Camera2dBundle::default(), WorldCursorCoords::default()));
}

fn my_cursor_system_multiwindow(
    // query to get the primary window
    q_window_primary: Query<&Window, With<PrimaryWindow>>,
    // query to get other windows
    q_window: Query<&Window>,
    // query to get camera transform
    mut q_camera: Query<(&Camera, &GlobalTransform, &mut WorldCursorCoords)>,
) {
    for (camera, camera_transform, mut worldcursor) in &mut q_camera {
        // get the window the camera is rendering to
        let window = match camera.target {
            // the camera is rendering to the primary window
            RenderTarget::Window(WindowRef::Primary) => {
                q_window_primary.single()
            },
            // the camera is rendering to some other window
            RenderTarget::Window(WindowRef::Entity(e_window)) => {
                q_window.get(e_window).unwrap()
            },
            // the camera is rendering to something else (like a texture), not a window
            _ => {
                // skip this camera
                continue;
            }
        };

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            worldcursor.0 = world_position;
        }
    }
}
// ANCHOR_END: multiple-windows

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_multiwindow)
        .add_systems(Update, my_cursor_system)
        .add_systems(Update, my_cursor_system_multiwindow)
        .run();
}
