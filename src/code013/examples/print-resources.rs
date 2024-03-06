use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::{RenderApp, Render, RenderSet};

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

// ANCHOR: app
// print main world resources
app.add_systems(Last, print_resources);

// print render world resources
app.sub_app_mut(RenderApp)
    .add_systems(Render, print_resources.in_set(RenderSet::Render));
// ANCHOR_END: app

    app.add_systems(Last, quit);

    app.run();
}
