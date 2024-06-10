use bevy::{input::{keyboard::{Key, KeyboardInput}, ButtonState}, prelude::*, window::PrimaryWindow};

// ANCHOR: char
fn text_input(
    mut evr_char: EventReader<KeyboardInput>,
    mut string: Local<String>,
) {
    for ev in evr_char.read() {
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

// ANCHOR: ime
// for this simple example, we will just enable/disable IME mode on mouse click
fn ime_toggle(
    mousebtn: Res<ButtonInput<MouseButton>>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if mousebtn.just_pressed(MouseButton::Left) {
        let mut window = q_window.single_mut();

        // toggle "IME mode"
        window.ime_enabled = !window.ime_enabled;

        // We need to tell the OS the on-screen coordinates where the text will
        // be displayed; for this simple example, let's just use the mouse cursor.
        // In a real app, this might be the position of a UI text field, etc.
        window.ime_position = window.cursor_position().unwrap();
    }
}

fn ime_input(
    mut evr_ime: EventReader<Ime>,
) {
    for ev in evr_ime.read() {
        match ev {
            Ime::Commit { value, .. } => {
                println!("IME confirmed text: {}", value);
            }
            Ime::Preedit { value, cursor, .. } => {
                println!("IME buffer: {:?}, cursor: {:?}", value, cursor);
            }
            Ime::Enabled { .. } => {
                println!("IME mode enabled!");
            }
            Ime::Disabled { .. } => {
                println!("IME mode disabled!");
            }
        }
    }
}
// ANCHOR_END: ime

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (text_input, ime_toggle, ime_input))
        .run();
}
