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
// ANCHOR: channels-app
app.add_systems(Startup, setup_net_session);
app.add_systems(FixedUpdate, (
    tell_the_net_task_what_to_do,
    handle_net_updates,
));
// ANCHOR_END: channels-app
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

// ANCHOR: channels
use bevy::tasks::IoTaskPool;
use async_channel::{Sender, Receiver};

/// Messages we send to our netcode task
enum MyNetControlMsg {
    DoSomething,
    // ...
}

/// Messages we receive from our netcode task
enum MyNetUpdateMsg {
    SomethingHappened,
    // ...
}

/// Channels used for communicating with our game's netcode task.
/// (The side used from our Bevy systems)
#[derive(Resource)]
struct MyNetChannels {
    tx_control: Sender<MyNetControlMsg>,
    rx_updates: Receiver<MyNetUpdateMsg>,
}

fn setup_net_session(
    mut commands: Commands,
) {
    // create our channels:
    let (tx_control, rx_control) = async_channel::unbounded();
    let (tx_updates, rx_updates) = async_channel::unbounded();

    // spawn our background i/o task for networking
    // and give it its side of the channels:
    IoTaskPool::get().spawn(async move {
        my_netcode(rx_control, tx_updates).await
    }).detach();

    // NOTE: `.detach()` to let the task run
    // without us storing the `Task` handle.
    // Otherwise, the task will get canceled!

    // (though in a real application, you probably want to
    // store the `Task` handle and have a system to monitor
    // your task and recreate it if necessary)

    // put our side of the channels in a resource for later
    commands.insert_resource(MyNetChannels {
        tx_control, rx_updates,
    });
}

fn handle_net_updates(
    my_channels: Res<MyNetChannels>,
) {
    // Non-blocking check for any new messages on the channel
    while let Ok(msg) = my_channels.rx_updates.try_recv() {
        // TODO: do something with `msg`
    }
}

fn tell_the_net_task_what_to_do(
    my_channels: Res<MyNetChannels>,
) {
    if let Err(e) = my_channels.tx_control.try_send(MyNetControlMsg::DoSomething) {
        // TODO: handle errors. Maybe our task has
        // returned or panicked, and closed the channel?
    }
}

/// This runs in the background I/O task
async fn my_netcode(
    rx_control: Receiver<MyNetControlMsg>,
    tx_updates: Sender<MyNetUpdateMsg>,
) {
    // TODO: Here we can connect and talk to our multiplayer server,
    // handle incoming `MyNetControlMsg`s, send `MyNetUpdateMsg`s, etc.

    while let Ok(msg) = rx_control.recv().await {
        // TODO: do something with `msg`

        // Send data back, to be handled from Bevy systems:
        tx_updates.send(MyNetUpdateMsg::SomethingHappened).await
            .expect("Error sending updates over channel");

        // We can also spawn additional parallel tasks
        IoTaskPool::get().spawn(async move {
            // ... some other I/O work ...
        }).detach();
        AsyncComputeTaskPool::get().spawn(async move {
            // ... some heavy CPU work ...
        }).detach();
    }
}
// ANCHOR_END: channels

fn decide_what_chunks_to_generate() -> Vec<UVec2> {
    unimplemented!()
}

fn generate_map_chunk(chunk_coord: UVec2) -> MyMapChunkData {
    unimplemented!()
}
