use bevy::prelude::*;

#[derive(Component)]
struct EnemyIsTrackingPlayer;
#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct MyResource;

#[derive(Component)]
struct PlayerXp(f32);

#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum MyState {
    State,
}

fn cleanup_stuff() {}
fn maybe_remove_stuff() {}

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

// ANCHOR: ref
/// Make sprites flash red on frames when the Health changes
fn debug_damage(
    mut query: Query<(&mut Sprite, Ref<Health>)>,
) {
    for (mut sprite, health) in query.iter_mut() {
        // detect if the Health changed this frame
        if health.is_changed() {
            eprintln!("HP is: {}", health.hp);
            // we can also check if the sprite has been changed
            if !sprite.is_changed() {
                sprite.color = Color::RED;
            }
        }
    }
}
// ANCHOR_END: ref

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

// ANCHOR: removal-detection
fn detect_removals(
    mut removals: RemovedComponents<EnemyIsTrackingPlayer>,
    // ... (maybe Commands or a Query ?) ...
) {
    for entity in removals.read() {
        // do something with the entity
        eprintln!("Entity {:?} had the component removed.", entity);
    }
}
// ANCHOR_END: removal-detection

fn _main() {
let mut app = App::new();
app.add_systems(Update, (
    debug_stats_change,
    detect_removed_res,
    debug_new_hostiles,
    debug_damage,
    update_player_xp,
    check_res_changed,
    check_res_added,
));
// ANCHOR: removal-detection-app
// StateTransition and FixedUpdate schedules run before Update,
// so our removal detection system in Update will work no problem
app.add_systems(OnExit(MyState::State), cleanup_stuff);
app.add_systems(FixedUpdate, maybe_remove_stuff);

app.add_systems(Update, (
    // in the same schedule, we must have ordering dependencies
    (
        maybe_remove_stuff,
        detect_removals,
    ).chain(),
));
// ANCHOR_END: removal-detection-app
}
