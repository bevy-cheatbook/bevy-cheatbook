#![allow(unused_variables)]
#![allow(unused_mut)]

use bevy::prelude::*;

struct PlayerInfo;
struct MyNetProto;

#[allow(dead_code)]
struct GameUnit;

impl MyNetProto {
    fn recv_message(&mut self) -> Option<()> {
        unimplemented!()
    }
}

#[allow(dead_code)]
// ANCHOR: event_enum
/// Different things that can happen in the game
enum GameEvent {
    UnitSpawn {
        id: u32,
        position: Vec2,
    },
    UnitDestroyed {
        id: u32,
    },
    UnitMove {
        id: u32,
        position: Vec2,
    },
    PlayerDefeated {
        player_id: u32,
    },
    PlayerJoined {
        player_id: u32,
    }
    // etc ...
}
// ANCHOR_END: event_enum

// ANCHOR: net_system
fn net_system(
    mut net_state: ResMut<MyNetProto>,
    mut ev_game: ResMut<Events<GameEvent>>
) {
    while let Some(data) = net_state.recv_message() {
        // handle `data` somehow, and send events;
        // this is just a placeholder:
        ev_game.send(GameEvent::UnitMove {
            id: 7,
            position: Vec2::zero()
        });
        ev_game.send(GameEvent::PlayerDefeated {
            player_id: 1
        });
        ev_game.send(GameEvent::UnitDestroyed {
            id: 6,
        });
    }
}
// ANCHOR_END: net_system

// ANCHOR: game_systems
fn unit_move_system(
    ev_game: Res<Events<GameEvent>>,
    mut evr_game: Local<EventReader<GameEvent>>,
    q_units: Query<&mut Transform, With<GameUnit>>,
) {
    for ev in evr_game.iter(&ev_game) {
        match ev {
            GameEvent::UnitMove { id, position } => {
                // TODO: handle unit movement here
            }
            // we don't care about other events
            _ => {}
        }
    }
}

fn unit_manager_system(
    ev_game: Res<Events<GameEvent>>,
    mut evr_game: Local<EventReader<GameEvent>>,
    commands: &mut Commands,
) {
    for ev in evr_game.iter(&ev_game) {
        match ev {
            GameEvent::UnitSpawn { id, position } => {
                // TODO: handle unit spawn here
            }
            GameEvent::UnitDestroyed { id } => {
                // TODO: handle unit despawn here
            }
            // we don't care about other events
            _ => {}
        }
    }
}

fn session_manager_system(
    ev_game: Res<Events<GameEvent>>,
    mut evr_game: Local<EventReader<GameEvent>>,
    mut player_info: ResMut<PlayerInfo>,
) {
    for ev in evr_game.iter(&ev_game) {
        match ev {
            GameEvent::PlayerJoined { player_id } => {
                // TODO: handle new player
            }
            GameEvent::PlayerDefeated { player_id } => {
                // TODO: handle player loss
            }
            // we don't care about other events
            _ => {}
        }
    }
}
// ANCHOR_END: game_systems

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<GameEvent>()
        .add_resource(PlayerInfo)
        .add_resource(MyNetProto)
        .add_system(net_system.system())
        .add_system(unit_move_system.system())
        .add_system(unit_manager_system.system())
        .add_system(session_manager_system.system())
        .run();
}
