use bevy::{ecs::system::{RunSystemOnce, SystemId}, prelude::*, utils::HashMap};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health {
    hp: f32,
}

#[derive(Bundle, Default)]
struct MySparklesBundle {
}

#[derive(Event)]
enum MyMagicEvent {
    Sparkles,
}

// ANCHOR: example
fn item_handler_health_potion(
    mut q_player: Query<&mut Health, With<Player>>,
) {
    let mut health = q_player.single_mut();
    health.hp += 25.0;
}

fn item_handler_magic_potion(
    mut evw_magic: EventWriter<MyMagicEvent>,
    mut commands: Commands,
) {
    evw_magic.send(MyMagicEvent::Sparkles);
    commands.spawn(MySparklesBundle::default());
}
// ANCHOR_END: example

// ANCHOR: register-fromworld
/// For this simple example, we will just organize our systems
/// using string keys in a hash map.
#[derive(Resource)]
struct MyItemSystems(HashMap<String, SystemId>);

impl FromWorld for MyItemSystems {
    fn from_world(world: &mut World) -> Self {
        let mut my_item_systems = MyItemSystems(HashMap::new());

        my_item_systems.0.insert(
            "health".into(),
            world.register_system(item_handler_health_potion)
        );
        my_item_systems.0.insert(
            "magic".into(),
            world.register_system(item_handler_magic_potion)
        );

        my_item_systems
    }
}
// ANCHOR_END: register-fromworld

// ANCHOR: register-exclusive
fn register_item_handler_systems(world: &mut World) {
    let mut my_item_systems = MyItemSystems(HashMap::new());

    my_item_systems.0.insert(
        "health".into(),
        world.register_system(item_handler_health_potion)
    );
    my_item_systems.0.insert(
        "magic".into(),
        world.register_system(item_handler_magic_potion)
    );

    world.insert_resource(my_item_systems);
}
// ANCHOR_END: register-exclusive

// ANCHOR: run-commands
fn trigger_health_item(
    mut commands: Commands,
    systems: Res<MyItemSystems>,
) {
    // TODO: do some logic to implement picking up the health item

    let id = systems.0["health"];
    commands.run_system(id);
}
// ANCHOR_END: run-commands

// ANCHOR: run-exclusive
fn trigger_magic_item(world: &mut World) {
    // TODO: do some logic to implement picking up the magic item

    let id = world.resource::<MyItemSystems>().0["magic"];
    world.run_system(id).expect("Error Running Oneshot System");

    // Since we are in an exclusive system, we can expect
    // the magic potion to now be in effect!
}
// ANCHOR_END: run-exclusive

fn blah() {
    let mut world = World::new();

// ANCHOR: run-once
world.run_system_once(my_oneshot_system_fn);
// ANCHOR_END: run-once
}

fn my_oneshot_system_fn() {}

// ANCHOR: register-app
fn my_plugin(app: &mut App) {
    let mut my_item_systems = MyItemSystems(HashMap::new());

    my_item_systems.0.insert(
        "health".into(),
        app.register_system(item_handler_health_potion)
    );
    my_item_systems.0.insert(
        "magic".into(),
        app.register_system(item_handler_magic_potion)
    );

    app.insert_resource(my_item_systems);
}
// ANCHOR_END: register-app

fn main() {
    let mut app = App::new();

// ANCHOR: register-exclusive-app
app.add_systems(Startup, register_item_handler_systems);
// ANCHOR_END: register-exclusive-app
// ANCHOR: register-fromworld-app
app.init_resource::<MyItemSystems>();
// ANCHOR_END: register-fromworld-app

    app.add_systems(Update, trigger_health_item);
    app.add_systems(Update, trigger_magic_item);
}
