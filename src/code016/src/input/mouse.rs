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
    // Games typically only have one window (the primary window)
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = window.cursor_position() {
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

// ANCHOR: enter-leave
// Check if cursor enters / leaves primary window
fn enter_leave_primary(
    mut evr_leave: EventReader<CursorLeft>,
    mut evr_enter: EventReader<CursorEntered>,
    // To obtain the Entity id of the primary window:
    e_window: Single<Entity, With<PrimaryWindow>>,
) {
    if evr_leave.read().any(|ev| ev.window == *e_window) {
        // cursor left primary window
    }
    if evr_enter.read().any(|ev| ev.window == *e_window) {
        // cursor entered primary window
    }
}

// Track which window (in a multi-window app) the cursor is in
fn current_cursor_window(
    mut evr_leave: EventReader<CursorLeft>,
    mut evr_enter: EventReader<CursorEntered>,
    // Here we can keep track which window the cursor is currently in.
    mut current_window: Local<Option<Entity>>,
) {
    if !evr_leave.is_empty() {
        evr_leave.clear();
        *current_window = None;
    }
    if let Some(ev) = evr_enter.read().last() {
        *current_window = Some(ev.window);
    }
}
// ANCHOR_END: enter-leave

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
