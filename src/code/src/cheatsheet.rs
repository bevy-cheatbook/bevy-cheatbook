use bevy::prelude::*;
use bevy::reflect::TypeUuid;

#[derive(Default)]
struct MyAutoResource;

struct MyResource;

impl MyResource {
    fn new() -> Self {
        MyResource
    }
}

struct Foo;
struct Bar;

#[derive(Default)]
struct MyComponent;

#[derive(Default)]
struct MyData;

struct MyEvent;

#[derive(Bundle)]
#[derive(Default)]
struct MyBundle {
    _blah: u8,
}
#[derive(Default)]
#[derive(Bundle)]
struct ExtraBundle {
    _blah: u8,
}
#[derive(Default)]
struct ExtraComponent;

#[derive(Default)]
#[derive(Bundle)]
struct MyParentBundle {
    _blah: u8,
}
#[derive(Default)]
#[derive(Bundle)]
struct MyChildBundle {
    _blah: u8,
}

struct ComponentA;
struct ComponentB;
struct ComponentC;

// ANCHOR: bundle
#[derive(Bundle)]
struct AbBundle {
    a: ComponentA,
    b: ComponentB,
}
// ANCHOR_END: bundle

struct MyOtherResource;

impl MyOtherResource {
    fn do_mut_stuff(&mut self) {}
}

struct MyFancyResource { /* stuff */ }

// ANCHOR: fromworld
impl FromWorld for MyFancyResource {
    fn from_world(world: &mut World) -> Self {
        // You have full access to anything in the ECS from here.

        // For instance, you can mutate other resources:
        let mut x = world.get_resource_mut::<MyOtherResource>().unwrap();
        x.do_mut_stuff();

        MyFancyResource { /* stuff */ }
    }
}
// ANCHOR_END: fromworld

pub mod res {
use bevy::prelude::*;
use super::*;
struct MyResourceA;
struct MyResourceB;
// ANCHOR: res
fn my_system(a: Res<MyResourceA>, mut b: ResMut<MyResourceB>) {
    // do something with `a` and `b`
}
// ANCHOR_END: res

// ANCHOR: res-app-builder
    fn main() {
        App::build()
            // specific resource value
            .insert_resource(MyResource::new())
            // auto-init using `Default` or `FromWorld`
            .init_resource::<MyAutoResource>()
            .run();
    }
// ANCHOR_END: res-app-builder

fn res2(mut commands: Commands) {
// ANCHOR: res-commands
    commands.insert_resource(MyData::default());
// ANCHOR_END: res-commands
}

pub fn _silence_warnings() {
    let _ = App::build()
        .add_system(res2.system())
        .add_system(my_system.system());
    main()
}
}

// ANCHOR: query
fn my_system(
    mut query: Query<(Entity, &ComponentA, &mut ComponentB, Option<&mut ComponentC>)>
) {
    // operate on all matching entities
    for (entity, a, mut b, c) in query.iter_mut() {
        // do something with `a` and `b`; `entity` is the Entity id
        if let Some(mut c) = c {
            // do something with `c` if it exists
        }
    }
}
// ANCHOR_END: query

// ANCHOR: query-set
fn conflicting_queries(
    mut set: QuerySet<(
        // these queries cannot be used at the same time, due to mutability conflict
        Query<&mut MyData, With<ComponentA>>,
        Query<&mut MyData, With<ComponentB>>,
    )>,
) {
    for data in set.q0_mut().iter_mut() {
        // access `data` of entities that match the first query
    }
    for data in set.q1_mut().iter_mut() {
        // access `data` of entities that match the second query
    }
}
// ANCHOR_END: query-set

fn my_system2(mut query: Query<(&ComponentA, &mut ComponentB)>) {
    let entity = Entity::new(0);
// ANCHOR: query-one
    if let Ok((a, mut b)) = query.get_mut(entity) {
        // do something with `a` and `b`
    } else {
        // entity does not match the query
    }
// ANCHOR_END: query-one
}

// ANCHOR: query-filter
fn with_filter(query: Query<&MyData, (Without<ComponentA>, Or<(With<Foo>, With<Bar>)>)>) {
    // only get `data` for entities that match the filter criteria
    for data in query.iter() {
        // use data
    }
}
// ANCHOR_END: query-filter

// ANCHOR: change-detection
fn with_change_detection(query: Query<&MyData, Changed<MyData>>) {
    // only get `data` if it changed
    for data in query.iter() {
        // use data
    }
}
// ANCHOR_END: change-detection

// ANCHOR: local
fn with_local(local: Local<MyData>) {
    // `local` is an instance of `MyData` uniquely for this system
}
// ANCHOR_END: local

// ANCHOR: commands
fn manage_entities_components_resources(mut commands: Commands) {
    // replaces any existing resource of this type
    commands.insert_resource(MyData::default());

    // create/spawn a new entity
    let id = commands.spawn()
        // add a bundle
        .insert_bundle(MyBundle::default())
        // add a single component
        .insert(MyComponent::default())
        // get the entity id
        .id();

    // shorthand syntax for using a bundle
    commands.spawn_bundle(MyBundle::default())
        .insert(ExtraComponent);

    // add components to existing entity
    commands.entity(id)
        .insert_bundle(MyBundle::default())
        .insert(MyComponent::default());

    // remove components from entity
    commands.entity(id)
        .remove::<MyComponent>()
        .remove_bundle::<MyBundle>();

    // despawn an entity
    commands.entity(id).despawn();
}
// ANCHOR_END: commands

// ANCHOR: parent-child
fn spawn_nested(mut commands: Commands) {
    let parent = commands.spawn_bundle(MyParentBundle::default())
        .with_children(|parent| {
            parent.spawn_bundle(MyChildBundle::default());
        }).id();

    // despawn a parent and all its children
    commands.entity(parent).despawn_recursive();
}
// ANCHOR_END: parent-child
fn spawn_nested2(mut commands: Commands) {
    let parent = Entity::new(0);
    let child = Entity::new(0);

    // ANCHOR: add-parent
    commands.entity(parent).push_children(&[child]);
    // ANCHOR_END: add-parent
}

pub mod systemchain {
use bevy::prelude::*;
use super::MyData;
// ANCHOR: system-chaining
fn output_system(/* system params */) -> MyData {
    MyData
}

fn input_system(In(data): In<MyData>) {
    // use `data`
}

fn main() {
    App::build()
        .add_system(output_system.system().chain(input_system.system()))
        .run();
}
// ANCHOR_END: system-chaining

pub fn _silence_warnings() {
    main()
}
}

pub mod events {
use bevy::prelude::*;
// ANCHOR: events
struct MyEvent;

fn sender(mut writer: EventWriter<MyEvent>) {
    writer.send(MyEvent);
}

fn receiver(mut reader: EventReader<MyEvent>) {
    for event in reader.iter() {
        // handle event
    }
}

fn main() {
    App::build()
        .add_event::<MyEvent>()
        // ...
        .run();
}
// ANCHOR_END: events

pub fn _silence_warnings() {
    let _ = App::build()
        .add_system(sender.system())
        .add_system(receiver.system());
    main()
}
}

#[derive(TypeUuid)]
#[uuid = "5ea5a728-4d83-4a20-9e6e-3ba86bab7d75"]
struct MyAssetType;

impl MyAssetType {
    fn new() -> Self {
        Self
    }
}

#[derive(Default)]
struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut AppBuilder) {
    }
}

fn setup() {}
fn game_update() {}
fn debug_system() {}

// ANCHOR: asset-load
fn asset_setup(server: Res<AssetServer>) {
    // load path as an asset of specified type
    let handle: Handle<MyAssetType> = server.load("path/to/asset.file");
    // autodetect type based on file extension
    let untyped = server.load_untyped("path/to/file.png");
    // load a whole folder
    let handles = server.load_folder("path/to/folder");
}
// ANCHOR_END: asset-load

// ANCHOR: asset-use
// store the handle somewhere (in a component or resource)
struct MyAsset(Handle<MyAssetType>);

fn asset_use(assets: Res<Assets<MyAssetType>>, my: Res<MyAsset>) {
    // the asset might not have finished loading yet
    if let Some(data) = assets.get(&my.0) {
        // it's ready
    } else {
        // it's not ready
    }
}
// ANCHOR_END: asset-use

// ANCHOR: asset-new
fn generate_asset(mut assets: ResMut<Assets<MyAssetType>>) {
    assets.add(MyAssetType::new());
}
// ANCHOR_END: asset-new

// ANCHOR: asset-event
fn asset_events(mut reader: EventReader<AssetEvent<MyAssetType>>) {
    for event in reader.iter() {
        match event {
            AssetEvent::Created { handle } => {
                // asset just finished loading
            }
            AssetEvent::Modified { handle } => {
                // asset was changed
            }
            AssetEvent::Removed { handle } => {
                // asset was unloaded
            }
        }
    }
}
// ANCHOR_END: asset-event

pub mod appbuilder{
use super::*;
// ANCHOR: app-builder
fn main() {
    // labels for custom stages:
    static DEBUG: &str = "debug";
    static MY_START: &str = "my_start";

    App::build()
        // bevy
        .add_plugins(DefaultPlugins)
        // custom plugin
        .add_plugin(MyPlugin::default())
        // specific resource value
        .insert_resource(MyResource::new())
        // auto-init using `Default` or `FromWorld`
        .init_resource::<MyAutoResource>()
        // add a custom event type:
        .add_event::<MyEvent>()
        // run once at startup:
        .add_startup_system(setup.system())
        // run each frame (in `UPDATE` stage):
        .add_system(game_update.system())
        // add custom stage:
        .add_stage_before(CoreStage::Update, MY_START, SystemStage::parallel())
        // serial stage (parallel system execution disabled)
        .add_stage_after(CoreStage::Update, DEBUG, SystemStage::single_threaded())
        // run system in specific stage:
        .add_system_to_stage(DEBUG, debug_system.system())
        // enter the bevy runtime
        .run();

}
// ANCHOR_END: app-builder

#[derive(Default)]
// ANCHOR: plugin
struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // use `app` builder
    }
}
// ANCHOR_END: plugin

pub fn _silence_warnings() {
    main()
}
}

pub mod appstates{
use super::*;
    fn state_independent() {}
    fn setup_menu() {}
    fn close_menu() {}
    fn update_player() {}

// ANCHOR: states
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MyState {
    MainMenu,
    InGame,
}

fn main() {
    App::build()
        // add the app state type
        .add_state(MyState::MainMenu)
        // this one will run regardless of state
        .add_system(state_independent.system())
        // add systems to states
        .add_system_set(
            SystemSet::on_enter(MyState::MainMenu)
                .with_system(setup_menu.system())
        )
        .add_system_set(
            SystemSet::on_exit(MyState::MainMenu)
                .with_system(close_menu.system())
        )
        .add_system_set(
            SystemSet::on_update(MyState::InGame)
                .with_system(update_player.system())
        )
        .run();
}
// ANCHOR_END: states

// ANCHOR: res-state
fn manage_state(mut state: ResMut<State<MyState>>) {
    if *state.current() == MyState::MainMenu {
        // FIXME new states
        state.set(MyState::InGame).unwrap();
        // ^ error if another state change is pending
        // or if already in the target state
    }
}
// ANCHOR_END: res-state

pub fn _silence_warnings() {
    let _ = App::build()
        .add_system(manage_state.system());
    main();
}
}

pub fn _use() {
    let _ = App::build()
        .init_resource::<MyFancyResource>()
        .add_startup_system(asset_setup.system())
        .add_system(asset_events.system())
        .add_system(with_local.system())
        .add_system(with_filter.system())
        .add_system(with_change_detection.system())
        .add_system(asset_use.system())
        .add_system(generate_asset.system())
        .add_system(conflicting_queries.system())
        .add_system(my_system.system())
        .add_system(my_system2.system())
        .add_system(spawn_nested.system())
        .add_system(spawn_nested2.system())
        .add_system(manage_entities_components_resources.system())
        .run();
}
