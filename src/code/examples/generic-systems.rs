use bevy::prelude::*;

// ANCHOR: cleanup
use bevy::ecs::component::Component;

/// Marker components to group entities for cleanup
mod cleanup {
    pub struct LevelUnload;
    pub struct MenuExit;
}

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
            .with_system(cleanup_system::<cleanup::MenuExit>.system()))
        .add_system_set(SystemSet::on_exit(AppState::InGame)
            .with_system(cleanup_system::<cleanup::LevelUnload>.system()))
        .run();
}
// ANCHOR_END: main
