use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseButtonInput, MouseWheel};
use bevy::input::keyboard::KeyboardInput;

// ANCHOR: res-input
fn my_simple_system(keys: Res<Input<KeyCode>>, btns: Res<Input<MouseButton>>) {
    // Keyboard input
    if keys.pressed(KeyCode::Space) {
        // space is being held down
    }

    // Mouse buttons
    if btns.just_pressed(MouseButton::Left) {
        // a left click just happened
    }
}
// ANCHOR_END: res-input

// ANCHOR: event-input
fn my_fancy_system(
    mut evr_keys: EventReader<KeyboardInput>,
    mut evr_cursor: EventReader<CursorMoved>,
    mut evr_motion: EventReader<MouseMotion>,
    mut evr_mousebtn: EventReader<MouseButtonInput>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut evr_touch: EventReader<TouchInput>,
) {
    // Keyboard input
    for ev in evr_keys.iter() {
        if ev.state.is_pressed() {
            eprintln!("Just pressed key: {:?}", ev.key_code);
        } else {
            eprintln!("Just released key: {:?}", ev.key_code);
        }
    }

    // Absolute cursor position (in window coordinates)
    for ev in evr_cursor.iter() {
        eprintln!("Cursor at: {}", ev.position);
    }

    // Relative mouse motion
    for ev in evr_motion.iter() {
        eprintln!("Mouse moved {} pixels", ev.delta);
    }

    // Mouse buttons
    for ev in evr_mousebtn.iter() {
        if ev.state.is_pressed() {
            eprintln!("Just pressed mouse button: {:?}", ev.button);
        } else {
            eprintln!("Just released mouse button: {:?}", ev.button);
        }
    }

    // scrolling (mouse wheel, touchpad, etc.)
    for ev in evr_scroll.iter() {
        eprintln!("Scrolled vertically by {} and horizontally by {}.", ev.y, ev.x);
    }

    // touch input
    for ev in evr_touch.iter() {
        eprintln!("Touch {} in {:?} at {}.", ev.id, ev.phase, ev.position);
    }
}
// ANCHOR_END: event-input

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(my_simple_system.system())
        .add_system(my_fancy_system.system())
        .run();
}
