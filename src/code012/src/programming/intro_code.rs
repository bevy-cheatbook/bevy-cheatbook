use bevy::prelude::*;

#[derive(Resource)]
struct EnemyAiSettings;
#[derive(Resource)]
struct GameModeData;
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Enemy;

// ANCHOR: example-system
fn enemy_detect_player(
    // access data from resources
    mut ai_settings: ResMut<EnemyAiSettings>,
    gamemode: Res<GameModeData>,
    // access data from entities/components
    query_player: Query<&Transform, With<Player>>,
    query_enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    // in case we want to spawn/despawn entities, etc.
    mut commands: Commands,
) {
    // ... implement your behavior here ...
}
// ANCHOR_END: example-system

// ANCHOR: exclusive
fn save_game(
    // get full access to the World, so we can access all data and do anything
    world: &mut World,
) {
    // ... save game data to disk, or something ...
}
// ANCHOR_END: exclusive

fn enemy_movement() {}
fn enemy_spawn() {}
fn enemy_despawn() {}
fn mouse_input() {}
fn controller_input() {}
fn footstep_sound() {}
fn menu_button_sound() {}
fn background_music() {}
fn player_movement() {}
fn camera_movement() {}
fn ui_button_animate() {}
fn menu_logo_animate() {}
fn cutscene() -> bool { false }
fn audio_muted() -> bool { false }
fn player_alive() -> bool { false }
fn mouse_enabled() -> bool { false }
fn gamepad_enabled() -> bool { false }

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MySets {
    MainMenuSet,
    GameplaySet,
    InputSet,
    EnemyAiSet,
    AudioSet,
}
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum MyStates {
    #[default]
    MainMenu,
    InGame,
}

fn _main() {
use MySets::*;
use MyStates::*;
let mut app = App::new();
app.add_plugins(DefaultPlugins);
app.add_state::<MyStates>();
app.add_systems(Update, enemy_detect_player);
// ANCHOR: example-scheduling
// Set configuration is per-schedule. Here we do it for `Update`
app.configure_sets(Update, (
    MainMenuSet
        .run_if(in_state(MainMenu)),
    GameplaySet
        .run_if(in_state(InGame)),
    InputSet
        .in_set(GameplaySet),
    EnemyAiSet
        .in_set(GameplaySet)
        .run_if(not(cutscene))
        .after(player_movement),
    AudioSet
        .run_if(not(audio_muted)),
));
app.add_systems(Update, (
    (
        ui_button_animate,
        menu_logo_animate.in_set(MainMenuSet),
    ),
    (
        enemy_movement,
        enemy_spawn,
        enemy_despawn.before(enemy_spawn),
    ).in_set(EnemyAiSet),
    (
        mouse_input.run_if(mouse_enabled),
        controller_input.run_if(gamepad_enabled),
    ).in_set(InputSet),
    (
        footstep_sound.in_set(GameplaySet),
        menu_button_sound.in_set(MainMenuSet),
        background_music,
    ).in_set(AudioSet),
    (
        player_movement
            .run_if(player_alive)
            .run_if(not(cutscene)),
        camera_movement,
    ).in_set(GameplaySet).after(InputSet),
));
// ANCHOR_END: example-scheduling
}
