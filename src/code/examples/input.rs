#![allow(unused_variables)]

use bevy::input::gamepad::{GamepadSettings, AxisSettings, ButtonSettings};
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseButtonInput, MouseWheel};
use bevy::input::keyboard::KeyboardInput;

// ANCHOR: gamepad-connect-disconnect
/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
#[derive(Resource)]
struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.iter() {
        // the ID of the gamepad
        let id = ev.gamepad;
        match &ev.event_type {
            GamepadEventType::Connected(info) => {
                println!("New gamepad connected with ID: {:?}, name: {}", id, info.name);

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
        match &ev.event_type {
            GamepadEventType::Connected(info) => println!("Gamepad {:?}: Connected ({})", ev.gamepad, info.name),
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
    settings.default_axis_settings.set_deadzone_lowerbound(-0.1);
    settings.default_axis_settings.set_deadzone_upperbound(0.1);

    // make the right stick "binary", squash higher values to 1.0 and lower values to 0.0
    let mut right_stick_settings = AxisSettings::default();
    right_stick_settings.set_deadzone_lowerbound(-0.5);
    right_stick_settings.set_deadzone_upperbound(0.5);
    right_stick_settings.set_livezone_lowerbound(-0.5);
    right_stick_settings.set_livezone_upperbound(0.5);
    // the raw value should change by at least this much,
    // for Bevy to register an input event:
    right_stick_settings.set_threshold(0.01);

    // make the triggers work in big/coarse steps, to get fewer events
    // reduces noise and precision
    let mut trigger_settings = AxisSettings::default();
    trigger_settings.set_threshold(0.25);

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

    // for buttons (or axes treated as buttons):
    let mut button_settings = ButtonSettings::default();
    // require them to be pressed almost all the way, to count
    button_settings.set_press_threshold(0.9);
    // require them to be released almost all the way, to count
    button_settings.set_release_threshold(0.1);

    settings.default_button_settings = button_settings;
}
// ANCHOR_END: gamepad-settings

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
