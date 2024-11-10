use bevy::{ecs::system::QueryLens, prelude::*};

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Planet;
#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,

}
#[derive(Component)]
struct EnemyAiState;

impl EnemyAiState {
    fn needs_recenter(&self) -> bool {
        true
    }
}

// ANCHOR: sys-simple-query
fn check_zero_health(
    // access entities that have `Health` and `Transform` components
    // get read-only access to `Health` and mutable access to `Transform`
    // optional component: get access to `Player` if it exists
    mut query: Query<(&Health, &mut Transform, Option<&Player>)>,
) {
    // get all matching entities
    for (health, mut transform, player) in &mut query {
        eprintln!("Entity at {} has {} HP.", transform.translation, health.hp);

        // center if hp is zero
        if health.hp <= 0.0 {
            transform.translation = Vec3::ZERO;
        }

        if let Some(player) = player {
            // the current entity is the player!
            // do something special!
        }
    }
}
// ANCHOR_END: sys-simple-query

// ANCHOR: iter-for-each
fn enemy_pathfinding(
    mut query_enemies: Query<(&Transform, &mut EnemyAiState)>,
) {
    query_enemies.iter_mut().for_each(|(transform, mut enemy_state)| {
        // TODO: do something with `transform` and `enemy_state`
    })
}
// ANCHOR_END: iter-for-each

// ANCHOR: sys-query-filter
fn debug_player_hp(
    // access the health (and optionally the PlayerName, if present), only for friendly players
    query: Query<(&Health, Option<&PlayerName>), (With<Player>, Without<Enemy>)>,
) {
    // get all matching entities
    for (health, name) in query.iter() {
        if let Some(name) = name {
            eprintln!("Player {} has {} HP.", name.0, health.hp);
        } else {
            eprintln!("Unknown player has {} HP.", health.hp);
        }
    }
}
// ANCHOR_END: sys-query-filter

// ANCHOR: query-entity
// add `Entity` to `Query` to get Entity IDs
fn query_entities(q: Query<(Entity, /* ... */)>) {
    for (e, /* ... */) in q.iter() {
        // `e` is the Entity ID of the entity we are accessing
    }
}
// ANCHOR_END: query-entity

// ANCHOR: query-single
fn query_player(mut q: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = q.single_mut();

    // do something with the player and its transform
}
// ANCHOR_END: query-single

// ANCHOR: query-many
#[derive(Resource)]
struct UiHudIndicators {
    // say we have 3 special UI elements
    entities_ui: [Entity; 3],
    entities_text: [Entity; 3],
}

fn update_ui_hud_indicators(
    indicators: Res<UiHudIndicators>,
    query_text: Query<&Text>,
    query_ui: Query<(&Style, &BackgroundColor)>,
) {
    // we can get everything as an array
    if let Ok(my_texts) = query_text.get_many(indicators.entities_text) {
        // the entities exist and match the query
        // TODO: something with `my_texts[0]`, `my_texts[1]`, `my_texts[2]`
    } else {
        // query unsuccessful
    };

    // we can use "destructuring syntax"
    // if we want to unpack everything into separate variables
    let [(style0, color0), (style1, color1), (style2, color2)] =
        query_ui.many(indicators.entities_ui);

    // TODO: something with all these variables
}
// ANCHOR_END: query-many

fn query_misc(mut query: Query<(&Health, &mut Transform)>) {
    let entity = Entity::from_raw(0);
    // ANCHOR: query-get
if let Ok((health, mut transform)) = query.get_mut(entity) {
    // do something with the components
} else {
    // the entity does not have the components from the query
}
    // ANCHOR_END: query-get
}

// ANCHOR: query-join
fn query_join(
    mut query_common: Query<(&Transform, &Health)>,
    mut query_player: Query<&PlayerName, With<Player>>,
    mut query_enemy: Query<&EnemyAiState, With<Enemy>>,
) {
    let mut player_with_common:
        QueryLens<(&Transform, &Health, &PlayerName), With<Player>> =
            query_player.join_filtered(&mut query_common);

    for (transform, health, player_name) in &player_with_common.query() {
        // TODO: do something with all these components
    }

    let mut enemy_with_common:
        QueryLens<(&Transform, &Health, &EnemyAiState), With<Enemy>> =
            query_enemy.join_filtered(&mut query_common);

    for (transform, health, enemy_ai) in &enemy_with_common.query() {
        // TODO: do something with all these components
    }
}
// ANCHOR_END: query-join

// ANCHOR: query-transmute
fn debug_positions(
    query: Query<&Transform>,
) {
    for transform in query.iter() {
        eprintln!("{:?}", transform.translation);
    }
}

fn move_player(
    mut query_player: Query<&mut Transform, With<Player>>,
) {
    // TODO: mutate the transform to move the player

    // say we want to call our debug_positions function

    // first, convert into a query for `&Transform`
    let mut lens = query_player.transmute_lens::<&Transform>();
    debug_positions(lens.query());
}

fn move_enemies(
    mut query_enemies: Query<&mut Transform, With<Enemy>>,
) {
    // TODO: mutate the transform to move our enemies

    let mut lens = query_enemies.transmute_lens::<&Transform>();
    debug_positions(lens.query());
}
// ANCHOR_END: query-transmute

// ANCHOR: query-combinations
fn print_potential_friends(
    q_player_names: Query<&PlayerName>,
) {
    // this will iterate over every possible pair of two entities
    // (that have the PlayerName component)

    for [player1, player2] in q_player_names.iter_combinations() {
        println!("Maybe {} could be friends with {}?", player1.0, player2.0);
    }
}

fn apply_gravity_to_planets(
    mut query: Query<&mut Transform, With<Planet>>,
) {
    // this will iterate over every possible pair of two planets

    // For mutability, we need a different syntax
    let mut combinations = query.iter_combinations_mut();
    while let Some([planet1, planet2]) = combinations.fetch_next() {
        // TODO: calculate the gravity force between the planets
    }
}
// ANCHOR_END: query-combinations

fn _main() {
    let mut app = App::new();
    app.add_systems(Update, (
        check_zero_health,
        debug_player_hp,
        query_entities,
        query_player,
        query_misc,
        query_join,
        move_enemies,
        move_player,
        enemy_pathfinding,
        debug_positions,
        update_ui_hud_indicators,
    ));
}
