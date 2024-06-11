use bevy::prelude::*;

// ANCHOR: platform-gesture-events
use bevy::input::gestures::{
    DoubleTapGesture, PanGesture, PinchGesture, RotationGesture
};

// these only work on macOS and iOS
fn builtin_gestures(
    mut evr_gesture_pinch: EventReader<PinchGesture>,
    mut evr_gesture_rotate: EventReader<RotationGesture>,
    mut evr_gesture_pan: EventReader<PanGesture>,
    mut evr_gesture_doubletap: EventReader<PanGesture>,
) {
    for ev_pinch in evr_gesture_pinch.read() {
        // Positive numbers are zooming in
        // Negative numbers are zooming out
        println!("Two-finger zoom by {}", ev_pinch.0);
    }
    for ev_rotate in evr_gesture_rotate.read() {
        // Positive numbers are anticlockwise
        // Negative numbers are clockwise
        println!("Two-finger rotate by {}", ev_rotate.0);
    }
    for ev_pan in evr_gesture_pan.read() {
        // Each event is a Vec2 giving you the X/Y pan amount
        println!("Two-finger pan by X: {}, Y: {}", ev_pan.0.x, ev_pan.0.y);
    }
    for ev_doubletap in evr_gesture_doubletap.read() {
        // This one has no data
        println!("Double-Tap gesture!");
    }
}
// ANCHOR_END: platform-gesture-events

fn main() {
    let mut app = App::new();
    app.add_systems(Update, builtin_gestures);
}
