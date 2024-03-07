use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;

struct OSAudioMagic {
    ptr: *mut u32,
}

impl OSAudioMagic {
    fn init() -> Self {
        Self {
            ptr: 0xDEADBEEF as *mut u32,
        }
    }
}

// ANCHOR: insert-nonsend
fn setup_platform_audio(world: &mut World) {
    // assuming `OSAudioMagic` is some primitive that is not thread-safe
    let instance = OSAudioMagic::init();

    world.insert_non_send_resource(instance);
}
// ANCHOR_END: insert-nonsend

// ANCHOR: nonsend
fn setup_raw_window(
    q_primary: Query<Entity, With<PrimaryWindow>>,
    mut windows: NonSend<WinitWindows>
) {
    let raw_window = windows.get_window(q_primary.single());
    // do some special things using `winit` APIs
}
// ANCHOR_END: nonsend

fn main() {
let mut app = App::new();
// ANCHOR: insert-nonsend-app
app.add_systems(Startup, setup_platform_audio);
// ANCHOR_END: insert-nonsend-app
// ANCHOR: insert-nonsend-app-world
app.world.insert_non_send_resource(OSAudioMagic::init());
// ANCHOR_END: insert-nonsend-app-world
// ANCHOR: nonsend-app
// just add it as a normal system;
// Bevy will notice the NonSend parameter
// and ensure it runs on the main thread
app.add_systems(Startup, setup_raw_window);
// ANCHOR_END: nonsend-app
}
