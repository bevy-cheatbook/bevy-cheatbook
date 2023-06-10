use bevy::prelude::*;

// ANCHOR: define
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum MyGameLoop {
    HandleInput,
    UpdateCharacters,
    ShowResults,
}
// ANCHOR_END: define

fn finish_update_system() {}

// ANCHOR: configure
fn main() {
    App::new()
        .configure_set(MyGameLoop::HandleInput.before(MyGameLoop::UpdateCharacters))
        .configure_set(MyGameLoop::ShowResults.after(finish_update_system))
        // ... finish building the app ...
        .run();
}
// ANCHOR_END: configure

mod add_systems {

use bevy::prelude::*;

use super::MyGameLoop;

fn add_input_to_characters() {}
fn handle_jump_button() {}
fn particle_effect() {}

// ANCHOR: add-systems
fn main() {
    App::new()
        .add_systems(
            (add_input_to_characters, handle_jump_button)
                .in_set(MyGameLoop::HandleInput))
        .add_system(particle_effect.in_set(MyGameLoop::ShowResults))
        // ... finish building the app ...
        .run();
}
// ANCHOR_END: add-systems
}

mod nested {

use bevy::prelude::*;

use super::MyGameLoop;

// ANCHOR: nested
fn main() {
    App::new()
        .configure_set(MyGameLoop::ShowResults
            .in_set(MyGameLoop::UpdateCharacters))
        // ... finish building the app ...
        .run();
}
// ANCHOR_END: nested
}
