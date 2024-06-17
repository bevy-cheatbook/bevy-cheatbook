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

#[derive(Clone, Copy)]
struct Something;

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

// ANCHOR: parallel-slice
use bevy::tasks::{ParallelSlice, ParallelSliceMut};

fn parallel_slices(/* ... */) {
    // say we have a big vec with a bunch of data
    let mut my_data = vec![Something; 10000];

    // and we want to process it across the number of
    // available CPU threads, splitting it into equal chunks
    my_data.par_splat_map_mut(ComputeTaskPool::get(), None, |i, data| {
        // `i` is the starting index of the current chunk
        // `data` is the sub-slice / chunk to process
        for item in data.iter_mut() {
            process_thing(item);
        }
    });

    // Example: we have a bunch of numbers
    let mut my_values = vec![10; 8192];

    // Example: process it in chunks of 1024
    // to compute the sums of each sequence of 1024 values.
    let sums = my_values.par_chunk_map(ComputeTaskPool::get(), 1024, |_, data| {
        // sum the current chunk of 1024 values
        let sum: u64 = data.iter().sum();
        // return it out of the closure
        sum
    });

    // `sums` is now a `Vec<u64>` containing
    // the returned value from each chunk, in order
}
// ANCHOR_END: parallel-slice

// ANCHOR: scoped-task
use bevy::tasks::ComputeTaskPool;

fn my_system(/* ... */) {
    // say we have a bunch of variables
    let mut a = Something;
    let mut b = Something;
    let mut more_things = [Something; 5];

    // and we want to process the above things in parallel
    ComputeTaskPool::get().scope(|scope| {
        // spawn our tasks using the scope:
        scope.spawn(async {
            process_thing(&mut a);
        });
        scope.spawn(async {
            process_thing(&mut b);
        });

        // nested spawning is also possible:
        // you can use the scope from within a task,
        // to spawn more tasks
        scope.spawn(async {
            for thing in more_things.iter_mut() {
                scope.spawn(async {
                    process_thing(thing);
                })
            }
            debug!("`more_things` array done processing.");
        });
    });

    // at this point, after the task pool scope returns,
    // all our tasks are done and everything has been processed
}
// ANCHOR_END: scoped-task

fn process_thing(x: &mut Something) {
    unimplemented!()
}
