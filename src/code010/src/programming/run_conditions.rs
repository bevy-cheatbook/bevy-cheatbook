use bevy::prelude::*;

fn server_session() {}
fn fetch_server_updates() {}
fn keyboard_input() {}
fn gamepad_input() {}
fn host_debug() {}
fn host_session() {}
fn host_player_movement() {}
fn host_enemy_ai() {}
fn smoke_particles() {}
fn water_animation() {}

#[derive(Resource)]
struct MyNetworkSession;

impl MyNetworkSession {
    fn is_connected(&self) -> bool {
        true
    }
}

// ANCHOR: conditions
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum MySystemSet {
    Host,
    Input,
}

#[derive(Debug, PartialEq, Eq)]
#[derive(Resource)]
enum MultiplayerKind {
    Client,
    Host,
    Local,
}

fn is_connected(
    mode: Res<MultiplayerKind>,
    session: Res<MyNetworkSession>,
) -> bool
{
    *mode == MultiplayerKind::Client && session.is_connected()
}

fn is_host(
    mode: Res<MultiplayerKind>,
) -> bool
{
    *mode == MultiplayerKind::Host || *mode == MultiplayerKind::Local
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // if we are currently connected to a server,
        // activate our client systems
        .add_systems((server_session, fetch_server_updates)
            .before(MySystemSet::Input)
            .distributive_run_if(is_connected))

        // if we are hosting the game,
        // activate our game hosting systems
        .configure_set(MySystemSet::Host
            .before(MySystemSet::Input)
            .run_if(is_host))
        .add_systems((host_session, host_player_movement, host_enemy_ai)
            .in_set(MySystemSet::Host))

        // other systems in our game
        .add_system(smoke_particles)
        .add_system(water_animation)

        .add_systems((keyboard_input, gamepad_input)
            .in_set(MySystemSet::Input))
        .run();
}
// ANCHOR_END: conditions

mod sub {

use super::*;

fn is_paused() -> bool { false }

// ANCHOR: combine

fn main() {
    App::new()
        // ...
        .add_system(host_player_movement
            .run_if(is_host.and_then(not(is_paused))))
        .run();
}

// ANCHOR_END: combine
}