use bevy::prelude::*;

fn particle_effects() {}
fn npc_behaviors() {}
fn enemy_movement() {}
fn player_movement() {}
fn input_handling() {}

// ANCHOR: app
fn main() {
let mut app = App::new();
app.add_systems(Update, (
    enemy_movement,
    input_handling,

    player_movement
        // `player_movement` must always run before `enemy_movement`
        .before(enemy_movement)
        // `player_movement` must always run after `input_handling`
        .after(input_handling),

    // order doesn't matter for some systems:
    particle_effects,
    npc_behaviors,
));
// ANCHOR_END: app
}

