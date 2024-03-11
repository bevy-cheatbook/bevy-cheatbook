use bevy::prelude::*;

#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Player;
#[derive(Resource)]
struct MyGameSettings;
#[derive(Resource)]
struct MyParticleTracker;
#[derive(Event)]
struct MyDamageEvent;

fn needs_crazy_things() -> bool {
    true
}

fn do_regular_things() {}
fn other_things() {}

// ANCHOR: fn
fn do_crazy_things(world: &mut World) {
    // we can do anything with any data in the Bevy ECS here!
}
// ANCHOR_END: fn

// ANCHOR: systemstate
use bevy::ecs::system::SystemState;

fn spawn_particles_for_enemies(
    world: &mut World,
    // behaves sort of like a query in a regular system
    q_enemies: &mut QueryState<&Transform, With<Enemy>>,
    // emulates a regular system with an arbitrary set of parameters
    params: &mut SystemState<(
        ResMut<MyGameSettings>,
        ResMut<MyParticleTracker>,
        Query<&mut Transform, With<Player>>,
        EventReader<MyDamageEvent>,
        // yes, even Commands ;)
        Commands,
    )>,
    // local resource, just like in a regular system
    mut has_run_once: Local<bool>,
) {
    // note: unlike with a regular Query, we need to provide the world as an argument.
    // The world will only be "locked" for the duration of this loop
    for transform in q_enemies.iter(world) {
        // TODO: do something with the transforms
    }

    // create a scope where we can access our things like a regular system
    {
        let (mut settings, mut tracker, mut q_player, mut evr, commands) =
            params.get_mut(world);

        // TODO: do things with our resources, query, events, commands, ...
    }

    // because our SystemState includes Commands,
    // we must apply them when we are done
    params.apply(world);

    // we are now free to directly spawn entities
    // because the World is no longer used by anything
    // (the SystemState and the QueryState are no longer accessing it)

    world.spawn_batch((0..10000) // efficiently spawn 10000 particles
        .map(|_| SpriteBundle {
            // ...
            ..Default::default()
        })
    );

    // and, of course, we can use our Local
    *has_run_once = true;
}
// ANCHOR_END: systemstate

fn main() {
let mut app = App::new();
// ANCHOR: app
app.add_systems(Update,
    do_crazy_things
        .run_if(needs_crazy_things)
        .after(do_regular_things)
        .before(other_things)
);
// ANCHOR_END: app
}
