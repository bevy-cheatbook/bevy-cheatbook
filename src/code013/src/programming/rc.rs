use bevy::prelude::*;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Health {
    hp: f32,
}

#[derive(Resource)]
struct MyNetworkSession;

impl MyNetworkSession {
    fn is_connected(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Eq)]
#[derive(Resource)]
enum MyMultiplayerMode {
    Client,
    Host,
    Local,
}

// ANCHOR: fn
fn run_if_player_alive(
    q_player: Query<&Health, With<Player>>,
) -> bool {
    let health = q_player.single();
    health.hp > 0.0
}

fn run_if_connected(
    mode: Res<MyMultiplayerMode>,
    session: Res<MyNetworkSession>,
) -> bool
{
    *mode != MyMultiplayerMode::Local && session.is_connected()
}

fn run_if_enemies_present(
    q_enemies: Query<(), With<Enemy>>,
) -> bool {
    !q_enemies.is_empty()
}
// ANCHOR_END: fn

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MyPlayerSet;

fn player_input() {}
fn player_movement() {}
fn player_alert_enemies() {}
fn manage_multiplayer_server() {}

fn main() {
    let mut app = App::new();

// ANCHOR: app
app.configure_sets(Update,
    MyPlayerSet
        .run_if(run_if_player_alive)
);

app.add_systems(Update, (
    player_input,
    player_movement,
    player_alert_enemies
        .run_if(run_if_enemies_present)
).in_set(MyPlayerSet));

app.add_systems(Update,
    manage_multiplayer_server
        .run_if(run_if_connected)
);
// ANCHOR_END: app
}
