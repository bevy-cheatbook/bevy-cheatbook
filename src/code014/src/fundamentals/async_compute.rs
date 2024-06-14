use bevy::prelude::*;
use bevy::utils::HashMap;

fn plugin(app: &mut App) {
// ANCHOR: async-compute-app
// every frame, we might have some new chunks that are ready,
// or the need to start generating some new ones. :)
app.add_systems(Update, (
    begin_generating_map_chunks, receive_generated_map_chunks
));
// ANCHOR_END: async-compute-app
}

struct MyMapChunkData {
}

// ANCHOR: async-compute
use bevy::tasks::futures_lite::future;
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};

#[derive(Resource)]
struct MyMapGenTasks {
    generating_chunks: HashMap<UVec2, Task<MyMapChunkData>>,
}

fn begin_generating_map_chunks(
    mut my_tasks: ResMut<MyMapGenTasks>,
) {
    let task_pool = AsyncComputeTaskPool::get();
    for chunk_coord in decide_what_chunks_to_generate(/* ... */) {
        // we might have already spawned a task for this `chunk_coord`
        if my_tasks.generating_chunks.contains_key(&chunk_coord) {
            continue;
        }
        let task = task_pool.spawn(async move {
            // TODO: do whatever you want here!
            generate_map_chunk(chunk_coord)
        });
        my_tasks.generating_chunks.insert(chunk_coord, task);
    }
}

fn receive_generated_map_chunks(
    mut my_tasks: ResMut<MyMapGenTasks>
) {
    my_tasks.generating_chunks.retain(|chunk_coord, task| {
        // check on our task to see how it's doing :)
        let status = block_on(future::poll_once(task));

        // keep the entry in our HashMap only if the task is not done yet
        let retain = status.is_none();

        // if this task is done, handle the data it returned!
        if let Some(mut chunk_data) = status {
            // TODO: do something with the returned `chunk_data`
        }

        retain
    });
}
// ANCHOR_END: async-compute

fn decide_what_chunks_to_generate() -> Vec<UVec2> {
    unimplemented!()
}

fn generate_map_chunk(chunk_coord: UVec2) -> MyMapChunkData {
    unimplemented!()
}
