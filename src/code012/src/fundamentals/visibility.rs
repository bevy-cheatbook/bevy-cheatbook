use bevy::prelude::*;

#[derive(Component)]
struct GameMapEntity;
#[derive(Component)]
struct ComponentA;
#[derive(Component)]
struct ComponentB;
#[derive(Component)]
struct MyAcceptButton;
#[derive(Component)]
struct Balloon;

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
            visibility: Visibility::Hidden,
            ..Default::default()
        },
    ));
}
// ANCHOR_END: spatialbundle

// ANCHOR: visibility
/// Prepare the game map, but do not display it until later
fn setup_map_hidden(
    mut commands: Commands,
) {
    commands.spawn((
        GameMapEntity,
        SceneBundle {
            scene: todo!(),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
    ));
}

/// When everything is ready, un-hide the game map
fn reveal_map(
    mut query: Query<&mut Visibility, With<GameMapEntity>>,
) {
    let mut vis_map = query.single_mut();
    *vis_map = Visibility::Visible;
}
// ANCHOR_END: visibility

// ANCHOR: inheritedvisibility
/// Check if a specific UI button is visible
/// (could be hidden if the whole menu is hidden?)
fn debug_player_visibility(
    query: Query<&InheritedVisibility, With<MyAcceptButton>>,
) {
    let vis = query.single();

    debug!("Button visibility: {:?}", vis.get());
}
// ANCHOR_END: inheritedvisibility

// ANCHOR: viewvisibility
/// Check if balloons are seen by any Camera, Light, etc… (not culled)
fn debug_balloon_visibility(
    query: Query<&ViewVisibility, With<Balloon>>,
) {
    for vis in query.iter() {
        if vis.get() {
            debug!("Balloon will be rendered.");
        }
    }
}
// ANCHOR_END: viewvisibility

fn main() {
let mut app = App::new();

{
// ANCHOR: inheritedvisibility-app
use bevy::render::view::VisibilitySystems;

app.add_systems(PostUpdate,
    debug_balloon_visibility
        .after(VisibilitySystems::CheckVisibility)
);
// ANCHOR_END: inheritedvisibility-app
}
{
// ANCHOR: viewvisibility-app
use bevy::render::view::VisibilitySystems;

app.add_systems(PostUpdate,
    debug_player_visibility
        .after(VisibilitySystems::VisibilityPropagate)
);
// ANCHOR_END: viewvisibility-app
}
}
// ANCHOR_END: computedvisibility
