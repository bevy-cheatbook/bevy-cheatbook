#[allow(dead_code)]
#[allow(unused_imports)]
mod app5 {
    use bevy::prelude::*;
    use super::*;

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
        AppState::SetupGame | AppState::CleanupGame => {
            // no music to play, either setting up or cleaning up the game
        }
    }
}
// ANCHOR_END: check-state

// ANCHOR: change-state
fn enter_game(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InGame);
}
// ANCHOR_END: change-state

// ANCHOR: state-input-clear
fn esc_to_menu(
    key: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::InGame)
    }
}
// ANCHOR_END: state-input-clear

// ANCHOR: app-states
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    // one of the states must be the default
    #[default]
    MainMenu,
    SetupGame,
    InGame,
    Paused,
    CleanupGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // add the app state type
        .add_state::<AppState>()

        // add systems to run regardless of state, as usual
        .add_system(play_music)

        // systems to run only in the main menu
        .add_system(
            handle_ui_buttons.in_set(OnUpdate(AppState::MainMenu))
        )

        // setup when entering the state
        .add_system(
            setup_menu.in_schedule(OnEnter(AppState::MainMenu))
        )

        // cleanup when exiting the state
        .add_system(
            close_menu.in_schedule(OnExit(AppState::MainMenu))
        )
        .run();
}
// ANCHOR_END: app-states

fn animate_trees() {}
fn animate_water() {}
fn player_movement() {}
fn player_idle() {}
fn reset_player() {}
fn hide_enemies() {}
fn setup_player() {}
fn despawn_player() {}
fn setup_map() {}
fn despawn_map() {}
fn handle_ui_buttons() {}
fn setup_menu() {}
fn close_menu() {}

fn main2() {
    App::new()
        .add_plugins(DefaultPlugins)
        // add the app state type
        .add_state::<AppState>()
// ANCHOR: app-states-complex
        // player movement only when actively playing
        .add_system(
            player_movement
                .in_set(OnUpdate(AppState::InGame))
        )
        // player idle animation while paused
        .add_system(
            player_idle
                .in_set(OnUpdate(AppState::Paused))
        )
        // animations both while paused and while actively playing
        .add_systems(
            (animate_trees, animate_water)
                .in_set(OnUpdate(AppState::InGame))
                .in_set(OnUpdate(AppState::Paused))
        )
        // things to do when becoming paused
        .add_system(
            hide_enemies
                .in_schedule(OnEnter(AppState::Paused))
        )
        // things to do when becoming active again
        .add_system(
            reset_player
                .in_schedule(OnExit(AppState::Paused))
        )
        // setup when first entering the game from main menu
        .add_systems(
            (setup_player, setup_map)
                .in_schedule(OnEnter(AppState::SetupGame))
        )
        // cleanup when exiting the game for good
        .add_systems(
            (despawn_player, despawn_map)
                .in_schedule(OnEnter(AppState::CleanupGame))
        )
// ANCHOR_END: app-states-complex
        .run();
}
}