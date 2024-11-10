use bevy::prelude::*;
use bevy::utils::Duration;

fn plugin(app: &mut App) {
    app.add_systems(Update, (
        gamepad_connections,
        gamepad_input,
        gamepad_input_events,
        gamepad_rumble,
        list_gamepads,
    ));
// ANCHOR: gamepad-settings-app
app.add_systems(Update,
    configure_gamepads
        .run_if(resource_exists_and_changed::<MyGamepad>)
);
// ANCHOR_END: gamepad-settings-app
}

// ANCHOR: gamepads
fn list_gamepads(
    gamepads: Res<Gamepads>,
) {
    println!("Currently connected gamepads:");
    for gamepad in gamepads.iter() {
        println!(
            "ID: {:?}; Name: {}",
            gamepad, gamepads.name(gamepad).unwrap_or("unknown")
        );
    }
}
// ANCHOR_END: gamepads

// ANCHOR: gamepad-connect-disconnect
use bevy::input::gamepad::{GamepadConnection, GamepadEvent};

/// Simple resource to store the ID of the first connected gamepad.
/// We can use it to know which gamepad to use for player input.
#[derive(Resource)]
struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        // we only care about connection events
        let GamepadEvent::Connection(ev_conn) = ev else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected(info) => {
                debug!(
                    "New gamepad connected: {:?}, name: {}",
                    ev_conn.gamepad, info.name,
                );
                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(ev_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                debug!("Lost connection with gamepad: {:?}", ev_conn.gamepad);
                // if it's the one we previously used for the player, remove it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == ev_conn.gamepad {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}
// ANCHOR_END: gamepad-connect-disconnect

// ANCHOR: gamepad-input
fn gamepad_input(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
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
        let left_stick = Vec2::new(x, y);

        // Example: check if the stick is pushed up
        if left_stick.length() > 0.9 && left_stick.y > 0.5 {
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
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        match ev {
            GamepadEvent::Axis(ev_axis) => {
                println!(
                    "Axis {:?} on gamepad {:?} is now at {:?}",
                    ev_axis.axis_type, ev_axis.gamepad, ev_axis.value
                );
            }
            GamepadEvent::Button(ev_button) => {
                // The "value" of a button is typically `0.0` or `1.0`, but it
                // is a `f32` because some gamepads may have buttons that are
                // pressure-sensitive or otherwise analog somehow.
                println!(
                    "Button {:?} on gamepad {:?} is now at {:?}",
                    ev_button.button_type, ev_button.gamepad, ev_button.value
                );
            }
            _ => {
                // we don't care about other events here (connect/disconnect)
            }
        }
    }
}
// ANCHOR_END: gamepad-input-events

// ANCHOR: gamepad-settings
use bevy::input::gamepad::{AxisSettings, ButtonSettings, GamepadSettings};

fn configure_gamepads(
    my_gamepad: Option<Res<MyGamepad>>,
    mut settings: ResMut<GamepadSettings>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
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

// ANCHOR: gamepad-rumble
use bevy::input::gamepad::{GamepadRumbleIntensity, GamepadRumbleRequest};

fn gamepad_rumble(
    mut evw_rumble: EventWriter<GamepadRumbleRequest>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    // add a short 100ms rumble at max intensity
    evw_rumble.send(GamepadRumbleRequest::Add {
        gamepad,
        duration: Duration::from_millis(100),
        intensity: GamepadRumbleIntensity::MAX,
    });

    // also rumble for a little longer (500 ms)
    // with the weak motor at half intensity
    // and the strong motor at quarter intensity
    evw_rumble.send(GamepadRumbleRequest::Add {
        gamepad,
        duration: Duration::from_millis(500),
        intensity: GamepadRumbleIntensity {
            strong_motor: 0.25,
            weak_motor: 0.5,
        },
    });
}
// ANCHOR_END: gamepad-rumble
