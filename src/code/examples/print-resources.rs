use bevy::app::AppExit;
use bevy::ecs::{archetype::Archetypes, component::Components};
use bevy::prelude::*;
use bevy::render::{RenderApp, RenderStage};

// ANCHOR: example
fn print_resources(world: &World) {
    let components = world.components();

    let mut r: Vec<_> = world
        .storages()
        .resources
        .iter()
        .map(|(id, _)| components.get_info(id).unwrap())
        .map(|info| info.name())
        .collect();

    // sort list alphebetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}
// ANCHOR_END: example

fn quit(mut evw: EventWriter<AppExit>) {
    evw.send(AppExit);
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // print main world resources
    app.add_system_to_stage(CoreStage::Last, print_resources);

    // print render world resources
    app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Render, print_resources);

    app.add_system_to_stage(CoreStage::Last, quit);

    app.run();
}
