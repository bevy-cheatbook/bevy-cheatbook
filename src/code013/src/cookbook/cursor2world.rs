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

// ANCHOR: 3d-ground-plane
/// Here we will store the position of the mouse cursor on the 3D ground plane.
#[derive(Resource, Default)]
struct MyGroundCoords {
    // Global (world-space) coordinates
    global: Vec3,
    // Local (relative to the ground plane) coordinates
    local: Vec2,
}

/// Used to help identify our main camera
#[derive(Component)]
struct MyGameCamera;

/// Used to help identify our ground plane
#[derive(Component)]
struct MyGroundPlane;

fn setup_3d_scene(mut commands: Commands) {
    // Make sure to add the marker component when you set up your camera
    commands.spawn((
        MyGameCamera,
        Camera3dBundle {
            // ... your camera configuration ...
            ..default()
        },
    ));
    // Spawn the ground
    commands.spawn((
        MyGroundPlane,
        PbrBundle {
            // feel free to change this to rotate/tilt or reposition the ground
            transform: Transform::default(),
            // TODO: set up your mesh / visuals for rendering:
            // mesh: ...
            // material: ...
            ..default()
        },
    ));
}

fn cursor_to_ground_plane(
    mut mycoords: ResMut<MyGroundCoords>,
    // query to get the window (so we can read the current cursor position)
    // (we will only work with the primary window)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MyGameCamera>>,
    // query to get ground plane's transform
    q_plane: Query<&GlobalTransform, With<MyGroundPlane>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // Ditto for the ground plane's transform
    let ground_transform = q_plane.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    let Some(cursor_position) = window.cursor_position() else {
        // if the cursor is not inside the window, we can't do anything
        return;
    };

    // Mathematically, we can represent the ground as an infinite flat plane.
    // To do that, we need a point (to position the plane) and a normal vector
    // (the "up" direction, perpendicular to the ground plane).

    // We can get the correct values from the ground entity's GlobalTransform
    let plane_origin = ground_transform.translation();
    let plane = Plane3d::new(ground_transform.up());

    // Ask Bevy to give us a ray pointing from the viewport (screen) into the world
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        // if it was impossible to compute for whatever reason; we can't do anything
        return;
    };

    // do a ray-plane intersection test, giving us the distance to the ground
    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        // If the ray does not intersect the ground
        // (the camera is not looking towards the ground), we can't do anything
        return;
    };

    // use the distance to compute the actual point on the ground in world-space
    let global_cursor = ray.get_point(distance);

    mycoords.global = global_cursor;
    eprintln!("Global cursor coords: {}/{}/{}",
        global_cursor.x, global_cursor.y, global_cursor.z
    );

    // to compute the local coordinates, we need the inverse of the plane's transform
    let inverse_transform_matrix = ground_transform.compute_matrix().inverse();
    let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);

    // we can discard the Y coordinate, because it should always be zero
    // (our point is supposed to be on the plane)
    mycoords.local = local_cursor.xz();
    eprintln!("Local cursor coords: {}/{}", local_cursor.x, local_cursor.z);
}
// ANCHOR_END: 3d-ground-plane

fn main() {
    let mut app = App::new();
// ANCHOR: simple-app
app.init_resource::<MyWorldCoords>();
app.add_systems(Startup, setup);
app.add_systems(Update, my_cursor_system);
// ANCHOR_END: simple-app

// ANCHOR: multiple-windows-app
app.add_systems(Startup, setup_multiwindow);
app.add_systems(Update, my_cursor_system_multiwindow);
// ANCHOR_END: multiple-windows-app

// ANCHOR: 3d-ground-plane-app
app.init_resource::<MyGroundCoords>();
app.add_systems(Startup, setup_3d_scene);
app.add_systems(Update, cursor_to_ground_plane);
// ANCHOR_END: 3d-ground-plane-app
}
