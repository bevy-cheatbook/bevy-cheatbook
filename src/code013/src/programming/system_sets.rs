use bevy::prelude::*;

// ANCHOR: definition
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudioSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyInputSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MyInputKindSet {
    Touch,
    Mouse,
    Gamepad,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MyGameplaySet {
    Player,
    Enemies,
}
// ANCHOR_END: definition

fn play_music() {}
fn play_ui_sounds() {}
fn player_movement() {}
fn player_animation() {}
fn player_level_up() {}
fn player_footsteps() {}
fn enemy_movement() {}
fn enemy_ai() {}
fn enemy_footsteps() {}
fn mouse_cursor_tracking() {}
fn mouse_clicks() {}
fn gamepad_cursor_tracking() {}
fn gamepad_buttons() {}
fn touch_gestures() {}

fn audio_enabled() -> bool {
    true
}
fn music_enabled() -> bool {
    true
}
fn player_is_alive() -> bool {
    true
}
fn enemies_present() -> bool {
    true
}
fn touchscreen_enabled() -> bool {
    true
}
fn mouse_enabled() -> bool {
    true
}
fn gamepad_connected() -> bool {
    true
}

fn main() {
let mut app = App::new();
// ANCHOR: app
app.add_systems(Update, (
    (
        play_music
            .run_if(music_enabled),
        play_ui_sounds,
    ).in_set(MyAudioSet),
    (
        player_movement,
        player_animation
            .after(player_movement),
        player_level_up,
        player_footsteps
            .in_set(MyAudioSet),
    ).in_set(MyGameplaySet::Player),
    (
        enemy_movement,
        enemy_ai,
        enemy_footsteps
            .in_set(MyAudioSet),
    ).in_set(MyGameplaySet::Enemies),
    (
        (
            mouse_cursor_tracking,
            mouse_clicks,
        ).in_set(MyInputKindSet::Mouse),
        (
            gamepad_cursor_tracking,
            gamepad_buttons,
        ).in_set(MyInputKindSet::Gamepad),
        (
            touch_gestures,
        ).in_set(MyInputKindSet::Touch),
    ).in_set(MyInputSet),
));
// ANCHOR_END: app
// ANCHOR: configure
app.configure_sets(Update, (
    MyAudioSet
        .run_if(audio_enabled),
    MyGameplaySet::Player
        .after(MyInputSet)
        .run_if(player_is_alive),
    MyGameplaySet::Enemies
        .run_if(enemies_present),
    MyInputKindSet::Touch
        .run_if(touchscreen_enabled),
    MyInputKindSet::Mouse
        .run_if(mouse_enabled),
    MyInputKindSet::Gamepad
        .run_if(gamepad_connected),
));
// ANCHOR_END: configure
}
