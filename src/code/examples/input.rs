use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseButtonInput, MouseWheel};
use bevy::input::keyboard::KeyboardInput;

// ANCHOR: keyboard-input
fn keyboard_input(
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
    }
    if keys.just_released(KeyCode::LControl) {
        // Left Ctrl was released
    }
    if keys.pressed(KeyCode::W) {
        // W is being held down
    }
}
// ANCHOR_END: keyboard-input

// ANCHOR: keyboard-events
fn keyboard_events(
    mut key_evr: EventReader<KeyboardInput>,
) {
    use bevy::input::ElementState;

    for ev in key_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
            }
            ElementState::Released => {
                println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
            }
        }
    }
}
// ANCHOR_END: keyboard-events

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
}
// ANCHOR_END: mouse-button-input

// ANCHOR: mouse-button-events
fn mouse_button_events(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    use bevy::input::ElementState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ElementState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}
// ANCHOR_END: mouse-button-events

// ANCHOR: mouse-motion
fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
) {
    for ev in motion_evr.iter() {
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
            ev.position.x, ev.position.y, ev.id
        );
    }
}
// ANCHOR_END: cursor-events

// ANCHOR: cursor-position
fn cursor_position(
    windows: Res<Windows>,
) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.get_primary().unwrap();

    if let Some(_position) = window.cursor_position() {
        // cursor is inside the window, position given
    } else {
        // cursor is not inside the window
    }
}
// ANCHOR_END: cursor-position

// ANCHOR: scroll-events
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

// ANCHOR: gamepad-connect-disconnect
/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(*id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}
// ANCHOR_END: gamepad-connect-disconnect

// ANCHOR: gamepad-input
fn gamepad_input(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };

    // The joysticks are represented using a separate axis for X and Y

    let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
    let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        // combine X and Y into one vector
        let left_stick_pos = Vec2::new(x, y);

        // implement a dead-zone to ignore small inputs
        if left_stick_pos.length() > 0.1 {
            // do something with the position of the left stick
        }
    }

    let jump_button = GamepadButton(gamepad, GamepadButtonType::South);
    let heal_button = GamepadButton(gamepad, GamepadButtonType::East);

    if buttons.just_pressed(jump_button) {
        // button pressed: make the player jump
    }

    if buttons.pressed(heal_button) {
        // button being held down: heal the player
    }
}
// ANCHOR_END: gamepad-input

// ANCHOR: touches
fn touches(
    touches: Res<Touches>,
) {
    // There is a lot more information available, see the API docs.
    // This example only shows some very basic things.

    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            println!("A new touch with ID {} just began.", finger.id());
        }
        println!(
            "Finger {} is at position ({},{}), started from ({},{}).",
            finger.id(),
            finger.position().x,
            finger.position().y,
            finger.start_position().x,
            finger.start_position().y,
        );
    }
}
// ANCHOR_END: touches

// ANCHOR: touch-events
fn touch_events(
    mut touch_evr: EventReader<TouchInput>,
) {
    use bevy::input::touch::TouchPhase;
    for ev in touch_evr.iter() {
        match ev.phase {
            TouchPhase::Started => {
                println!("Touch {} started at: {:?}", ev.id, ev.position);
            }
            TouchPhase::Moved => {
                println!("Touch {} moved to: {:?}", ev.id, ev.position);
            }
            TouchPhase::Ended => {
                println!("Touch {} ended at: {:?}", ev.id, ev.position);
            }
            TouchPhase::Cancelled => {
                println!("Touch {} cancelled at: {:?}", ev.id, ev.position);
            }
        }
    }
}
// ANCHOR_END: touch-events

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(keyboard_input)
        .add_system(keyboard_events)
        .add_system(mouse_button_input)
        .add_system(mouse_button_events)
        .add_system(mouse_motion)
        .add_system(cursor_events)
        .add_system(cursor_position)
        .add_system(scroll_events)
        .add_system(gamepad_connections)
        .add_system(gamepad_input)
        .add_system(touches)
        .add_system(touch_events)
        .run();
}
