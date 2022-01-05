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
    use bevy::prelude::*;
    #[derive(Component)]
    pub struct LevelUnload;
    #[derive(Component)]
    pub struct MenuClose;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainMenu)
        // add the cleanup systems
        .add_system_set(SystemSet::on_exit(AppState::MainMenu)
            .with_system(cleanup_system::<cleanup::MenuClose>))
        .add_system_set(SystemSet::on_exit(AppState::InGame)
            .with_system(cleanup_system::<cleanup::LevelUnload>))
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
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(process_layer::<1>)
        .add_system(process_layer::<2>)
        .add_system(process_layer::<3>)
        .run();
}
// ANCHOR_END: const
}

