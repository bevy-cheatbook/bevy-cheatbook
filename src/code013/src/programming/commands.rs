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

    // remove everything except the given components / bundles
    commands.entity(my_entity_id)
        .retain::<(TransformBundle, ComponentC)>();
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

// ANCHOR: command-closure
fn my_system(mut commands: Commands) {
    let x = 420;

    commands.add(move |world: &mut World| {
        // do whatever you want with `world` here

        // note: it's a closure, you can use variables from
        // the parent scope/function
        eprintln!("{}", x);
    });
}
// ANCHOR_END: command-closure

// ANCHOR: command-impl
use bevy::ecs::system::Command;

struct MyCustomCommand {
    // you can have some parameters
    data: u32,
}

impl Command for MyCustomCommand {
    fn apply(self, world: &mut World) {
        // do whatever you want with `world` and `self.data` here
    }
}

fn my_other_system(mut commands: Commands) {
    commands.add(MyCustomCommand {
        data: 920, // set your value
    });
}
// ANCHOR_END: command-impl

// ANCHOR: command-ext
pub trait MyCustomCommandsExt {
    // define a method that we will be able to call on `commands`
    fn do_custom_thing(&mut self, data: u32);
}

// implement our trait for Bevy's `Commands`
impl<'w, 's> MyCustomCommandsExt for Commands<'w, 's> {
    fn do_custom_thing(&mut self, data: u32) {
        self.add(MyCustomCommand {
            data,
        });
    }
}

fn my_fancy_system(mut commands: Commands) {
    // now we can call our custom method just like Bevy's `spawn`, etc.
    commands.do_custom_thing(42);
}
// ANCHOR_END: command-ext

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
