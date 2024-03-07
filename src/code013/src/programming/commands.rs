use bevy::prelude::*;

#[derive(Component)]
struct ComponentA;
#[derive(Component, Default)]
struct ComponentB;
#[derive(Component, Default)]
struct ComponentC;
#[derive(Bundle, Default)]
struct MyBundle {
    b: ComponentB,
}
#[derive(Resource)]
struct MyResource;

impl MyResource {
    fn new() -> Self {
        Self
    }
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Friendly;
#[derive(Component)]
struct PlayerName(String);
#[derive(Component)]
struct PlayerXp(u32);
#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}
#[derive(Bundle)]
struct PlayerBundle {
    name: PlayerName,
    xp: PlayerXp,
    health: Health,
    _p: Player,
    sprite: SpriteBundle,
}

// ANCHOR: example-commands
fn spawn_things(
    mut commands: Commands,
) {
    // manage resources
    commands.insert_resource(MyResource::new());
    commands.remove_resource::<MyResource>();

    // create a new entity using `spawn`,
    // providing the data for the components it should have
    // (typically using a Bundle)
    commands.spawn(PlayerBundle {
        name: PlayerName("Henry".into()),
        xp: PlayerXp(1000),
        health: Health {
            hp: 100.0, extra: 20.0
        },
        _p: Player,
        sprite: Default::default(),
    });

    // you can use a tuple if you need additional components or bundles
    // (tuples of component and bundle types are considered bundles)
    // (note the extra parentheses)
    let my_entity_id = commands.spawn((
        // add some components
        ComponentA,
        ComponentB::default(),
        // add some bundles
        MyBundle::default(),
        TransformBundle::default(),
    )).id(); // get the Entity (id) by calling `.id()` at the end

    // add/remove components of an existing entity
    commands.entity(my_entity_id)
        .insert(ComponentC::default())
        .remove::<ComponentA>()
        .remove::<(ComponentB, MyBundle)>();
}

fn make_all_players_hostile(
    mut commands: Commands,
    // we need the Entity id, to perform commands on specific entities
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        commands.entity(entity)
            // add an `Enemy` component to the entity
            .insert(Enemy)
            // remove the `Friendly` component
            .remove::<Friendly>();
    }
}

fn despawn_all_enemies(
    mut commands: Commands,
    query: Query<Entity, With<Enemy>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// ANCHOR_END: example-commands

fn spawn_new_enemies_if_needed() {}
fn enemy_ai() {}

fn _main() {
let mut app = App::new();
// ANCHOR: order
app.add_systems(Update, spawn_new_enemies_if_needed);

// This system will see any newly-spawned enemies when it runs,
// because Bevy will make sure to apply the first system's Commands
// (thanks to the explicit `.after()` dependency)
app.add_systems(Update, enemy_ai.after(spawn_new_enemies_if_needed));
// ANCHOR_END: order
}
