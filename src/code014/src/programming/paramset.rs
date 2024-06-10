use bevy::prelude::*;

#[derive(Component)]
struct Health {
    hp: f32,
}
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Player;
#[derive(Resource)]
struct MyResource;

mod conflict {
use super::*;
// ANCHOR: conflict
fn reset_health(
    mut q_player: Query<&mut Health, With<Player>>,
    mut q_enemy: Query<&mut Health, With<Enemy>>,
) {
    // ...
}
// ANCHOR_END: conflict
}

// ANCHOR: paramset
fn reset_health(
    // access the health of enemies and the health of players
    // (note: some entities could be both!)
    mut set: ParamSet<(
        Query<&mut Health, With<Enemy>>,
        Query<&mut Health, With<Player>>,
        // also access the whole world ... why not
        &World,
    )>,
) {
    // set health of enemies (use the 1st param in the set)
    for mut health in set.p0().iter_mut() {
        health.hp = 50.0;
    }

    // set health of players (use the 2nd param in the set))
    for mut health in set.p1().iter_mut() {
        health.hp = 100.0;
    }

    // read some data from the world (use the 3rd param in the set)
    let my_resource = set.p2().resource::<MyResource>();
}
// ANCHOR_END: paramset

fn _main() {
    App::new()
        .add_systems(Update, reset_health);
}
