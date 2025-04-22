use bevy::prelude::*;

// ANCHOR: mouse-button-input
fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}
// ANCHOR_END: mouse-button-input

// ANCHOR: mouse-button-input-iter
fn mouse_button_iter(
    buttons: Res<Input<MouseButton>>,
) {
    for button in buttons.get_pressed() {
        println!("{:?} is currently held down", button);
    }
    for button in buttons.get_just_pressed() {
        println!("{:?} was pressed", button);
    }
    for button in buttons.get_just_released() {
        println!("{:?} was released", button);
    }
}
// ANCHOR_END: mouse-button-input-iter

// ANCHOR: mouse-button-events
use bevy::input::mouse::MouseButtonInput;

fn mouse_button_events(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}
// ANCHOR_END: mouse-button-events

// ANCHOR: mouse-motion
use bevy::input::mouse::MouseMotion;

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
) {
    for ev in motion_evr.read() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}
// ANCHOR_END: mouse-motion

// ANCHOR: cursor-events
fn cursor_events(
    mut cursor_evr: EventReader<CursorMoved>,
) {
    for ev in cursor_evr.iter() {
        println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.window
        );
    }
}
// ANCHOR_END: cursor-events

// ANCHOR: cursor-position
use bevy::window::PrimaryWindow;

fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}
// ANCHOR_END: cursor-position

// ANCHOR: scroll-events
use bevy::input::mouse::MouseWheel;

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }
}
// ANCHOR_END: scroll-events

// ANCHOR: touchpad-gesture-events
use bevy::input::touchpad::{TouchpadMagnify, TouchpadRotate};

// these only work on macOS
fn touchpad_gestures(
    mut evr_touchpad_magnify: EventReader<TouchpadMagnify>,
    mut evr_touchpad_rotate: EventReader<TouchpadRotate>,
) {
    for ev_magnify in evr_touchpad_magnify.iter() {
        // Positive numbers are zooming in
        // Negative numbers are zooming out
        println!("Touchpad zoom by {}", ev_magnify.0);
    }
    for ev_rotate in evr_touchpad_rotate.iter() {
        // Positive numbers are anticlockwise
        // Negative numbers are clockwise
        println!("Touchpad rotate by {}", ev_rotate.0);
    }
}
// ANCHOR_END: touchpad-gesture-events

fn handle_middleclick() {}

fn handle_drag() {}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Update, (
        mouse_button_input,
        mouse_button_iter,
        mouse_button_events,
        mouse_motion,
        cursor_events,
        cursor_position,
        scroll_events,
        touchpad_gestures,
    ));

// ANCHOR: run-conditions
use bevy::input::common_conditions::*;

app.add_systems(Update, (
    handle_middleclick
        .run_if(input_just_pressed(MouseButton::Middle)),
    handle_drag
        .run_if(input_pressed(MouseButton::Left)),
));
// ANCHOR_END: run-conditions

    app.run();
}
