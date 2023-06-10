use bevy::prelude::*;

fn particle_effects() {}
fn npc_behaviors() {}
fn enemy_movement() {}
fn player_movement() {}
fn input_handling() {}

// ANCHOR: system-order
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // order doesn't matter for these systems:
        .add_system(particle_effects)
        .add_system(npc_behaviors)
        .add_system(enemy_movement)

        .add_system(input_handling)

        .add_system(
            player_movement
                // `player_movement` must always run before `enemy_movement`
                .before(enemy_movement)
                // `player_movement` must always run after `input_handling`
                .after(input_handling)
        )
        .run();
}
// ANCHOR_END: system-order
