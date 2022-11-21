use bevy::prelude::*;

// ANCHOR: example
use bevy::window::CursorGrabMode;

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        // if you want to use the cursor, but not let it leave the window,
        // use `Confined` mode:
        window.set_cursor_grab_mode(CursorGrabMode::Confined);

        // for a game that doesn't use the cursor (like a shooter):
        // use `Locked` mode to keep the cursor in one place
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        // also hide the cursor
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }
}
// ANCHOR_END: example

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(cursor_grab_system)
        .run();
}
