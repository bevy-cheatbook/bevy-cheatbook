use bevy::prelude::*;

fn particle_effects() {}
fn npc_behaviors() {}
fn enemy_movement() {}
fn player_movement() {}
fn input_handling() {}
fn spawn_monsters() {}
fn spawn_zombies() {}
fn spawn_spiders() {}
fn spawn_particles() {}
fn animate_particles() {}
fn debug_particle_statistics() {}

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

    // we can apply ordering to multiple systems at once:
    (
        spawn_monsters,
        spawn_zombies,
        spawn_spiders,
    ).before(enemy_movement),

    // to run a sequence of systems in order, use `.chain()`
    // (this is just syntax sugar to automatically add
    // before/after dependencies between the systems in the tuple)
    (
        spawn_particles,
        animate_particles,
        debug_particle_statistics,
    ).chain()
));
// ANCHOR_END: app
}

