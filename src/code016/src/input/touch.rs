use bevy::input::gamepad::{GamepadSettings, AxisSettings, ButtonSettings};
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseButtonInput, MouseWheel};
use bevy::input::keyboard::KeyboardInput;

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

    for finger in touches.iter_just_released() {
        println!("Touch with ID {} just ended.", finger.id());
    }
    for finger in touches.iter_just_canceled() {
        println!("Touch with ID {} was canceled.", finger.id());
    }
}
// ANCHOR_END: touches

// ANCHOR: touch-events
fn touch_events(
    mut evr_touch: EventReader<TouchInput>,
) {
    use bevy::input::touch::TouchPhase;
    for ev in evr_touch.read() {
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
            TouchPhase::Canceled => {
                println!("Touch {} canceled at: {:?}", ev.id, ev.position);
            }
        }
    }
}
// ANCHOR_END: touch-events
