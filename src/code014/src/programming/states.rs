use bevy::prelude::*;

fn start_load_assets() {}
fn spawn_progress_bar() {}
fn despawn_loading_screen() {}
fn setup_main_menu_ui() {}
fn setup_main_menu_camera() {}
fn despawn_main_menu() {}
fn spawn_game_map() {}
fn setup_game_camera() {}
fn spawn_enemies() {}
fn setup_singleplayer() {}
fn setup_multiplayer() {}
fn handle_main_menu_ui_input() {}
fn play_main_menu_sounds() {}
fn camera_movement() {}
fn play_game_music() {}
fn player_movement() {}
fn enemy_ai() {}
fn player_net_sync() {}
fn enemy_net_sync() {}
fn load_settings() {}
fn setup_window_icon() {}
fn my_plugin_system1() {}
fn my_plugin_system2() {}
fn update_loading_progress_bar() {}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyMainMenuSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyGameplaySet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MySingleplayerSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyMultiplayerSet;

// ANCHOR: definition
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum MyAppState {
    LoadingScreen,
    MainMenu,
    InGame,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MyGameModeState {
    #[default]
    NotInGame,
    Singleplayer,
    Multiplayer,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MyPausedState {
    #[default]
    Paused,
    Running,
}
// ANCHOR_END: definition

// ANCHOR: custom-transition
fn my_apply_transitions(world: &mut World) {
    world.run_schedule(StateTransition);
}
// ANCHOR_END: custom-transition

fn main() {
let mut app = App::new();

// ANCHOR: app-register
// Specify the initial value:
app.insert_state(MyAppState::LoadingScreen);

// Or use the default (if the type impls Default):
app.init_state::<MyGameModeState>();
app.init_state::<MyPausedState>();
// ANCHOR_END: app-register

// ANCHOR: app-example-transitions
// do the respective setup and cleanup on state transitions
app.add_systems(OnEnter(MyAppState::LoadingScreen), (
    start_load_assets,
    spawn_progress_bar,
));
app.add_systems(OnExit(MyAppState::LoadingScreen), (
    despawn_loading_screen,
));
app.add_systems(OnEnter(MyAppState::MainMenu), (
    setup_main_menu_ui,
    setup_main_menu_camera,
));
app.add_systems(OnExit(MyAppState::MainMenu), (
    despawn_main_menu,
));
app.add_systems(OnEnter(MyAppState::InGame), (
    spawn_game_map,
    setup_game_camera,
    spawn_enemies,
));
app.add_systems(OnEnter(MyGameModeState::Singleplayer), (
    setup_singleplayer,
));
app.add_systems(OnEnter(MyGameModeState::Multiplayer), (
    setup_multiplayer,
));
// ...
// ANCHOR_END: app-example-transitions

// ANCHOR: app-example
// configure some system sets to help us manage our systems
// (note: it is per-schedule, so we also need it for FixedUpdate
// if we plan to use fixed timestep)
app.configure_sets(Update, (
    MyMainMenuSet
        .run_if(in_state(MyAppState::MainMenu)),
    MyGameplaySet
        // note: you can check for a combination of different states
        .run_if(in_state(MyAppState::InGame))
        .run_if(in_state(MyPausedState::Running)),
));
app.configure_sets(FixedUpdate, (
    // configure the same set here, so we can use it in both
    // FixedUpdate and Update
    MyGameplaySet
        .run_if(in_state(MyAppState::InGame))
        .run_if(in_state(MyPausedState::Running)),
    // configure a bunch of different sets only for FixedUpdate
    MySingleplayerSet
        // inherit configuration from MyGameplaySet and add extras
        .in_set(MyGameplaySet)
        .run_if(in_state(MyGameModeState::Singleplayer)),
    MyMultiplayerSet
        .in_set(MyGameplaySet)
        .run_if(in_state(MyGameModeState::Multiplayer)),
));

// now we can easily add our different systems
app.add_systems(Update, (
    update_loading_progress_bar
        .run_if(in_state(MyAppState::LoadingScreen)),
    (
        handle_main_menu_ui_input,
        play_main_menu_sounds,
    ).in_set(MyMainMenuSet),
    (
        camera_movement,
        play_game_music,
    ).in_set(MyGameplaySet),
));
app.add_systems(FixedUpdate, (
    (
        player_movement,
        enemy_ai,
    ).in_set(MySingleplayerSet),
    (
        player_net_sync,
        enemy_net_sync,
    ).in_set(MyMultiplayerSet),
));

// of course, if we need some global (state-independent)
// setup to run on app startup, we can still use Startup as usual
app.add_systems(Startup, (
    load_settings,
    setup_window_icon,
));
// ANCHOR_END: app-example

// ANCHOR: plugin-config-app
app.add_plugins(MyPlugin {
    state: MyAppState::InGame,
});
// ANCHOR_END: plugin-config-app

// ANCHOR: app-custom-transition
// Example: also do state transitions on each fixed timestep run
app.add_systems(
    FixedUpdate,
    my_apply_transitions
        .before(MyGameplaySet)
);
// ANCHOR_END: app-custom-transition
}

// ANCHOR: plugin-config
pub struct MyPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MyPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            my_plugin_system1,
            my_plugin_system2,
            // ...
        ).run_if(in_state(self.state.clone())));
    }
}
// ANCHOR_END: plugin-config

// ANCHOR: check-state
fn debug_current_gamemode_state(state: Res<State<MyGameModeState>>) {
    eprintln!("Current state: {:?}", state.get());
}
// ANCHOR_END: check-state

// ANCHOR: change-state
fn toggle_pause_game(
    state: Res<State<MyPausedState>>,
    mut next_state: ResMut<NextState<MyPausedState>>,
) {
    match state.get() {
        MyPausedState::Paused => next_state.set(MyPausedState::Running),
        MyPausedState::Running => next_state.set(MyPausedState::Paused),
    }
}

// if you have multiple states that must be set correctly,
// don't forget to manage them all
fn new_game_multiplayer(
    mut next_app: ResMut<NextState<MyAppState>>,
    mut next_mode: ResMut<NextState<MyGameModeState>>,
) {
    next_app.set(MyAppState::InGame);
    next_mode.set(MyGameModeState::Multiplayer);
}
// ANCHOR_END: change-state
