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

struct MyComponent;

#[derive(Default)]
struct MyData;

struct MyEvent;

#[derive(Bundle)]
#[derive(Default)]
struct MyBundle;
#[derive(Default)]
#[derive(Bundle)]
struct ExtraBundle;
struct ExtraComponent;

#[derive(Default)]
#[derive(Bundle)]
struct MyParentBundle;
#[derive(Default)]
#[derive(Bundle)]
struct MyChildBundle;

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

// ANCHOR: fromresources
impl FromResources for MyFancyResource {
    fn from_resources(resources: &Resources) -> Self {
        // you have full access to any other resources you need here,
        // you can even mutate them:
        let mut x = resources.get_mut::<MyOtherResource>().unwrap();
        x.do_mut_stuff();

        MyFancyResource { /* stuff */ }
    }
}
// ANCHOR_END: fromresources

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
            .add_resource(MyResource::new())
            // auto-init using `Default` or `FromResources`
            .init_resource::<MyAutoResource>()
            .run();
    }
// ANCHOR_END: res-app-builder

fn res2(commands: &mut Commands) {
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
fn manage_entities_components_resources(commands: &mut Commands) {
    // build and spawn an entity
    let entity = commands.spawn(MyBundle::default())
        .with(ExtraComponent)
        .with_bundle(ExtraBundle::default())
        // get the entity id
        .current_entity().unwrap();

    // despawn a single entity
    commands.despawn(entity);

    // replaces any existing resource of this type
    commands.insert_resource(MyData::default());

    // add components to existing entity
    commands.insert_one(entity, MyComponent);
    commands.insert(entity, MyBundle::default());

    // remove
    commands.remove_one::<MyComponent>(entity);
    commands.remove::<MyBundle>(entity);
}
// ANCHOR_END: commands

// ANCHOR: parent-child
fn spawn_nested(commands: &mut Commands) {
    let parent = commands.spawn(MyParentBundle::default())
        .with_children(|parent| {
            parent.spawn(MyChildBundle::default());
        }).current_entity().unwrap();

    // despawn a parent and all its children
    commands.despawn_recursive(parent);
}
// ANCHOR_END: parent-child
fn spawn_nested2(commands: &mut Commands) {
    let parent = Entity::new(0);
    let child = Entity::new(0);

    // ANCHOR: add-parent
    commands.insert_one(child, Parent(parent));
    // ANCHOR_END: add-parent
}

pub mod systemchain {
use bevy::prelude::*;
use super::MyData;
// ANCHOR: system-chaining
fn output_system(/* system params */) -> MyData {
    MyData::default()
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

fn sender(mut events: ResMut<Events<MyEvent>>) {
    events.send(MyEvent);
}

fn receiver(
    events: Res<Events<MyEvent>>,
    mut reader: Local<EventReader<MyEvent>>,
) {
    for event in reader.iter(&events) {
        // handle event
    }
}

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
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
fn asset_events(
    events: Res<Events<AssetEvent<MyAssetType>>>,
    mut reader: Local<EventReader<AssetEvent<MyAssetType>>>
) {
    for event in reader.iter(&events) {
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
        .add_resource(MyResource::new())
        // auto-init using `Default` or `FromResources`
        .init_resource::<MyAutoResource>()
        // add a custom event type:
        .add_event::<MyEvent>()
        // run once at startup:
        .add_startup_system(setup.system())
        // run each frame (in `UPDATE` stage):
        .add_system(game_update.system())
        // add custom stage:
        .add_stage_before(stage::UPDATE, MY_START, SystemStage::parallel())
        // serial stage (parallel system execution disabled)
        .add_stage_after(stage::UPDATE, DEBUG, SystemStage::serial())
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
    fn handle_buttons() {}

// ANCHOR: states
#[derive(Debug, Clone, PartialEq, Eq)]
enum MyState {
    MainMenu,
    InGame,
}

fn main() {
    // label for the state stage
    static STATE: &str = "state";

    App::build()
        // add the app state resource with initial state
        .add_resource(State::new(MyState::MainMenu))
        // add the state stage
        .add_stage_before(
            stage::UPDATE, STATE,
            StateStage::<MyState>::default()
        )
        // this one goes in `UPDATE` as normal
        .add_system(state_independent.system())
        // add systems to states
        .on_state_enter(STATE, MyState::MainMenu, setup_menu.system())
        .on_state_update(STATE, MyState::MainMenu, handle_buttons.system())
        .on_state_exit(STATE, MyState::MainMenu, close_menu.system())
        .run();
}
// ANCHOR_END: states

// ANCHOR: res-state
fn manage_state(mut state: ResMut<State<MyState>>) {
    if *state.current() == MyState::MainMenu {
        state.set_next(MyState::InGame).unwrap();
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
