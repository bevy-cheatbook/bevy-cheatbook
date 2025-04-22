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
    // `single` checks that only one entity exists that matches the query
    if let Ok((name, mut xp, mut health)) = query_player.single_mut() {
        if xp.0 > 1000 {
            xp.0 = 0;
            health.hp = 100.0;
            health.extra += 25.0;
            info!("Player {} leveled up!", name.0);
        }
    };
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

// ANCHOR: required-camera
fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        // Add Camera2d to pull in everything needed for a 2D camera
        Camera2d,
        // Add our own transform
        Transform::from_xyz(1.0, 2.0, 3.0),

        // By just adding the above 2 components, our entity
        // will also automatically get other things, like:
        // - `Camera`: required by `Camera2d`
        // - `GlobalTransform`: required by `Transform`
        // - and various others … (see the API docs)
        // They will be initialized with default values.
        //
        // `Camera2d` also requires `Transform`, but we provided
        // our own value for that, so Bevy will use our value
        // instead of the default.
    ));
}
// ANCHOR_END: required-camera

mod require {
use bevy::prelude::*;
// ANCHOR: require-components
#[derive(Component)]
#[require(Enemy, Health, Sprite, Transform, Visibility)]
struct RegularEnemy;

#[derive(Component)]
#[require(
    Enemy,
    // init using a custom function instead of `Default`
    Health(500.0),
    Sprite,
    Transform,
    Visibility,
)]
struct BigBoss;
// ANCHOR_END: require-components
// ANCHOR: require-components-default
#[derive(Component, Default)]
struct Enemy;

#[derive(Component)]
struct Health(f32);

impl Default for Health {
    fn default() -> Self {
        Health(100.0)
    }
}
// ANCHOR_END: require-components-default

// ANCHOR: require-components-spawn
fn spawn_enemies(
    mut commands: Commands,
) {
    commands.spawn(BigBoss);
    commands.spawn((
        RegularEnemy,
        Transform::from_xyz(10.0, 20.0, 30.0),
    ));
}
// ANCHOR_END: require-components-spawn
fn _main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (
            spawn_enemies,
        ));
}
}

fn _main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (
            setup,
            level_up_player,
            make_enemies_friendly,
        ));
}
