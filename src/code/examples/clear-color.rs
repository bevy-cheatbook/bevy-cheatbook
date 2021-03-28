use bevy::prelude::*;

// ANCHOR: main
fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .run();
}
// ANCHOR_END: main
