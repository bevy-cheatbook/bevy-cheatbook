#![allow(unused_variables)]

use bevy::input::gamepad::{GamepadSettings, AxisSettings, ButtonSettings};
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
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        // Either the left or right shift are being held down
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        // Either delete or backspace was just pressed
    }
}
// ANCHOR_END: keyboard-input

// ANCHOR: keyboard-events
fn keyboard_events(
    mut key_evr: EventReader<KeyboardInput>,
) {
    use bevy::input::ButtonState;

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
            }
            ButtonState::Released => {
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
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}
// ANCHOR_END: mouse-button-input

// ANCHOR: mouse-button-events
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
    for ev in gamepad_evr.iter() {
        // the ID of the gamepad
        let id = ev.gamepad;
        match ev.event_type {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == id {
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
    let axis_lx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickX
    };
    let axis_ly = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickY
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        // combine X and Y into one vector
        let left_stick_pos = Vec2::new(x, y);

        // Example: check if the stick is pushed up
        if left_stick_pos.length() > 0.9 && left_stick_pos.y > 0.5 {
            // do something
        }
    }

    // In a real game, the buttons would be configurable, but here we hardcode them
    let jump_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::South
    };
    let heal_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::East
    };

    if buttons.just_pressed(jump_button) {
        // button just pressed: make the player jump
    }

    if buttons.pressed(heal_button) {
        // button being held down: heal the player
    }
}
// ANCHOR_END: gamepad-input

// ANCHOR: gamepad-input-events
fn gamepad_input_events(
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };

    for ev in gamepad_evr.iter() {
        if ev.gamepad != gamepad {
            // event not from our gamepad
            continue;
        }

        use GamepadEventType::{AxisChanged, ButtonChanged};

        match ev.event_type {
            AxisChanged(GamepadAxisType::RightStickX, x) => {
                // Right Stick moved (X)
            }
            AxisChanged(GamepadAxisType::RightStickY, y) => {
                // Right Stick moved (Y)
            }
            ButtonChanged(GamepadButtonType::DPadDown, val) => {
                // buttons are also reported as analog, so use a threshold
                if val > 0.5 {
                    // button pressed
                }
            }
            _ => {} // don't care about other inputs
        }
    }
}
// ANCHOR_END: gamepad-input-events

fn gamepad_print_allevents(
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.iter() {
        match ev.event_type {
            GamepadEventType::Connected => println!("Gamepad {:?}: Connected", ev.gamepad),
            GamepadEventType::Disconnected => println!("Gamepad {:?}: Disconnected", ev.gamepad),
            GamepadEventType::ButtonChanged(button, val) => {
                println!("Gamepad {:?}: {:?} changed: {}", ev.gamepad, button, val);
            },
            GamepadEventType::AxisChanged(axis, val) => {
                println!("Gamepad {:?}: {:?} changed: {}", ev.gamepad, axis, val);
            },
        }
    }
}

// ANCHOR: gamepad-settings
// this should be run once, when the game is starting
// (transition entering your in-game state might be a good place to put it)
fn configure_gamepads(
    my_gamepad: Option<Res<MyGamepad>>,
    mut settings: ResMut<GamepadSettings>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };

    // add a larger default dead-zone to all axes (ignore small inputs, round to zero)
    settings.default_axis_settings.negative_low = -0.1;
    settings.default_axis_settings.positive_low = 0.1;

    // make the right stick "binary", squash higher values to 1.0 and lower values to 0.0
    let right_stick_settings = AxisSettings {
        positive_high:  0.5, // values  0.5 to  1.0, become  1.0
        positive_low:   0.5, // values  0.0 to  0.5, become  0.0
        negative_low:  -0.5, // values -0.5 to  0.0, become  0.0
        negative_high: -0.5, // values -1.0 to -0.5, become -1.0
        // the raw value should change by at least this much,
        // for Bevy to register an input event:
        threshold: 0.01,
    };

    // make the triggers work in big/coarse steps, to get fewer events
    // reduces noise and precision
    let trigger_settings = AxisSettings {
        threshold: 0.2,
        // also set some conservative deadzones
        positive_high: 0.8,
        positive_low: 0.2,
        negative_high: -0.8,
        negative_low: -0.2,
    };

    // set these settings for the gamepad we use for our player
    settings.axis_settings.insert(
        GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickX },
        right_stick_settings.clone()
    );
    settings.axis_settings.insert(
        GamepadAxis { gamepad, axis_type: GamepadAxisType::RightStickY },
        right_stick_settings.clone()
    );
    settings.axis_settings.insert(
        GamepadAxis { gamepad, axis_type: GamepadAxisType::LeftZ },
        trigger_settings.clone()
    );
    settings.axis_settings.insert(
        GamepadAxis { gamepad, axis_type: GamepadAxisType::RightZ },
        trigger_settings.clone()
    );

    // for buttons (or axes treated as buttons), make them less sensitive
    let button_settings = ButtonSettings {
        // require them to be pressed almost all the way, to count
        press: 0.9,
        // require them to be released almost all the way, to count
        release: 0.1,
    };

    settings.default_button_settings = button_settings;
}
// ANCHOR_END: gamepad-settings

// ANCHOR: text
/// prints every char coming in; press enter to echo the full string
fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
) {
    for ev in char_evr.iter() {
        println!("Got char: '{}'", ev.char);
        string.push(ev.char);
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", *string);
        string.clear();
    }
}
// ANCHOR_END: text

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
        // in real apps you probably want to store and track touch ids somewhere
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

// ANCHOR: dnd-file
#[derive(Component)]
struct MyDropTarget;

fn file_drop(
    mut dnd_evr: EventReader<FileDragAndDrop>,
    query_ui_droptarget: Query<&Interaction, With<MyDropTarget>>,
) {
    for ev in dnd_evr.iter() {
        println!("{:?}", ev);
        if let FileDragAndDrop::DroppedFile { id, path_buf } = ev {
            println!("Dropped file with path: {:?}", path_buf);

            if id.is_primary() {
                // it was dropped over the main window
            }

            for interaction in query_ui_droptarget.iter() {
                if *interaction == Interaction::Hovered {
                    // it was dropped over our UI element
                    // (our UI element is being hovered over)
                }
            }
        }
    }
}
// ANCHOR_END: dnd-file

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(configure_gamepads)
        .add_system(file_drop)
        .add_system(text_input)
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
        .add_system(gamepad_input_events)
        .add_system(gamepad_print_allevents)
        .add_system(touches)
        .add_system(touch_events)
        .run();
}
