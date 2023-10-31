use bevy::prelude::*;

#[derive(Resource)]
struct ResourceA;
#[derive(Resource)]
struct ResourceB;
#[derive(Resource)]
struct ResourceC;
#[derive(Resource, Debug)]
struct StartingLevel(u32);

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

fn setup_camera() {}
fn move_player() {}
fn enemies_ai() {}

// ANCHOR: systems-appbuilder
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // run these only once at launch
        .add_systems(Startup, (setup_camera, debug_start))
        // run these every frame update
        .add_systems(Update, (move_player, enemies_ai))
        // ...
        .run();
}
// ANCHOR_END: systems-appbuilder

