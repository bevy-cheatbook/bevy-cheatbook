use bevy::ecs::{archetype::Archetypes, component::Components};
use bevy::prelude::*;
use bevy::reflect::TypeRegistration;

// ANCHOR: example
fn print_resources(archetypes: &Archetypes, components: &Components) {
    let mut r: Vec<String> = archetypes
        .resource()
        .components()
        .map(|id| components.get_info(id).unwrap())
        // get_short_name removes the path information
        // i.e. `bevy_audio::audio::Audio` -> `Audio`
        // if you want to see the path info replace
        // `TypeRegistration::get_short_name` with `String::from`
        .map(|info| TypeRegistration::get_short_name(info.name()))
        .collect();

    // sort list alphebetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}
// ANCHOR_END: example

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(print_resources)
        .run();
}
