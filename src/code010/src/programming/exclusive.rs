use bevy::prelude::*;

fn system_a() {}
fn system_b() {}

// ANCHOR: exclusive-fn
fn do_crazy_things(world: &mut World) {
    // we can do anything with any data in the Bevy ECS here!
}
// ANCHOR_END: exclusive-fn

// ANCHOR: command-flush
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        .add_systems(
            (
                // This system produces some commands
                system_a,
                // This will apply the queued commands from system_a
                apply_system_buffers,
                // This system will have access to the results of
                // system_a's commands
                system_b,
            ).chain() // Ensure the order of these systems is maintained.
        )

        .run();
}
// ANCHOR_END: command-flush