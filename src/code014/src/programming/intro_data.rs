use bevy::prelude::*;

// ANCHOR: query
#[derive(Component)]
struct Xp(u32);

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

fn level_up(
    // We want to access the Xp and Health data:
    mut query: Query<(&mut Xp, &mut Health)>,
) {
    // process all relevant entities
    for (mut xp, mut health) in query.iter_mut() {
        if xp.0 > 1000 {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
        }
    }
}
// ANCHOR_END: query

// ANCHOR: commands
/// Marker for the player
#[derive(Component)]
struct Player;

/// Bundle to make it easy to spawn the player entity
/// with all the correct components:
#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    health: Health,
    xp: Xp,
    // including all the components from another bundle
    sprite: SpriteBundle,
}

fn spawn_player(
    // needed for safely creating/removing data in the ECS World
    // (anything done via Commands will be applied later)
    mut commands: Commands,
    // needed for loading assets
    asset_server: Res<AssetServer>,
) {
    // create a new entity with whatever components we want
    commands.spawn(PlayerBundle {
        marker: Player,
        health: Health {
            current: 100,
            max: 125,
        },
        xp: Xp(0),
        sprite: SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            // use the default values for all other components in the bundle
            ..Default::default()
        },
    });

    // Call .id() if you want to store the Entity ID of your new entity
    let my_entity = commands.spawn((/* ... */)).id();
}
// ANCHOR_END: commands

// ANCHOR: res
#[derive(Resource)]
struct GameSettings {
    current_level: u32,
    difficulty: u32,
    max_time_seconds: u32,
}

fn setup_game(
    mut commands: Commands,
) {
    // Add the GameSettings resource to the ECS
    // (if one already exists, it will be overwritten)
    commands.insert_resource(GameSettings {
        current_level: 1,
        difficulty: 100,
        max_time_seconds: 60,
    });
}

fn spawn_extra_enemies(
    mut commands: Commands,
    // we can easily access our resource from any system
    game_settings: Res<GameSettings>,
) {
    if game_settings.difficulty > 50 {
        commands.spawn((
            // ...
        ));
    }
}

// ANCHOR_END: res
