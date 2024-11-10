use bevy::prelude::*;

fn main() {
// ANCHOR: singlethread-updateonly
use bevy::ecs::schedule::ExecutorKind;

App::new()
    .add_plugins(DefaultPlugins)
    .edit_schedule(Update, |schedule| {
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);
    })
    // ...
// ANCHOR_END: singlethread-updateonly
    .run();

// ANCHOR: taskpool-overprovision
use bevy::core::TaskPoolThreadAssignmentPolicy;
use bevy::tasks::available_parallelism;

App::new()
    .add_plugins(DefaultPlugins.set(TaskPoolPlugin {
        task_pool_options: TaskPoolOptions {
            compute: TaskPoolThreadAssignmentPolicy {
                // set the minimum # of compute threads
                // to the total number of available threads
                min_threads: available_parallelism(),
                max_threads: std::usize::MAX, // unlimited max threads
                percent: 1.0, // this value is irrelevant in this case
            },
            // keep the defaults for everything else
            ..default()
        }
    }))
    // ...
// ANCHOR_END: taskpool-overprovision
    .run();

// ANCHOR: taskpool-custom
App::new()
    .add_plugins(DefaultPlugins.set(TaskPoolPlugin {
        task_pool_options: TaskPoolOptions {
            min_total_threads: 1,
            max_total_threads: std::usize::MAX, // unlimited threads
            io: TaskPoolThreadAssignmentPolicy {
                // say we know our app is i/o intensive (asset streaming?)
                // so maybe we want lots of i/o threads
                min_threads: 4,
                max_threads: std::usize::MAX,
                percent: 0.5, // use 50% of available threads for I/O
            },
            async_compute: TaskPoolThreadAssignmentPolicy {
                // say our app never does any background compute,
                // so we don't care, but keep one thread just in case
                min_threads: 1,
                max_threads: 1,
                percent: 0.0,
            },
            compute: TaskPoolThreadAssignmentPolicy {
                // say we want to use at least half the CPU for compute
                // (maybe over-provisioning if there are very few cores)
                min_threads: available_parallelism() / 2,
                // but limit it to a maximum of 8 threads
                max_threads: 8,
                // 1.0 in this case means "use all remaining threads"
                // (that were not assigned to io/async_compute)
                // (clamped to min_threads..=max_threads)
                percent: 1.0,
            },
        }
    }))
    // ...
// ANCHOR_END: taskpool-custom
    .run();

// ANCHOR: disable-pipelined-rendering
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;

App::new()
    .add_plugins(DefaultPlugins.build().disable::<PipelinedRenderingPlugin>())
    // ...
    .run();
// ANCHOR_END: disable-pipelined-rendering
}

fn setup(
    mut commands: Commands,
) {
// ANCHOR: cluster-smallz
use bevy::pbr::ClusterConfig;

commands.spawn((
    Camera3dBundle {
        // ... your 3D camera configruation
        ..Default::default()
    },
    ClusterConfig::FixedZ {
        // 4096 clusters is the Bevy default
        // if you don't have many lights, you can reduce this value
        total: 4096,
        // Bevy default is 24 Z-slices
        // For a top-down-view game, 1 is probably optimal.
        z_slices: 1,
        dynamic_resizing: true,
        z_config: Default::default(),
    }
));
// ANCHOR_END: cluster-smallz
// ANCHOR: cluster-single
commands.spawn((
    Camera3dBundle {
        // ... your 3D camera configruation
        ..Default::default()
    },
    ClusterConfig::Single,
));
// ANCHOR_END: cluster-single
}
