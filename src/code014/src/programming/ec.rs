use bevy::prelude::*;

// ANCHOR: component
#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}

#[derive(Component)]
enum AiMode {
    Passive,
    ChasingPlayer,
}
// ANCHOR_END: component

// ANCHOR: component-newtype
#[derive(Component)]
struct PlayerXp(u32);

#[derive(Component)]
struct PlayerName(String);
// ANCHOR_END: component-newtype

// ANCHOR: component-marker
/// Add this to all menu ui entities to help identify them
#[derive(Component)]
struct MainMenuUI;

/// Marker for hostile game units
#[derive(Component)]
struct Enemy;

/// This will be used to identify the main player entity
#[derive(Component)]
struct Player;

/// Tag all creatures that are currently friendly towards the player
#[derive(Component)]
struct Friendly;
// ANCHOR_END: component-marker

// ANCHOR: spawn-despawn
fn setup(mut commands: Commands) {
    // create a new entity
    commands.spawn((
        // Initialize all your components and bundles here
        Enemy,
        Health {
            hp: 100.0,
            extra: 25.0,
        },
        AiMode::Passive,
        // ...
    ));

    // If you want to get the Entity ID, just call `.id()` after spawn
    let my_entity = commands.spawn((/* ... */)).id();

    // destroy an entity, removing all data associated with it
    commands.entity(my_entity).despawn();
}
// ANCHOR_END: spawn-despawn

// ANCHOR: query
fn level_up_player(
    // get the relevant data. some components read-only, some mutable
    mut query_player: Query<(&PlayerName, &mut PlayerXp, &mut Health), With<Player>>,
) {
    // `single` assumes only one entity exists that matches the query
    let (name, mut xp, mut health) = query_player.single_mut();
    if xp.0 > 1000 {
        xp.0 = 0;
        health.hp = 100.0;
        health.extra += 25.0;
        info!("Player {} leveled up!", name.0);
    }
}

fn die(
    // `Entity` can be used to get the ID of things that match the query
    query_health: Query<(Entity, &Health)>,
    // we also need Commands, so we can despawn entities if we have to
    mut commands: Commands,
) {
    // we can have many such entities (enemies, player, whatever)
    // so we loop to check all of them
    for (entity_id, health) in query_health.iter() {
        if health.hp <= 0.0 {
            commands.entity(entity_id).despawn();
        }
    }
}
// ANCHOR_END: query

// ANCHOR: insert-remove
fn make_enemies_friendly(
    query_enemy: Query<Entity, With<Enemy>>,
    mut commands: Commands,
) {
    for entity_id in query_enemy.iter() {
        commands.entity(entity_id)
            .remove::<Enemy>()
            .insert(Friendly);
    }
}
// ANCHOR_END: insert-remove

fn _main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (
            setup,
            level_up_player,
            make_enemies_friendly,
        ));
}
