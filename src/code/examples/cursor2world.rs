use bevy::prelude::*;

// ANCHOR: example
/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Transform, &OrthographicProjection), With<MainCamera>>
) {
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let (camera_transform, ortho_projection) = q_camera.single();

        // undo orthographic scale and apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0 / ortho_projection.scale);
        eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);
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
