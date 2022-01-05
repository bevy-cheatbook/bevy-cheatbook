#![allow(dead_code)]

use bevy::prelude::*;

// ANCHOR: system
use bevy::app::AppExit;

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}
// ANCHOR_END: system

// ANCHOR: main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
// ANCHOR_END: main
