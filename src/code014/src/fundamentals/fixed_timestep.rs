use bevy::prelude::*;

fn camera_movement() {}
fn animation() {}
fn juicy_explosions() {}
fn physics_collisions() {}
fn enemy_ai() {}
fn gameplay_simulation() {}

fn main() {
let mut app = App::new();
// ANCHOR: basic
// These systems will run every frame
// (at the framerate being rendered to your screen)
app.add_systems(Update, (
    camera_movement,
    animation,
    juicy_explosions,
));

// These systems will run as many times as needed
// as to maintain a fixed rate on average
app.add_systems(FixedUpdate, (
    physics_collisions,
    enemy_ai,
    gameplay_simulation,
));
// ANCHOR_END: basic
// ANCHOR: configure
// Set the Fixed Timestep interval to 96 Hz
app.insert_resource(Time::<Fixed>::from_hz(96.0));

// Set the Fixed Timestep interval to 250 milliseconds
app.insert_resource(Time::<Fixed>::from_seconds(0.25));
// ANCHOR_END: configure
}

// ANCHOR: time
fn print_time_delta(time: Res<Time>) {
    // If we add this system to `Update`, this will print the time delta
    // between subsequent frames (the display frame rate)

    // If we add this system to `FixedUpdate`, this will always print the
    // same value (equal to the fixed timestep interval).

    println!("Elapsed seconds: {}", time.delta_seconds());
}

// This system will access the Fixed time
// regardless of what schedule it runs in
fn print_fixed_time_info(time_fixed: Res<Time<Fixed>>) {
    // `Time<Fixed>` gives us some additional methods, such as checking
    // the overstep (partial timestep / amount of extra time accumulated)
    println!(
        "Time remaining until the next fixed update run: {}",
        time_fixed.delta_seconds() - time_fixed.overstep().as_secs_f32()
    );
}

// This system will access the regular frame time regardless
// of what schedule it runs in
fn check_virtual_time(time_fixed: Res<Time<Virtual>>) {
    // ...
}
// ANCHOR_END: time
