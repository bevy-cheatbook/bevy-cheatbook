use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
// ANCHOR: perspective-far
commands.spawn(Camera3dBundle {
    projection: Projection::Perspective(PerspectiveProjection {
        far: 10000.0, // change the maximum render distance
        ..default()
    }),
    ..default()
});
// ANCHOR_END: perspective-far
let parent = commands.spawn(()).id();
// ANCHOR: insert-visibilitybundle
commands.entity(parent)
    .insert(VisibilityBundle::default());
// ANCHOR_END: insert-visibilitybundle
// ANCHOR: gltf-ass-label
let handle_scene: Handle<Scene> = asset_server.load("my.gltf#Scene0");
// ANCHOR_END: gltf-ass-label
}
