#![allow(dead_code)]
#![allow(unreachable_code)]

use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;

#[derive(Default)]
#[derive(Component)]
struct ComponentA;
#[derive(Default)]
#[derive(Component)]
struct ComponentB;
#[derive(Default)]
#[derive(Component)]
struct ComponentC;

#[derive(Resource)]
struct MyResource;
#[derive(Resource)]
struct ResourceA;
#[derive(Resource)]
struct ResourceB;
#[derive(Resource)]
struct ResourceC;

#[allow(dead_code)]
mod derive_systemparam {
use bevy::prelude::*;
#[derive(Resource)]
pub struct UserKeybindings;
#[derive(Resource)]
pub struct GameSaveSettings;
#[derive(Resource)]
pub struct GraphicsSettings;
// ANCHOR: derive-system-param
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
/// Define a common set of system parameters.
/// 'w: the World lifetime, needed for resources and queries (anything fetching data from the ECS).
/// 's: the system lifetime, needed for Locals and queries (anything with per-system state).
pub struct MyCommonSettings<'w, 's> {
    keys: Res<'w, UserKeybindings>,
    save: Res<'w, GameSaveSettings>,
    gfx: Res<'w, GraphicsSettings>,
    extra: Local<'s, bool>,
    q_transforms: Query<'w, 's, &'static Transform>,
}

fn read_all_settings(
    // we can just access our stuff from here now
    settings: MyCommonSettings,
) {
    // ...
}
// ANCHOR_END: derive-system-param

fn main() {
    App::new().add_system(read_all_settings).run();
}

}

// ANCHOR: component-storage
/// Component for entities that can cast magic spells
#[derive(Component)] // Use the default table storage
struct Mana {
    mana: f32,
}

/// Component for enemies that currently "see" the player
/// Every frame, add/remove to entities based on visibility
/// (use sparse-set storage due to frequent add/remove)
#[derive(Component)]
#[component(storage = "SparseSet")]
struct CanSeePlayer;

/// Component for entities that are currently taking bleed damage
/// Add to entities to apply bleed effect, remove when done
/// (use sparse-set storage to not fragment tables,
/// as this is a "temporary effect")
#[derive(Component)]
#[component(storage = "SparseSet")]
struct Bleeding {
    damage_rate: f32,
}
// ANCHOR_END: component-storage

// ANCHOR: struct-component
#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}
// ANCHOR_END: struct-component

#[derive(Debug, PartialEq, Eq)]
// ANCHOR: newtype-component
#[derive(Component)]
struct PlayerXp(u32);

#[derive(Component)]
struct PlayerName(String);
// ANCHOR_END: newtype-component

// ANCHOR: marker-component
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
// ANCHOR_END: marker-component

#[derive(Bundle, Default)]
struct MyBundle {
    _blah: ComponentA,
}

#[derive(Bundle, Default)]
struct MyParentBundle {
    _blah: ComponentA,
}

#[derive(Bundle, Default)]
struct MyChildBundle {
    _blah: ComponentA,
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
#[derive(Resource)]
struct GoalsReached {
    main_goal: bool,
    bonus: bool,
}
// ANCHOR_END: resource

#[derive(Resource, Default)]
struct MyOtherResource;

impl MyOtherResource {
    fn do_mut_stuff(&mut self) {}
}

// ANCHOR: fromworld
#[derive(Resource)]
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

// ANCHOR: resource-default
#[derive(Resource, Default, Debug)]
struct StartingLevel(usize);
// ANCHOR_END: resource-default

// ANCHOR: local-resource
#[derive(Default)]
struct MyState;

fn my_system1(mut local: Local<MyState>) {
    // you can do anything you want with the local here
}

fn my_system2(mut local: Local<MyState>) {
    // the local in this system is a different instance
}
// ANCHOR_END: local-resource

#[allow(dead_code)]
mod localconfig {
use bevy::prelude::*;
#[derive(Resource)]
struct MyStuff;
// ANCHOR: local-config
#[derive(Default)]
struct MyConfig {
    magic: usize,
}

fn my_system(
    mut cmd: Commands,
    my_res: Res<MyStuff>,
    // note this isn't a valid system parameter
    config: &MyConfig,
) {
    // TODO: do stuff
}

fn main() {
    let config = MyConfig {
        magic: 420,
    };

    App::new()
        .add_plugins(DefaultPlugins)
        // create a "move closure", so we can use the `config`
        // variable that we created above
        .add_system(move |cmd: Commands, res: Res<MyStuff>| {
            // call our function from inside the closure
            my_system(cmd, res, &config);
        })
        .run();
}
// ANCHOR_END: local-config
}

#[allow(dead_code)]
mod localconfig2 {
use bevy::prelude::*;
#[derive(Resource)]
struct MyStuff;
// ANCHOR: local-config-return
#[derive(Default)]
struct MyConfig {
    magic: usize,
}

fn main() {
    // create a "constructor" closure, which can initialize
    // our data and move it into a closure that bevy can run as a system
    let constructor = || {
        // create the `MyConfig`
        let config = MyConfig {
            magic: 420,
        };

        // this is the actual system that bevy will run
        move |mut commands: Commands, res: Res<MyStuff>| {
            // we can use `config` here, the value from above will be "moved in"
            // we can also use our system params: `commands`, `res`
        }
    };

    App::new()
        .add_plugins(DefaultPlugins)
        // note the parentheses `()`
        // we are calling the "constructor" we made above,
        // which will return the actual system that gets added to bevy
        .add_system(constructor())
        .run();
}
// ANCHOR_END: local-config-return
}

#[allow(dead_code)]
mod pluginconfig {
use bevy::prelude::*;
fn health_system() {}
fn movement_system() {}
fn player_invincibility() {}
fn free_camera() {}
// ANCHOR: plugin-config

struct MyGameplayPlugin {
    /// Should we enable dev hacks?
    enable_dev_hacks: bool,
}

impl Plugin for MyGameplayPlugin {
    fn build(&self, app: &mut App) {
        // add our gameplay systems
        app.add_system(health_system);
        app.add_system(movement_system);
        // ...

        // if "dev mode" is enabled, add some hacks
        if self.enable_dev_hacks {
            app.add_system(player_invincibility);
            app.add_system(free_camera);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MyGameplayPlugin {
            enable_dev_hacks: false, // change to true for dev testing builds
        })
        .run();
}
// ANCHOR_END: plugin-config

fn _main2() {
// ANCHOR: defaultplugins-config
App::new()
    .add_plugins(DefaultPlugins.set(
        // here we configure the main window
        WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 600.0,
                // ...
                ..Default::default()
            },
            ..Default::default()
        }
    ))
    .run();
// ANCHOR_END: defaultplugins-config
}
}

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

// ANCHOR: sys-query-set
fn reset_health(
    // access the health of enemies and the health of players
    // (note: some entities could be both!)
    mut set: ParamSet<(
        Query<&mut Health, With<Enemy>>,
        Query<&mut Health, With<Player>>,
        // also access the whole world ... why not
        &World,
    )>,
) {
    // set health of enemies (use the 1st param in the set)
    for mut health in set.p0().iter_mut() {
        health.hp = 50.0;
    }

    // set health of players (use the 2nd param in the set))
    for mut health in set.p1().iter_mut() {
        health.hp = 100.0;
    }

    // read some data from the world (use the 3rd param in the set)
    let my_resource = set.p2().resource::<MyResource>();

    // since we only used the conflicting system params one at a time,
    // everything is safe and our code can compile; ParamSet guarantees this
}
// ANCHOR_END: sys-query-set

fn example_transforms() {
// ANCHOR: transform-init
// To simply position something at specific coordinates
let xf_pos567 = Transform::from_xyz(5.0, 6.0, 7.0);

// To scale an object, making it twice as big in all dimensions
let xf_scale = Transform::from_scale(Vec3::splat(2.0));

// To rotate an object in 2D (Z-axis rotation) by 30°
// (angles are in radians! must convert from degrees!)
let xf_rot2d = Transform::from_rotation(Quat::from_rotation_z((30.0_f32).to_radians()));

// 3D rotations can be complicated; explore the methods available on `Quat`

// Simple 3D rotation by Euler-angles (X, Y, Z)
let xf_rot2d = Transform::from_rotation(Quat::from_euler(
    EulerRot::XYZ,
    (20.0_f32).to_radians(),
    (10.0_f32).to_radians(),
    (30.0_f32).to_radians(),
));

// Everything:
let xf = Transform::from_xyz(1.0, 2.0, 3.0)
    .with_scale(Vec3::new(0.5, 0.5, 1.0))
    .with_rotation(Quat::from_rotation_y(0.125 * std::f32::consts::PI));
// ANCHOR_END: transform-init
}

#[derive(Component)]
struct Balloon;

// ANCHOR: transform-mut
fn inflate_balloons(
    mut query: Query<&mut Transform, With<Balloon>>,
    keyboard: Res<Input<KeyCode>>,
) {
    // every time the Spacebar is pressed,
    // make all the balloons in the game bigger by 25%
    if keyboard.just_pressed(KeyCode::Space) {
        for mut transform in query.iter_mut() {
            transform.scale *= 1.25;
        }
    }
}
// ANCHOR_END: transform-mut

// ANCHOR: spatialbundle
fn spawn_special_entity(
    mut commands: Commands,
) {
    // create an entity that does not use one of the common Bevy bundles,
    // but still needs transforms and visibility
    commands.spawn((
        ComponentA,
        ComponentB,
        SpatialBundle {
            transform: Transform::from_scale(Vec3::splat(3.0)),
            visibility: Visibility {
                is_visible: false,
            },
            ..Default::default()
        },
    ));
}
// ANCHOR_END: spatialbundle

fn insert_visibilitybundle(
    mut commands: Commands,
) {
    let parent = Entity::from_raw(0);
// ANCHOR: insert-visibilitybundle
    commands.entity(parent)
        .insert(VisibilityBundle::default());
// ANCHOR_END: insert-visibilitybundle
}

// ANCHOR: propagation
fn spawn_toplevel_entity(
    mut commands: Commands,
) {
    // this can be a top-level entity that controls a hierarchy of children
    let parent = commands.spawn(SpatialBundle {
        transform: Transform::from_scale(Vec3::splat(3.0)),
        visibility: Visibility {
            is_visible: false,
        },
        ..Default::default()
    }).id();

    // Child transforms behave relative to the parent.
    // For visibility: if the parent is hidden, that also hides the children.
    let child = commands.spawn(SpatialBundle {
        transform: Transform::from_xyz(1.0, 2.0, 3.0),
        ..Default::default()
    }).id();

   commands.entity(parent).push_children(&[child]);
}
// ANCHOR_END: propagation

#[derive(Component)]
struct GameMapEntity;

// ANCHOR: visibility
/// Prepare the game map, but do not display it until later
fn setup_map_hidden(
    mut commands: Commands,
) {
    commands.spawn((
        GameMapEntity,
        SceneBundle {
            scene: todo!(),
            visibility: Visibility {
                is_visible: false,
            },
            ..Default::default()
        },
    ));
}

/// When everything is ready, un-hide the game map
fn reveal_map(
    mut query: Query<&mut Visibility, With<GameMapEntity>>,
) {
    let mut vis_map = query.single_mut();
    vis_map.is_visible = true;
}
// ANCHOR_END: visibility

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

// ANCHOR: changetrackers
/// Make sprites flash red on frames when the Health changes
fn debug_damage(
    mut query: Query<(&mut Sprite, ChangeTrackers<Health>)>,
) {
    for (mut sprite, tracker) in query.iter_mut() {
        // detect if the Health changed this frame
        if tracker.is_changed() {
            sprite.color = Color::RED;
        } else {
            // extra check so we don't mutate on every frame without changes
            if sprite.color != Color::WHITE {
                sprite.color = Color::WHITE;
            }
        }
    }
}
// ANCHOR_END: changetrackers

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

// ANCHOR: changed-res
fn check_res_changed(
    my_res: Res<MyResource>,
) {
    if my_res.is_changed() {
        // do something
    }
}

fn check_res_added(
    // use Option, not to panic if the resource doesn't exist yet
    my_res: Option<Res<MyResource>>,
) {
    if let Some(my_res) = my_res {
        // the resource exists

        if my_res.is_added() {
            // it was just added
            // do something
        }
    }
}
// ANCHOR_END: changed-res

// ANCHOR: res-removal-detection
fn detect_removed_res(
    my_res: Option<Res<MyResource>>,
    mut my_res_existed: Local<bool>,
) {
    if let Some(my_res) = my_res {
        // the resource exists!

        // remember that!
        *my_res_existed = true;

        // (... you can do something with the resource here if you want ...)
    } else if *my_res_existed {
        // the resource does not exist, but we remember it existed!
        // (it was removed)

        // forget about it!
        *my_res_existed = false;

        // ... do something now that it is gone ...
    }
}
// ANCHOR_END: res-removal-detection

use bevy::render::camera::Camera;
// ANCHOR: query-parent
fn camera_with_parent(
    q_child: Query<(&Parent, &Transform), With<Camera>>,
    q_parent: Query<&GlobalTransform>,
) {
    for (parent, child_transform) in q_child.iter() {
        // `parent` contains the Entity ID we can use
        // to query components from the parent:
        let parent_global_transform = q_parent.get(parent.get());

        // do something with the components
    }
}
// ANCHOR_END: query-parent

#[derive(Component)]
struct MySquadDamage;
#[derive(Component)]
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

fn despawn_child(
    mut commands: Commands,
) {
    let parent_entity = Entity::from_raw(0);
    let child_entity = Entity::from_raw(0);
// ANCHOR: despawn-child
    commands.entity(parent_entity).remove_children(&[child_entity]);
    commands.entity(child_entity).despawn();
// ANCHOR_END: despawn-child
}

// ANCHOR: example-commands
fn spawn_things(
    mut commands: Commands,
) {
    // manage resources
    commands.insert_resource(GoalsReached { main_goal: false, bonus: false });
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

// ANCHOR: asset-access
#[derive(Resource)]
struct SpriteSheets {
    map_tiles: Handle<TextureAtlas>,
}

fn use_sprites(
    handles: Res<SpriteSheets>,
    atlases: Res<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
) {
    // Could be `None` if the asset isn't loaded yet
    if let Some(atlas) = atlases.get(&handles.map_tiles) {
        // do something with the texture atlas
    }
}
// ANCHOR_END: asset-access

// ANCHOR: asset-add
fn add_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let new_mat = StandardMaterial {
        base_color: Color::rgba(0.25, 0.50, 0.75, 1.0),
        unlit: true,
        ..Default::default()
    };

    let handle = materials.add(new_mat);

    // do something with the handle
}
// ANCHOR_END: asset-add

// ANCHOR: asset-event
#[derive(Resource)]
struct MyMapImage {
    handle: Handle<Image>,
}

fn fixup_images(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
    map_img: Res<MyMapImage>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                // a texture was just loaded or changed!

                // WARNING: this mutable access will cause another
                // AssetEvent (Modified) to be emitted!
                let texture = assets.get_mut(handle).unwrap();
                // ^ unwrap is OK, because we know it is loaded now

                if *handle == map_img.handle {
                    // it is our special map image!
                } else {
                    // it is some other image
                }
            }
            AssetEvent::Modified { handle } => {
                // an image was modified
            }
            AssetEvent::Removed { handle } => {
                // an image was unloaded
            }
        }
    }
}
// ANCHOR_END: asset-event

// ANCHOR: asset-server
#[derive(Resource)]
struct UiFont(Handle<Font>);

fn load_ui_font(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    let handle: Handle<Font> = server.load("font.ttf");

    // we can store the handle in a resource:
    //  - to prevent the asset from being unloaded
    //  - if we want to use it to access the asset later
    commands.insert_resource(UiFont(handle));
}
// ANCHOR_END: asset-server

// ANCHOR: asset-path-labels
fn load_gltf_things(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    // get a specific mesh
    let my_mesh: Handle<Mesh> = server.load("my_scene.gltf#Mesh0/Primitive0");

    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("my_scene.gltf#Scene0");
    commands.spawn(SceneBundle {
        scene: my_scene,
        ..Default::default()
    });
}
// ANCHOR_END: asset-path-labels

// ANCHOR: asset-folder
#[derive(Resource)]
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

// ANCHOR: spawn-gltf-simple
fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("my.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(2.0, 0.0, -5.0),
        ..Default::default()
    });
}
// ANCHOR_END: spawn-gltf-simple

// ANCHOR: gltf-complex
use bevy::gltf::Gltf;

/// Helper resource for tracking our asset
#[derive(Resource)]
struct MyAssetPack(Handle<Gltf>);

fn load_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let gltf = ass.load("my_asset_pack.glb");
    commands.insert_resource(MyAssetPack(gltf));
}

fn spawn_gltf_objects(
    mut commands: Commands,
    my: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&my.0) {
        // spawn the first scene in the file
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..Default::default()
        });

        // spawn the scene named "YellowCar"
        commands.spawn(SceneBundle {
            scene: gltf.named_scenes["YellowCar"].clone(),
            transform: Transform::from_xyz(1.0, 2.0, 3.0),
            ..Default::default()
        });

        // PERF: the `.clone()`s are just for asset handles, don't worry :)
    }
}
// ANCHOR_END: gltf-complex

// ANCHOR: gltf-manual-pbr
use bevy::gltf::GltfMesh;

fn gltf_manual_entity(
    mut commands: Commands,
    my: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    if let Some(gltf) = assets_gltf.get(&my.0) {
        // Get the GLTF Mesh named "CarWheel"
        // (unwrap safety: we know the GLTF has loaded already)
        let carwheel = assets_gltfmesh.get(&gltf.named_meshes["CarWheel"]).unwrap();

        // Spawn a PBR entity with the mesh and material of the first GLTF Primitive
        commands.spawn(PbrBundle {
            mesh: carwheel.primitives[0].mesh.clone(),
            // (unwrap: material is optional, we assume this primitive has one)
            material: carwheel.primitives[0].material.clone().unwrap(),
            ..Default::default()
        });
    }
}
// ANCHOR_END: gltf-manual-pbr

// ANCHOR: gltf-assetpath
fn use_gltf_things(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // spawn the first scene in the file
    let scene0 = ass.load("my_asset_pack.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: scene0,
        ..Default::default()
    });

    // spawn the second scene
    let scene1 = ass.load("my_asset_pack.glb#Scene1");
    commands.spawn(SceneBundle {
        scene: scene1,
        transform: Transform::from_xyz(1.0, 2.0, 3.0),
        ..Default::default()
    });
}
// ANCHOR_END: gltf-assetpath

fn commands_catchall(mut commands: Commands) {

// ANCHOR: sprite-flipping
commands.spawn(SpriteBundle {
    sprite: Sprite {
        flip_y: true,
        flip_x: false,
        ..Default::default()
    },
    ..Default::default()
});
// ANCHOR_END: sprite-flipping

// ANCHOR: commands-current-entity
let e = commands.spawn(()).id();
// ANCHOR_END: commands-current-entity

// ANCHOR: commands-resource
commands.insert_resource(GoalsReached { main_goal: false, bonus: false });
commands.remove_resource::<MyResource>();
// ANCHOR_END: commands-resource

// ANCHOR: parenting
// spawn the parent and get its Entity id
let parent = commands.spawn(MyParentBundle::default()).id();

// do the same for the child
let child = commands.spawn(MyChildBundle::default()).id();

// add the child to the parent
commands.entity(parent).push_children(&[child]);

// you can also use `with_children`:
commands.spawn(MyParentBundle::default())
    .with_children(|parent| {
        parent.spawn(MyChildBundle::default());
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
    let (player, mut transform) = q.single_mut();

    // do something with the player and its transform
}
// ANCHOR_END: query-single

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

#[allow(dead_code)]
mod app0 {
    use bevy::prelude::*;

    fn init_menu() {}
    fn debug_start() {}
    fn move_player() {}
    fn enemies_ai() {}

// ANCHOR: systems-appbuilder
fn main() {
    App::new()
        // ...

        // run it only once at launch
        .add_startup_system(init_menu)
        .add_startup_system(debug_start)

        // run it every frame update
        .add_system(move_player)
        .add_system(enemies_ai)

        // ...
        .run();
}
// ANCHOR_END: systems-appbuilder
}

#[derive(Component)]
struct Asteroid;

// ANCHOR: time-delta
fn asteroids_fly(
    time: Res<Time>,
    mut q: Query<&mut Transform, With<Asteroid>>,
) {
    for mut transform in q.iter_mut() {
        // move our asteroids along the X axis
        // at a speed of 10.0 units per second
        transform.translation.x += 10.0 * time.delta_seconds();
    }
}
// ANCHOR_END: time-delta

// ANCHOR: time-monotonic
use std::time::Instant;

/// Say, for whatever reason, we want to keep track
/// of when exactly some specific entities were spawned.
#[derive(Component)]
struct SpawnedTime(Instant);

fn spawn_my_stuff(
    mut commands: Commands,
    time: Res<Time>,
) {
    commands.spawn((/* ... */))
        // we can use startup time and elapsed duration
        .insert(SpawnedTime(time.startup() + time.elapsed()))
        // or just the time of last update
        .insert(SpawnedTime(time.last_update().unwrap()));
}
// ANCHOR_END: time-monotonic

// ANCHOR: timer
use std::time::Duration;

#[derive(Component)]
struct FuseTime {
    /// track when the bomb should explode (non-repeating timer)
    timer: Timer,
}

fn explode_bombs(
    mut commands: Commands,
    mut q: Query<(Entity, &mut FuseTime)>,
    time: Res<Time>,
) {
    for (entity, mut fuse_timer) in q.iter_mut() {
        // timers gotta be ticked, to work
        fuse_timer.timer.tick(time.delta());

        // if it finished, despawn the bomb
        if fuse_timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Resource)]
struct BombsSpawnConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

/// Spawn a new bomb in set intervals of time
fn spawn_bombs(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<BombsSpawnConfig>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        commands.spawn((
            FuseTime {
                // create the non-repeating fuse timer
                timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
            },
            // ... other components ...
        ));
    }
}

/// Configure our bomb spawning algorithm
fn setup_bomb_spawning(
    mut commands: Commands,
) {
    commands.insert_resource(BombsSpawnConfig {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
    })
}
// ANCHOR_END: timer

// ANCHOR: stopwatch
use bevy::time::Stopwatch;

#[derive(Component)]
struct JumpDuration {
    time: Stopwatch,
}

fn jump_duration(
    time: Res<Time>,
    mut q_player: Query<&mut JumpDuration, With<Player>>,
    kbd: Res<Input<KeyCode>>,
) {
    // assume we have exactly one player that jumps with Spacebar
    let mut jump = q_player.single_mut();

    if kbd.just_pressed(KeyCode::Space) {
        jump.time.reset();
    }

    if kbd.pressed(KeyCode::Space) {
        println!("Jumping for {} seconds.", jump.time.elapsed_secs());
        // stopwatch has to be ticked to progress
        jump.time.tick(time.delta());
    }
}
// ANCHOR_END: stopwatch

#[allow(dead_code)]
mod app1 {
    use bevy::prelude::*;
    use super::*;

// ANCHOR: appinit-resource
fn main() {
    App::new()
        // ...

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
    fn debug_levelups() {}
    fn player_level_up() {}

    struct LevelUpEvent;

// ANCHOR: app-builder
fn main() {
    App::new()
        // Bevy itself:
        .add_plugins(DefaultPlugins)

        // resources:
        .insert_resource(StartingLevel(3))
        // if it implements `Default` or `FromWorld`
        .init_resource::<MyFancyResource>()

        // events:
        .add_event::<LevelUpEvent>()

        // systems to run once at startup:
        .add_startup_system(spawn_things)

        // systems to run each frame:
        .add_system(player_level_up)
        .add_system(debug_levelups)
        .add_system(debug_stats_change)
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
    fn my_system() {}

// ANCHOR: plugins
struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyOtherResource>()
            .add_event::<MyEvent>()
            .add_startup_system(plugin_init)
            .add_system(my_system);
    }
}

fn main() {
    App::new()
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

    App::new()
        .add_plugins(DefaultPlugins)

        // add DEBUG stage after Bevy's Update
        // also make it single-threaded
        .add_stage_after(CoreStage::Update, DEBUG, SystemStage::single_threaded())

        // systems are added to the `CoreStage::Update` stage by default
        .add_system(player_gather_xp)
        .add_system(player_take_damage)

        // add our debug systems
        .add_system_to_stage(DEBUG, debug_player_hp)
        .add_system_to_stage(DEBUG, debug_stats_change)
        .add_system_to_stage(DEBUG, debug_new_hostiles)

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
    App::new()
        .add_plugins(DefaultPlugins)

        // add the app state type
        .add_state(AppState::MainMenu)

        // add systems to run regardless of state, as usual
        .add_system(play_music)

        // systems to run only in the main menu
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(handle_ui_buttons)
        )

        // setup when entering the state
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_menu)
        )

        // cleanup when exiting the state
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(close_menu)
        )
        .run();
}
// ANCHOR_END: app-states

    fn animate_trees() {}
    fn animate_water() {}
    fn player_movement() {}
    fn player_idle() {}
    fn reset_player() {}
    fn hide_enemies() {}
    fn setup_player() {}
    fn despawn_player() {}
    fn setup_map() {}
    fn despawn_map() {}
    fn handle_ui_buttons() {}
    fn setup_menu() {}
    fn close_menu() {}

fn main2() {
    App::new()
        .add_plugins(DefaultPlugins)
        // add the app state type
        .add_state(AppState::InGame)
// ANCHOR: state-stack
        // player movement only when actively playing
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_movement)
        )
        // player idle animation while paused
        .add_system_set(
            SystemSet::on_inactive_update(AppState::InGame)
                .with_system(player_idle)
        )
        // animations both while paused and while active
        .add_system_set(
            SystemSet::on_in_stack_update(AppState::InGame)
                .with_system(animate_trees)
                .with_system(animate_water)
        )
        // things to do when becoming inactive
        .add_system_set(
            SystemSet::on_pause(AppState::InGame)
                .with_system(hide_enemies)
        )
        // things to do when becoming active again
        .add_system_set(
            SystemSet::on_resume(AppState::InGame)
                .with_system(reset_player)
        )
        // setup when first entering the game
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup_player)
                .with_system(setup_map)
        )
        // cleanup when finally exiting the game
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(despawn_player)
                .with_system(despawn_map)
        )
// ANCHOR_END: state-stack
        .run();
}
}

#[allow(dead_code)]
mod app6 {
use bevy::prelude::*;

#[derive(Resource)]
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

// ANCHOR: system-pipe
fn main() {
    App::new()
        // ...
        .add_system(net_receive.pipe(handle_io_errors))
        // ...
        .run();
}
// ANCHOR_END: system-pipe
}

#[allow(dead_code)]
mod app7 {
use bevy::prelude::*;

    fn keyboard_input() {}
    fn gamepad_input() {}
    fn player_movement() {}
    fn debug_movement() {}

// ANCHOR: labels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum MySystems {
    InputSet,
    Movement,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(StageLabel)]
enum MyStages {
    Prepare,
    Cleanup,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(StageLabel)]
struct DebugStage;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // Add our game systems:
        .add_system_set(
            SystemSet::new()
                .label(MySystems::InputSet)
                .with_system(keyboard_input)
                .with_system(gamepad_input)
        )
        .add_system(player_movement.label(MySystems::Movement))

        // temporary debug system, let's just use a string label
        .add_system(debug_movement.label("temp-debug"))

        // Add our custom stages:
        // note that Bevy's `CoreStage` is an enum just like ours!
        .add_stage_before(CoreStage::Update, MyStages::Prepare, SystemStage::parallel())
        .add_stage_after(CoreStage::Update, MyStages::Cleanup, SystemStage::parallel())

        .add_stage_after(CoreStage::Update, DebugStage, SystemStage::parallel())

        // we can just use a string for this one:
        .add_stage_before(CoreStage::PostUpdate, "temp-debug-hack", SystemStage::parallel())

        .run();
}
// ANCHOR_END: labels
}

#[allow(dead_code)]
mod app8a {
use bevy::prelude::*;

    fn particle_effects() {}
    fn npc_behaviors() {}
    fn enemy_movement() {}
    fn player_movement() {}
    fn input_handling() {}

// ANCHOR: system-order
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // order doesn't matter for these systems:
        .add_system(particle_effects)
        .add_system(npc_behaviors)
        .add_system(enemy_movement)

        .add_system(input_handling)

        .add_system(
            player_movement
                // `player_movement` must always run before `enemy_movement`
                .before(enemy_movement)
                // `player_movement` must always run after `input_handling`
                .after(input_handling)
        )
        .run();
}
// ANCHOR_END: system-order
}

#[allow(dead_code)]
mod app8b {
use bevy::prelude::*;

    fn input_joystick() {}
    fn input_keyboard() {}
    fn input_touch() {}
    fn input_parameters() {}
    fn player_movement() {}

// ANCHOR: system-labels

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum MyLabel {
    /// everything that handles input
    Input,
    /// everything that updates player state
    Player,
    /// everything that moves things (works with transforms)
    Movement,
    /// systems that update the world map
    Map,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // use labels, because we want to have multiple affected systems
        .add_system(input_joystick.label(MyLabel::Input))
        .add_system(input_keyboard.label(MyLabel::Input))
        .add_system(input_touch.label(MyLabel::Input))

        .add_system(input_parameters.before(MyLabel::Input))

        .add_system(
            player_movement
                .before(MyLabel::Map)
                .after(MyLabel::Input)
                // we can have multiple labels on this system
                .label(MyLabel::Player)
                .label(MyLabel::Movement)
                // can also use loose strings as labels
                .label("player_movement")
        )

        // … and so on …

        .run();
}
// ANCHOR_END: system-labels
}

#[allow(dead_code)]
mod app9 {
use bevy::prelude::*;

    fn server_session() {}
    fn server_updates() {}
    fn keyboard_input() {}
    fn gamepad_input() {}
    fn session_ui() {}
    fn player_movement() {}
    fn smoke_particles() {}

// ANCHOR: systemset-labels
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // group our input handling systems into a set
        .add_system_set(
            SystemSet::new()
                .label("input")
                .with_system(keyboard_input)
                .with_system(gamepad_input)
        )

        // our "net" systems should run before "input"
        .add_system_set(
            SystemSet::new()
                .label("net")
                .before("input")
                // individual systems can still have
                // their own labels (and ordering)
                .with_system(server_session.label("session"))
                .with_system(server_updates.after("session"))
        )

        // some ungrouped systems
        .add_system(player_movement.after("input"))
        .add_system(session_ui.after("session"))
        .add_system(smoke_particles)

        .run();
}
// ANCHOR_END: systemset-labels
}

#[allow(dead_code)]
mod app10 {
use bevy::prelude::*;

    fn server_session() {}
    fn fetch_server_updates() {}
    fn keyboard_input() {}
    fn gamepad_input() {}
    fn host_debug() {}
    fn host_session() {}
    fn host_player_movement() {}
    fn host_enemy_ai() {}
    fn smoke_particles() {}
    fn water_animation() {}

    #[derive(Resource)]
    struct MyNetworkSession;

    impl MyNetworkSession {
        fn is_connected(&self) -> bool {
            true
        }
    }

// ANCHOR: run-criteria
use bevy::ecs::schedule::ShouldRun;

#[derive(Debug, PartialEq, Eq)]
#[derive(Resource)]
enum MultiplayerKind {
    Client,
    Host,
    Local,
}

fn run_if_connected(
    mode: Res<MultiplayerKind>,
    session: Res<MyNetworkSession>,
) -> ShouldRun
{
    if *mode == MultiplayerKind::Client && session.is_connected() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn run_if_host(
    mode: Res<MultiplayerKind>,
) -> ShouldRun
{
    if *mode == MultiplayerKind::Host || *mode == MultiplayerKind::Local {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // if we are currently connected to a server,
        // activate our client systems
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_connected)
                .before("input")
                .with_system(server_session)
                .with_system(fetch_server_updates)
        )

        // if we are hosting the game,
        // activate our game hosting systems
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_host)
                .before("input")
                .with_system(host_session)
                .with_system(host_player_movement)
                .with_system(host_enemy_ai)
        )

        // other systems in our game
        .add_system(smoke_particles)
        .add_system(water_animation)
        .add_system_set(
            SystemSet::new()
                .label("input")
                .with_system(keyboard_input)
                .with_system(gamepad_input)
        )
        .run();
}
// ANCHOR_END: run-criteria

mod sub {
    use super::*;

// ANCHOR: run-criteria-label
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(RunCriteriaLabel)]
enum MyRunCriteria {
    Client,
    Host,
}

fn main() {
    App::new()
        // ...
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(
                    // assign it a label
                    run_if_host
                        .label(MyRunCriteria::Host)
                )
                .with_system(host_session)
                .with_system(host_player_movement)
                .with_system(host_enemy_ai)
        )

        // extra system for debugging the host
        // it can share our previously-registered run criteria
        .add_system(host_debug
            .with_run_criteria(MyRunCriteria::Host)
        )
        .run();
}

// ANCHOR_END: run-criteria-label
}
}

#[allow(dead_code)]
mod app11 {
    use bevy::prelude::*;
    use super::*;
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
// ANCHOR: events-appbuilder
fn main() {
    App::new()
        // ...
        .add_event::<LevelUpEvent>()
        .add_system(player_level_up)
        .add_system(debug_levelups)
        // ...
        .run();
}
// ANCHOR_END: events-appbuilder
}

#[allow(dead_code)]
mod app12 {
    use super::*;
    use bevy::{app::PluginGroupBuilder, log::LogPlugin};

    struct FooPlugin;

    impl Plugin for FooPlugin {
        fn build(&self, app: &mut App) {}
    }

    struct BarPlugin;

    impl Plugin for BarPlugin {
        fn build(&self, app: &mut App) {}
    }

// ANCHOR: plugin-groups
struct MyPluginGroup;

impl PluginGroup for MyPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FooPlugin)
            .add(BarPlugin)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyPluginGroup)
        .run();
}
// ANCHOR_END: plugin-groups

    fn disable_plugins() {
// ANCHOR: plugin-groups-disable
App::new()
    .add_plugins(
        DefaultPlugins.build()
            .disable::<LogPlugin>()
    )
    .run();
// ANCHOR_END: plugin-groups-disable
    }
}

#[allow(dead_code)]
mod app13 {
use super::*;
// ANCHOR: fixed-timestep
use bevy::time::FixedTimestep;

// The timestep says how many times to run the SystemSet every second
// For TIMESTEP_1, it's once every second
// For TIMESTEP_2, it's twice every second

const TIMESTEP_1_PER_SECOND: f64 = 60.0 / 60.0;
const TIMESTEP_2_PER_SECOND: f64 = 30.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                .with_system(slow_timestep)
        )
        .add_system_set(
            SystemSet::new()
                // This prints out "goodbye world" twice every second
                .with_run_criteria(FixedTimestep::step(TIMESTEP_2_PER_SECOND))
                .with_system(fast_timestep)
        )
        .run();
}

fn slow_timestep() {
    println!("hello world");
}

fn fast_timestep() {
    println!("goodbye world");
}
// ANCHOR_END: fixed-timestep
}

#[allow(dead_code)]
mod app14 {
use super::*;
// ANCHOR: removal-detection
/// Some component type for the sake of this example.
#[derive(Component)]
struct Seen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // we could add our system to Bevy's `PreUpdate` stage
        // (alternatively, you could create your own stage)
        .add_system_to_stage(CoreStage::PreUpdate, remove_components)
        // our detection system runs in a later stage
        // (in this case: Bevy's default `Update` stage)
        .add_system(detect_removals)
        .run();
}

fn remove_components(
    mut commands: Commands,
    q: Query<(Entity, &Transform), With<Seen>>,
) {
    for (e, transform) in q.iter() {
        if transform.translation.y < -10.0 {
            // remove the `Seen` component from the entity
            commands.entity(e)
                .remove::<Seen>();
        }
    }
}

fn detect_removals(
    removals: RemovedComponents<Seen>,
    // ... (maybe Commands or a Query ?) ...
) {
    for entity in removals.iter() {
        // do something with the entity
    }
}
// ANCHOR_END: removal-detection
}

#[allow(dead_code)]
mod app15 {
use super::*;

struct OSAudioMagic {
    ptr: *mut u32,
}

impl OSAudioMagic {
    fn init() -> Self {
        Self {
            ptr: 0xDEADBEEF as *mut u32,
        }
    }
}

// ANCHOR: insert-nonsend
fn setup_platform_audio(world: &mut World) {
    // assuming `OSAudioMagic` is some primitive that is not thread-safe
    let instance = OSAudioMagic::init();

    world.insert_non_send_resource(instance);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_platform_audio)
        .run();
}
// ANCHOR_END: insert-nonsend
}

#[allow(dead_code)]
mod app16 {
use super::*;
// ANCHOR: nonsend
fn setup_raw_window(mut windows: NonSend<WinitWindows>) {
    let raw_window = windows.get_window(WindowId::primary()).unwrap();
    // do some special things
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // just add it as a normal system;
        // Bevy will notice the NonSend parameter
        // and ensure it runs on the main thread
        .add_startup_system(setup_raw_window)
        .run();
}
// ANCHOR_END: nonsend
}

#[allow(dead_code)]
mod app17 {
use super::*;

fn some_more_things(world: &mut World) {}

// ANCHOR: exclusive-fn
fn do_crazy_things(world: &mut World) {
    // we can do anything with any data in the Bevy ECS here!
}
// ANCHOR_END: exclusive-fn

// ANCHOR: exclusive-app
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // this will run at the start of CoreStage::Update (the default stage)
        .add_system(do_crazy_things)

        // this will run at the end of CoreStage::PostUpdate
        .add_system_to_stage(
            CoreStage::PostUpdate,
            some_more_things
                .at_end()
        )

        .run();
}
// ANCHOR_END: exclusive-app
}

#[allow(dead_code)]
mod app18 {
use super::*;
// ANCHOR: globaltransform
/// Print the up-to-date global coordinates of the player as of **this frame**.
fn debug_globaltransform(
    query: Query<&GlobalTransform, With<Player>>,
) {
    let gxf = query.single();
    debug!("Player at: {:?}", gxf.translation());
}

fn main() {
    // the label to use for ordering
    use bevy::transform::TransformSystem;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            debug_globaltransform
                .after(TransformSystem::TransformPropagate)
        )
        .run();
}
// ANCHOR_END: globaltransform
}

#[allow(dead_code)]
mod app19 {
use super::*;
// ANCHOR: computedvisibility
/// Check if the Player was hidden manually
fn debug_player_visibility(
    query: Query<&ComputedVisibility, With<Player>>,
) {
    let vis = query.single();

    // check if it was manually hidden via Visibility
    // (incl. by any parent entity)
    debug!("Player visible due to hierachy: {:?}", vis.is_visible_in_hierarchy());
}

/// Check if balloons are seen by any Camera, Light, etc… (not culled)
fn debug_balloon_visibility(
    query: Query<&ComputedVisibility, With<Balloon>>,
) {
    for vis in query.iter() {
        debug!("Balloon is in view: {:?}", vis.is_visible_in_view());

        // check overall final actual visibility
        // (combines visible-in-hierarchy and visible-in-view)
        debug!("Balloon is visible: {:?}", vis.is_visible());
    }
}

fn main() {
    // the labels to use for ordering
    use bevy::render::view::VisibilitySystems;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            debug_balloon_visibility
                // in-view visibility (culling) is done here:
                .after(VisibilitySystems::CheckVisibility)
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            debug_player_visibility
                // hierarchy propagation is done here:
                .after(VisibilitySystems::VisibilityPropagate)
        )
        .run();
}
// ANCHOR_END: computedvisibility
}

#[allow(dead_code)]
mod app20 {
use super::*;
// ANCHOR: asset-watch
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .run();
}
// ANCHOR_END: asset-watch
}

/// REGISTER ALL SYSTEMS TO DETECT COMPILATION ERRORS!
pub fn _main_all() {
    App::new()
        .add_startup_system(debug_start)
        .add_startup_system(load_ui_font)
        .add_startup_system(load_extra_assets)
        .add_startup_system(spawn_gltf)
        .add_startup_system(load_gltf)
        .add_startup_system(spawn_gltf_objects)
        .add_startup_system(use_gltf_things)
        .add_startup_system(gltf_manual_entity)
        .add_startup_system(setup_bomb_spawning)
        .add_startup_system(spawn_special_entity)
        .add_startup_system(spawn_toplevel_entity)
        .add_startup_system(setup_map_hidden)
        .add_system(detect_removed_res)
        .add_system(check_res_added)
        .add_system(check_res_changed)
        .add_system(commands_catchall)
        .add_system(query_entities)
        .add_system(query_player)
        .add_system(query_misc)
        .add_system(debug_player_hp)
        .add_system(debug_stats_change)
        .add_system(debug_new_hostiles)
        .add_system(debug_damage)
        .add_system(check_zero_health)
        .add_system(reset_health)
        .add_system(my_system1)
        .add_system(my_system2)
        .add_system(complex_system)
        .add_system(spawn_things)
        .add_system(close_menu)
        .add_system(make_all_players_hostile)
        .add_system(use_sprites)
        .add_system(fixup_images)
        .add_system(update_player_xp)
        .add_system(camera_with_parent)
        .add_system(process_squad_damage)
        .add_system(add_material)
        .add_system(load_gltf_things)
        .add_system(spawn_my_stuff)
        .add_system(explode_bombs)
        .add_system(spawn_bombs)
        .add_system(jump_duration)
        .add_system(asteroids_fly)
        .add_system(despawn_child)
        .add_system(inflate_balloons)
        .add_system(reveal_map)
        .add_system(insert_visibilitybundle)
        .run();
}
