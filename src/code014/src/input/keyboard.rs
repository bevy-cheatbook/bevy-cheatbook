use bevy::prelude::*;

// ANCHOR: res
fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
    }
    if keys.just_released(KeyCode::ControlLeft) {
        // Left Ctrl was released
    }
    if keys.pressed(KeyCode::KeyW) {
        // W is being held down
    }
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        // Either the left or right shift are being held down
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Backspace]) {
        // Either delete or backspace was just pressed
    }
}
// ANCHOR_END: res

// ANCHOR: res-iter
fn keyboard_iter(
    keys: Res<ButtonInput<KeyCode>>,
) {
    for key in keys.get_pressed() {
        println!("{:?} is currently held down", key);
    }
    for key in keys.get_just_pressed() {
        println!("{:?} was pressed", key);
    }
    for key in keys.get_just_released() {
        println!("{:?} was released", key);
    }
}
// ANCHOR_END: res-iter

// ANCHOR: events
fn keyboard_events(
    mut evr_kbd: EventReader<KeyboardInput>,
) {
    for ev in evr_kbd.read() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({:?})", ev.key_code, ev.logical_key);
            }
            ButtonState::Released => {
                println!("Key release: {:?} ({:?})", ev.key_code, ev.logical_key);
            }
        }
    }
}
// ANCHOR_END: events

// ANCHOR: focus-lost
use bevy::input::keyboard::KeyboardFocusLost;

fn detect_special_sequence(
    mut evr_focus_lost: EventReader<KeyboardFocusLost>,
    mut remembered_keys: Local<Vec<KeyCode>>,
) {
    // Imagine we need to remeber a sequence of keypresses
    // for some special gameplay reason.
    // TODO: implement that; store state in `remembered_keys`

    // But it might go wrong if the user Alt-Tabs, we need to reset
    if !evr_focus_lost.is_empty() {
        remembered_keys.clear();
        evr_focus_lost.clear();
    }
}
// ANCHOR_END: focus-lost

// ANCHOR: char
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};

fn text_input(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut string: Local<String>,
) {
    for ev in evr_kbd.read() {
        // We don't care about key releases, only key presses
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            // Handle pressing Enter to finish the input
            Key::Enter => {
                println!("Text input: {}", &*string);
                string.clear();
            }
            // Handle pressing Backspace to delete last char
            Key::Backspace => {
                string.pop();
            }
            // Handle key presses that produce text characters
            Key::Character(input) => {
                // Ignore any input that contains control (special) characters
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                string.push_str(&input);
            }
            _ => {}
        }
    }
}
// ANCHOR_END: char

fn handle_jump() {}
fn handle_shooting() {}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Update, keyboard_input);
    app.add_systems(Update, keyboard_iter);
    app.add_systems(Update, keyboard_events);
    app.add_systems(Update, text_input);

// ANCHOR: run-conditions
use bevy::input::common_conditions::*;

app.add_systems(Update, (
    handle_jump
        .run_if(input_just_pressed(KeyCode::Space)),
    handle_shooting
        .run_if(input_pressed(KeyCode::Enter)),
));
// ANCHOR_END: run-conditions

    app.run();
}
