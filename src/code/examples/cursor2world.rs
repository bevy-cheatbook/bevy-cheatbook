use bevy::prelude::*;

// ANCHOR: example
/// Used to help identify our main camera
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn my_cursor_system(
    // events to get cursor position
    mut evr_cursor: EventReader<CursorMoved>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>
) {
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = q_camera.iter().next().unwrap();

    for ev in evr_cursor.iter() {
        // get the size of the window that the event is for
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = ev.position - size / 2.0;

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);
    }
}
// ANCHOR_END: example

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(my_cursor_system.system())
        .run();
}
