use bevy::prelude::*;

// ANCHOR: cleanup
/// Marker components to group entities for cleanup
mod cleanup {
    pub struct LevelUnload;
    pub struct MenuExit;
}

fn cleanup_system<T: Component>(
    commands: &mut Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        commands.despawn_recursive(e);
    }
}
// ANCHOR_END: cleanup

// ANCHOR: main
#[derive(Debug, Clone)]
enum AppState {
    MainMenu,
    InGame,
}

/// Label for the main stage
static MAIN: &str = "MainStage";

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // add the state stage
        .add_resource(State::new(AppState::MainMenu))
        .add_stage_before(
            stage::UPDATE, MAIN,
            StateStage::<AppState>::default()
        )
        // add the cleanup systems
        .on_state_exit(
            MAIN, AppState::MainMenu,
            cleanup_system::<cleanup::MenuExit>.system()
        )
        .on_state_exit(
            MAIN, AppState::InGame,
            cleanup_system::<cleanup::LevelUnload>.system()
        )
        .run();
}
// ANCHOR_END: main
