use bevy::prelude::*;

// ANCHOR: cleanup
use bevy::ecs::component::Component;

fn cleanup_system<T: Component>(
    mut commands: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
// ANCHOR_END: cleanup

// ANCHOR: main
/// Marker components to group entities for cleanup
mod cleanup {
    pub struct LevelUnload;
    pub struct MenuClose;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainMenu)
        // add the cleanup systems
        .add_system_set(SystemSet::on_exit(AppState::MainMenu)
            .with_system(cleanup_system::<cleanup::MenuClose>.system()))
        .add_system_set(SystemSet::on_exit(AppState::InGame)
            .with_system(cleanup_system::<cleanup::LevelUnload>.system()))
        .run();
}
// ANCHOR_END: main

#[allow(dead_code)]
mod constgenerics {
use bevy::prelude::*;

// ANCHOR: const
fn process_layer<const LAYER_ID: usize>(
    // system params
) {
    // do something for this `LAYER_ID`
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(process_layer::<1>.system())
        .add_system(process_layer::<2>.system())
        .add_system(process_layer::<3>.system())
        .run();
}
// ANCHOR_END: const
}

