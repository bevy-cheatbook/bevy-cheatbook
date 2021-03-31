use bevy::prelude::*;

#[derive(Default)]
struct ComponentA;
#[derive(Default)]
struct ComponentB;
#[derive(Default)]
struct ComponentC;

struct MyResource;
struct ResourceA;
struct ResourceB;
struct ResourceC;

#[allow(dead_code)]
mod derive_systemparam {
use bevy::prelude::*;
pub struct UserKeybindings;
pub struct GameSaveSettings;
pub struct GraphicsSettings;
// ANCHOR: derive-system-param
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct MyCommonSettings<'a> {
    keys: Res<'a, UserKeybindings>,
    save: Res<'a, GameSaveSettings>,
    gfx: Res<'a, GraphicsSettings>,
}

fn read_all_settings(
    settings: MyCommonSettings,
) {
    // ...
}
// ANCHOR_END: derive-system-param

fn main() {
    App::build().add_system(read_all_settings.system()).run();
}

}

// ANCHOR: struct-component
struct Health {
    hp: f32,
    extra: f32,
}
// ANCHOR_END: struct-component

#[derive(Debug, PartialEq, Eq)]
// ANCHOR: newtype-component
struct PlayerXp(u32);

struct PlayerName(String);
// ANCHOR_END: newtype-component

// ANCHOR: marker-component
/// Add this to all menu ui entities to help identify them
struct MainMenuUI;
/// Marker to help identify the player
struct Player;
/// Marker for hostiles
struct Enemy;
// ANCHOR_END: marker-component

#[derive(Bundle, Default)]
struct MyBundle {
    _blah: bool,
}

#[derive(Bundle, Default)]
struct MyParentBundle {
    _blah: bool,
}

#[derive(Bundle, Default)]
struct MyChildBundle {
    _blah: bool,
}

// ANCHOR: bundle
#[derive(Bundle)]
struct PlayerBundle {
    xp: PlayerXp,
    name: PlayerName,
    health: Health,
    _p: Player,

    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    #[bundle]
    sprite: SpriteSheetBundle,
}
// ANCHOR_END: bundle

#[allow(dead_code)]
// ANCHOR: resource
struct GoalsReached {
    main_goal: bool,
    bonus: bool,
}
// ANCHOR_END: resource

#[derive(Default)]
struct MyOtherResource;

impl MyOtherResource {
    fn do_mut_stuff(&mut self) {}
}

// ANCHOR: fromworld
struct MyFancyResource { /* stuff */ }

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

#[derive(Debug)]
// ANCHOR: resource-default
#[derive(Default)]
struct StartingLevel(usize);
// ANCHOR_END: resource-default

#[derive(Default)]
struct MyState;
// ANCHOR: local-resource
fn my_system(mut local: Local<MyState>) {}
// ANCHOR_END: local-resource

// ANCHOR: sys-param-tuple
fn complex_system(
    (a, mut b): (Res<ResourceA>, ResMut<ResourceB>),
    // this resource might not exist, so wrap it in an Option
    mut c: Option<ResMut<ResourceC>>,
) {
    if let Some(mut c) = c {
        // do something
    }
}
// ANCHOR_END: sys-param-tuple

// ANCHOR: sys-debug-res
fn debug_start(
    // access resource
    start: Res<StartingLevel>
) {
    eprintln!("Starting on level {:?}", *start);
}
// ANCHOR_END: sys-debug-res

// ANCHOR: sys-simple-query
fn check_zero_health(
    // access entities that have `Health` and `Transform` components
    // get read-only access to `Health` and mutable access to `Transform`
    // optional component: get access to `Player` if it exists
    mut query: Query<(&Health, &mut Transform, Option<&Player>)>,
) {
    // get all matching entities
    for (health, mut transform, player) in query.iter_mut() {
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

// ANCHOR: sys-query-filter
fn debug_player_hp(
    // access the health, only for friendly players, optionally with name
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

// ANCHOR: sys-query-set
fn reset_health(
    // access the health of enemies and the health of players
    // (note: some entities could be both!)
    mut q: QuerySet<(
        Query<&mut Health, With<Enemy>>,
        Query<&mut Health, With<Player>>
    )>,
) {
    // set health of enemies
    for mut health in q.q0_mut().iter_mut() {
        health.hp = 50.0;
    }

    // set health of players
    for mut health in q.q1_mut().iter_mut() {
        health.hp = 100.0;
    }
}
// ANCHOR_END: sys-query-set

// ANCHOR: change-detection
/// Print the stats of friendly players when they change
fn debug_stats_change(
    query: Query<
        // components
        (&Health, &PlayerXp),
        // filters
        (Without<Enemy>, Or<(Changed<Health>, Changed<PlayerXp>)>), 
    >,
) {
    for (health, xp) in query.iter() {
        eprintln!(
            "hp: {}+{}, xp: {}",
            health.hp, health.extra, xp.0
        );
    }
}

/// detect new enemies and print their health
fn debug_new_hostiles(
    query: Query<(Entity, &Health), Added<Enemy>>,
) {
    for (entity, health) in query.iter() {
        eprintln!("Entity {:?} is now an enemy! HP: {}", entity, health.hp);
    }
}
// ANCHOR_END: change-detection

fn maybe_lvl_up(xp: &PlayerXp) -> PlayerXp {
    unimplemented!()
}

// ANCHOR: change-if-wrap
fn update_player_xp(
    mut query: Query<&mut PlayerXp>,
) {
    for mut xp in query.iter_mut() {
        let new_xp = maybe_lvl_up(&xp);

        // avoid triggering change detection if the value is the same
        if new_xp != *xp {
            *xp = new_xp;
        }
    }
}
// ANCHOR_END: change-if-wrap


use bevy::render::camera::Camera;
// ANCHOR: query-parent
fn camera_with_parent(
    q_child: Query<(&Parent, &Transform), With<Camera>>,
    q_parent: Query<&GlobalTransform>,
) {
    for (parent, child_transform) in q_child.iter() {
        // `parent` contains the Entity ID we can use
        // to query components from the parent:
        let parent_global_transform = q_parent.get(parent.0);

        // do something with the components
    }
}
// ANCHOR_END: query-parent

struct MySquadDamage;
struct MyUnitHealth;

// ANCHOR: query-child
fn process_squad_damage(
    q_parent: Query<(&MySquadDamage, &Children)>,
    q_child: Query<&MyUnitHealth>,
) {
    // get the properties of each squad
    for (squad_dmg, children) in q_parent.iter() {
        // `children` is a collection of Entity IDs
        for &child in children.iter() {
            // get the health of each child unit
            let health = q_child.get(child);

            // do something
        }
    }
}
// ANCHOR_END: query-child

// ANCHOR: example-commands
fn spawn_player(
    mut commands: Commands,
) {
    // manage resources
    commands.insert_resource(GoalsReached { main_goal: false, bonus: false });
    commands.remove_resource::<MyResource>();

    // create a new entity using `spawn`
    let entity_id = commands.spawn()
        // add a component
        .insert(ComponentA)
        // add a bundle
        .insert_bundle(MyBundle::default())
        // get the Entity ID
        .id();

    // shorthand for creating an entity with a bundle
    commands.spawn_bundle(PlayerBundle {
        name: PlayerName("Henry".into()),
        xp: PlayerXp(1000),
        health: Health {
            hp: 100.0, extra: 20.0
        },
        _p: Player,
        sprite: Default::default(),
    });

    // spawn another entity
    // NOTE: tuples of arbitrary components are valid bundles
    let other = commands.spawn_bundle((
        ComponentA::default(),
        ComponentB::default(),
        ComponentC::default(),
    )).id();

    // add/remove components of an existing entity
    commands.entity(entity_id)
        .insert(ComponentB)
        .remove::<ComponentA>()
        .remove_bundle::<MyBundle>();

    // despawn an entity
    commands.entity(other).despawn();
}

fn make_all_players_hostile(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        // add an `Enemy` component to the entity
        commands.entity(entity).insert(Enemy);
    }
}
// ANCHOR_END: example-commands

// ANCHOR: despawn-recursive
fn close_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in query.iter() {
        // despawn the entity and its children
        commands.entity(entity).despawn_recursive();
    }
}
// ANCHOR_END: despawn-recursive

// ANCHOR: events
struct LevelUpEvent(Entity);

fn player_level_up(
    mut ev_levelup: EventWriter<LevelUpEvent>,
    query: Query<(Entity, &PlayerXp)>,
) {
    for (entity, xp) in query.iter() {
        if xp.0 > 1000 {
            ev_levelup.send(LevelUpEvent(entity));
        }
    }
}

fn debug_levelups(
    mut ev_levelup: EventReader<LevelUpEvent>,
) {
    for ev in ev_levelup.iter() {
        eprintln!("Entity {:?} leveled up!", ev.0);
    }
}
// ANCHOR_END: events

// ANCHOR: asset-access
struct SpriteSheets {
    map_tiles: Handle<TextureAtlas>,
}

fn use_sprites(
    handles: Res<SpriteSheets>,
    assets: Res<Assets<TextureAtlas>>,
) {
    // Could be `None` if the asset isn't loaded yet
    if let Some(asset) = assets.get(&handles.map_tiles) {
        // do something with the texture atlas
    }
}
// ANCHOR_END: asset-access

// ANCHOR: asset-event
struct MapTexture {
    handle: Handle<Texture>,
}

fn fixup_textures(
    mut ev_asset: EventReader<AssetEvent<Texture>>,
    mut assets: ResMut<Assets<Texture>>,
    map_tex: Res<MapTexture>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            // a texture was just loaded!

            let texture = assets.get_mut(handle).unwrap();
            // ^ unwrap is OK, because we know it is loaded now

            if *handle == map_tex.handle {
                // it is our special map texture!
            } else {
                // it is some other texture
            }
        }
    }
}
// ANCHOR_END: asset-event

// ANCHOR: asset-server
struct UiFont(Handle<Font>);

fn load_ui_font(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    let handle: Handle<Font> = server.load("font.ttf");

    // store the handle in a resource, so we can
    // easily access it later to build UIs
    commands.insert_resource(UiFont(handle));
}
// ANCHOR_END: asset-server

// ANCHOR: asset-folder
struct ExtraAssets(Vec<HandleUntyped>);

fn load_extra_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    if let Ok(handles) = server.load_folder("extra") {
        commands.insert_resource(ExtraAssets(handles));
    }
}
// ANCHOR_END: asset-folder

fn commands_catchall(mut commands: Commands) {
// ANCHOR: ui-camera
commands.spawn_bundle(UiCameraBundle::default());
// ANCHOR_END: ui-camera

// ANCHOR: commands-current-entity
let e = commands.spawn().id();
// ANCHOR_END: commands-current-entity

// ANCHOR: commands-resource
commands.insert_resource(GoalsReached { main_goal: false, bonus: false });
commands.remove_resource::<MyResource>();
// ANCHOR_END: commands-resource

// ANCHOR: parenting
// spawn the parent and get its Entity id
let parent = commands.spawn_bundle(MyParentBundle::default())
    .id();

// do the same for the child
let child = commands.spawn_bundle(MyChildBundle::default())
    .id();

// add the child to the parent
commands.entity(parent).push_children(&[child]);

// you can also use `with_children`:
commands.spawn_bundle(MyParentBundle::default())
    .with_children(|parent| {
        parent.spawn_bundle(MyChildBundle::default());
    });
// ANCHOR_END: parenting
}

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
    let (player, mut transform) = q.single_mut()
        .expect("There should always be exactly one player in the game!");

    // do something with the player and its transform
}
// ANCHOR_END: query-single

fn query_misc(mut query: Query<(&Health, &mut Transform)>) {
    let entity = Entity::new(0);
    // ANCHOR: query-get
    if let Ok((health, mut transform)) = query.get_mut(entity) {
        // do something with the components
    } else {
        // the entity does not have the components from the query
    }
    // ANCHOR_END: query-get
}

#[allow(dead_code)]
mod app1 {
    use bevy::prelude::*;
    use super::*;

// ANCHOR: appinit-resource
fn main() {
    App::build()
        // if it implements `Default` or `FromWorld`
        .init_resource::<MyFancyResource>()
        // if not, or if you want to set a specific value
        .insert_resource(StartingLevel(3))
        // ...
        .run();
}
// ANCHOR_END: appinit-resource
}

#[allow(dead_code)]
mod app2 {
    use bevy::prelude::*;
    use super::*;

    fn setup() {}

// ANCHOR: app-builder
fn main() {
    App::build()
        // bevy
        .add_plugins(DefaultPlugins)

        // resources:
        .insert_resource(StartingLevel(3))
        // if it implements `Default` or `FromWorld`
        .init_resource::<MyFancyResource>()

        // events:
        .add_event::<LevelUpEvent>()

        // systems to run once at startup:
        .add_startup_system(spawn_player.system())

        // systems to run each frame:
        .add_system(player_level_up.system())
        .add_system(debug_levelups.system())
        .add_system(debug_stats_change.system())
        // ...

        // launch the app!
        .run();
}
// ANCHOR_END: app-builder
}

#[allow(dead_code)]
mod app3 {
    use bevy::prelude::*;
    use super::*;

    struct MyEvent;
    fn plugin_init() {}

// ANCHOR: plugins
struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<MyOtherResource>()
            .add_event::<MyEvent>()
            .add_startup_system(plugin_init.system())
            .add_system(my_system.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(MyPlugin)
        .run();
}
// ANCHOR_END: plugins
}

#[allow(dead_code)]
mod app4 {
    use bevy::prelude::*;
    use super::*;

    fn player_gather_xp() {}
    fn player_take_damage() {}

// ANCHOR: custom-stage
fn main() {
    // label for our debug stage
    static DEBUG: &str = "debug";

    App::build()
        .add_plugins(DefaultPlugins)

        // add DEBUG stage after Bevy's Update
        // also make it single-threaded
        .add_stage_after(CoreStage::Update, DEBUG, SystemStage::single_threaded())

        // systems are added to the `CoreStage::Update` stage by default
        .add_system(player_gather_xp.system())
        .add_system(player_take_damage.system())

        // add our debug systems
        .add_system_to_stage(DEBUG, debug_player_hp.system())
        .add_system_to_stage(DEBUG, debug_stats_change.system())
        .add_system_to_stage(DEBUG, debug_new_hostiles.system())

        .run();
}
// ANCHOR_END: custom-stage
}

#[allow(dead_code)]
#[allow(unused_imports)]
mod app5 {
    use bevy::prelude::*;
    use super::*;

// ANCHOR: check-state
fn play_music(
    app_state: Res<State<AppState>>,
    // ...
) {
    match app_state.current() {
        AppState::MainMenu => {
            // TODO: play menu music
        }
        AppState::InGame => {
            // TODO: play game music
        }
        AppState::Paused => {
            // TODO: play pause screen music
        }
    }
}
// ANCHOR_END: check-state

// ANCHOR: change-state
fn enter_game(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::InGame).unwrap();
    // ^ this can fail if we are already in the target state
    // or if another state change is already queued
}
// ANCHOR_END: change-state

fn pushpop(mut app_state: ResMut<State<AppState>>) {
// ANCHOR: state-push-pop
    // to go into the pause screen
    app_state.push(AppState::Paused).unwrap();
    // to go back into the game
    app_state.pop().unwrap();
// ANCHOR_END: state-push-pop
}

// ANCHOR: state-input-clear
fn esc_to_menu(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::MainMenu).unwrap();
        keys.reset(KeyCode::Escape);
    }
}
// ANCHOR_END: state-input-clear

// ANCHOR: app-states
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)

        // add the app state type
        .add_state(AppState::MainMenu)

        // add systems to run regardless of state, as usual
        .add_system(play_music.system())

        // systems to run only in the main menu
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(handle_ui_buttons.system())
        )

        // setup when entering the state
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_menu.system())
        )

        // cleanup when exiting the state
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(close_menu.system())
        )
        .run();
}
// ANCHOR_END: app-states

    fn animate_trees() {}
    fn animate_water() {}
    fn player_movement() {}
    fn reset_player() {}
    fn hide_player() {}
    fn setup_player() {}
    fn despawn_player() {}
    fn setup_map() {}
    fn despawn_map() {}
    fn handle_ui_buttons() {}
    fn setup_menu() {}
    fn close_menu() {}

fn main2() {
    App::build()
        .add_plugins(DefaultPlugins)
        // add the app state type
        .add_state(AppState::InGame)
// ANCHOR: state-stack
        // animate things even while paused
        .add_system_set(
            SystemSet::on_inactive_update(AppState::InGame)
                .with_system(animate_trees.system())
                .with_system(animate_water.system())
        )
        // player movement only when actively playing
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_movement.system())
        )
        // reset player when unpausing
        .add_system_set(
            SystemSet::on_resume(AppState::InGame)
                .with_system(reset_player.system())
        )
        // hide the player when pausing
        .add_system_set(
            SystemSet::on_pause(AppState::InGame)
                .with_system(hide_player.system())
        )
        // setup when first entering the game
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup_player.system())
                .with_system(setup_map.system())
        )
        // cleanup when finally exiting the game
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(despawn_player.system())
                .with_system(despawn_map.system())
        )
// ANCHOR_END: state-stack
        .run();
}
}

#[allow(dead_code)]
mod app6 {
use bevy::prelude::*;

struct MyNetProto;

impl MyNetProto {
    fn receive_updates(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// ANCHOR: system-io
fn net_receive(mut netcode: ResMut<MyNetProto>) -> std::io::Result<()> {
    netcode.receive_updates()?;

    Ok(())
}

fn handle_io_errors(In(result): In<std::io::Result<()>>) {
    if let Err(e) = result {
        eprintln!("I/O error occurred: {}", e);
    }
}
// ANCHOR_END: system-io
// ANCHOR: system-chain
fn main() {
    App::build()
        // ...
        .add_system(net_receive.system().chain(handle_io_errors.system()))
        // ...
        .run();
}
// ANCHOR_END: system-chain
}

#[allow(dead_code)]
mod app7 {
use bevy::prelude::*;

// ANCHOR: labels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(StageLabel)]
enum MyStages {
    Prepare,
    Cleanup,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // Add our custom stages:
        // note that Bevy's `CoreStage` is an enum just like ours!
        .add_stage_before(CoreStage::Update, MyStages::Prepare, SystemStage::parallel())
        .add_stage_after(CoreStage::Update, MyStages::Cleanup, SystemStage::parallel())
        // we can just use a string for this one:
        .add_stage_before(CoreStage::PostUpdate, "temp-debug-hack", SystemStage::parallel())
        .run();
}
// ANCHOR_END: labels
}

#[allow(dead_code)]
mod app8 {
use bevy::prelude::*;

    fn particle_effects() {}
    fn npc_behaviors() {}
    fn enemy_movement() {}
    fn map_player_input() {}
    fn update_map() {}
    fn input_parameters() {}
    fn player_movement() {}

// ANCHOR: system-labels
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)

        // order doesn't matter for these systems:
        .add_system(particle_effects.system())
        .add_system(npc_behaviors.system())
        .add_system(enemy_movement.system())

        // create labels, because we need to order other systems around these:
        .add_system(map_player_input.system().label("input"))
        .add_system(update_map.system().label("map"))

        // this will always run before anything labeled "input"
        .add_system(input_parameters.system().before("input"))

        // this will always run after anything labeled "input" and "map"
        // also label it just in case
        .add_system(
            player_movement.system()
                .label("player_movement")
                .after("input")
                .after("map")
        )
        .run();
}
// ANCHOR_END: system-labels
}

/// REGISTER ALL SYSTEMS TO DETECT COMPILATION ERRORS!
pub fn _main_all() {
    App::build()
        .add_startup_system(debug_start.system())
        .add_startup_system(load_ui_font.system())
        .add_startup_system(load_extra_assets.system())
        .add_system(commands_catchall.system())
        .add_system(query_entities.system())
        .add_system(query_player.system())
        .add_system(query_misc.system())
        .add_system(debug_player_hp.system())
        .add_system(debug_stats_change.system())
        .add_system(debug_new_hostiles.system())
        .add_system(check_zero_health.system())
        .add_system(reset_health.system())
        .add_system(my_system.system())
        .add_system(complex_system.system())
        .add_system(spawn_player.system())
        .add_system(close_menu.system())
        .add_system(make_all_players_hostile.system())
        .add_system(use_sprites.system())
        .add_system(fixup_textures.system())
        .add_system(update_player_xp.system())
        .add_system(camera_with_parent.system())
        .add_system(process_squad_damage.system())
        .run();
}
