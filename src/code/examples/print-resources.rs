use bevy::ecs::{archetype::Archetypes, component::Components};
use bevy::prelude::*;

// ANCHOR: example
fn print_resources(archetypes: &Archetypes, components: &Components) {
    // TODO FIX THIS WHEN BEVY 0.9.1 IS RELEASED

    // let mut r: Vec<_> = archetypes
    //     .resource()
    //     .components()
    //     .map(|id| components.get_info(id).unwrap())
    //     .map(|info| info.name())
    //     .collect();

    // // sort list alphebetically
    // r.sort();
    // r.iter().for_each(|name| println!("{}", name));
}
// ANCHOR_END: example

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(print_resources)
        .run();
}
