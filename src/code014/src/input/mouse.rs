use bevy::prelude::*;

// ANCHOR: mouse-button-input
fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
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
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Middle]) {
        // Either the left or the middle (wheel) button was just pressed
    }
}
// ANCHOR_END: mouse-button-input

// ANCHOR: mouse-button-input-iter
fn mouse_button_iter(
    buttons: Res<ButtonInput<MouseButton>>,
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

    for ev in mousebtn_evr.read() {
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
    mut evr_motion: EventReader<MouseMotion>,
) {
    for ev in evr_motion.read() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}
// ANCHOR_END: mouse-motion

// ANCHOR: cursor-events
fn cursor_events(
    mut evr_cursor: EventReader<CursorMoved>,
) {
    for ev in evr_cursor.read() {
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
    mut evr_scroll: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
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
