use bevy::prelude::*;

// ANCHOR: grab
use bevy::window::{CursorGrabMode, PrimaryWindow};

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor.visible = false;
}
// ANCHOR_END: grab

// ANCHOR: ungrab
fn cursor_ungrab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;
}
// ANCHOR_END: ungrab

// ANCHOR: recenter
#[cfg(target_os = "windows")]
fn cursor_recenter(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();
    let center = Vec2::new(
        primary_window.width() / 2.0,
        primary_window.height() / 2.0,
    );
    primary_window.set_cursor_position(Some(center));
}
// ANCHOR_END: recenter

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, (cursor_grab, cursor_ungrab));

// ANCHOR: recenter-app
#[cfg(target_os = "windows")]
app.add_systems(Update, cursor_recenter);
// ANCHOR_END: recenter-app
}
