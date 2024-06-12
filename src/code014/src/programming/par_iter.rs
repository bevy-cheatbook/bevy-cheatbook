use bevy::{ecs::batching::BatchingStrategy, prelude::*};

fn plugin(app: &mut App) {
    app.add_systems(Update, (
        my_particle_physics,
        my_particle_timers,
        handle_many_events,
        par_iter_custom_batch_size,
    ));
}

#[derive(Event)]
struct MyEvent;

#[derive(Component)]
struct MyComponent;

#[derive(Component)]
struct MyParticleState {
    timer: Timer,
}
#[derive(Component)]
struct MyParticle;

impl MyParticleState {
    fn move_particle(&self, _xf: &mut Transform) {}
}

// ANCHOR: query
fn my_particle_physics(
    mut q_particles: Query<(&mut Transform, &MyParticleState), With<MyParticle>>,
) {
    q_particles.par_iter_mut().for_each(|(mut transform, my_state)| {
        my_state.move_particle(&mut transform);
    });
}
// ANCHOR_END: query

// ANCHOR: commands
fn my_particle_timers(
    time: Res<Time>,
    mut q_particles: Query<(Entity, &mut MyParticleState), With<MyParticle>>,
    par_commands: ParallelCommands,
) {
    q_particles.par_iter_mut().for_each(|(e_particle, mut my_state)| {
        my_state.timer.tick(time.delta());

        if my_state.timer.finished() {
            par_commands.command_scope(|mut commands| {
                commands.entity(e_particle).despawn();
            })
        }
    });
}
// ANCHOR_END: commands

// ANCHOR: events
fn handle_many_events(
    mut evr: EventReader<MyEvent>,
) {
    evr.par_read().for_each(|ev| {
        // TODO: do something with `ev`
    });
}
// ANCHOR_END: events

// ANCHOR: batching-strategy
fn par_iter_custom_batch_size(
    q: Query<&MyComponent>,
) {
    q.par_iter().batching_strategy(
        BatchingStrategy::new()
            // whatever fine-tuned values you come up with ;)
            .min_batch_size(256)
            .max_batch_size(4096)
    ).for_each(|my_component| {
        // TODO: do some heavy work
    });

    q.par_iter().batching_strategy(
        // fixed batch size
        BatchingStrategy::fixed(1024)
    ).for_each(|my_component| {
        // TODO: do some heavy work
    });
}
// ANCHOR_END: batching-strategy
