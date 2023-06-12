use bevy::prelude::*;

fn handle_main_menu_ui() {}
fn spawn_main_menu_ui() {}
fn delete_main_menu_ui() {}

// ANCHOR: define
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // Create the state.
        .add_state::<AppState>()

        .add_system(handle_main_menu_ui
            .run_if(in_state(AppState::MainMenu)))

        .run();
}
// ANCHOR_END: define

mod transitions {
use super::*;

// ANCHOR: transitions
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()

        .add_system(spawn_main_menu_ui
            // Executes when entering the MainMenu state.
            .in_schedule(OnEnter(AppState::MainMenu)))
        
        .add_system(delete_main_menu_ui
            // Executes when leaving the MainMenu state.
            .in_schedule(OnExit(AppState::MainMenu)))
        
        .run();
}
// ANCHOR_END: transitions
}

// ANCHOR: check-state
fn play_music(
    app_state: Res<State<AppState>>,
    // ...
) {
    match app_state.0 {
        AppState::MainMenu => {
            // TODO: play menu music
        }
        AppState::InGame => {
            // TODO: play game music
        }
        AppState::Paused => {
            // TODO: play pause screen music
        }
    }
}
// ANCHOR_END: check-state

// ANCHOR: change-state
fn enter_game(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InGame);
}
// ANCHOR_END: change-state
