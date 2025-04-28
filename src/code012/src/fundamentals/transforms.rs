use bevy::prelude::*;

#[derive(Component)]
struct MySpecialMarker;
#[derive(Component)]
struct MyGameCamera;
#[derive(Component)]
struct Player;
#[derive(Component)]
struct ThrowableProjectile;
#[derive(Component)]
struct Balloon;
#[derive(Component)]
struct ComponentA;
#[derive(Component)]
struct ComponentB;

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
let xf_rot3d = Transform::from_rotation(Quat::from_euler(
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

// ANCHOR: transform-mut
fn inflate_balloons(
    mut query: Query<&mut Transform, With<Balloon>>,
    keyboard: Res<Input<KeyCode>>,
) {
    // every time the Spacebar is pressed,
    // make all the balloons in the game bigger by 25%
    if keyboard.just_pressed(KeyCode::Space) {
        for mut transform in &mut query {
            transform.scale *= 1.25;
        }
    }
}

fn throwable_fly(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<ThrowableProjectile>>,
) {
    // every frame, make our projectiles fly across the screen and spin
    for mut transform in &mut query {
        // do not forget to multiply by the time delta!
        // this is required to move at the same speed regardless of frame rate!
        transform.translation.x += 100.0 * time.delta_seconds();
        transform.rotate_z(2.0 * time.delta_seconds());
    }
}
// ANCHOR_END: transform-mut

// ANCHOR: globaltransform
/// Print the up-to-date global coordinates of the player
fn debug_globaltransform(
    query: Query<&GlobalTransform, With<Player>>,
) {
    let gxf = query.single();
    debug!("Player at: {:?}", gxf.translation());
}
// ANCHOR_END: globaltransform

fn main() {
let mut app = App::new();
// ANCHOR: globaltransform-app
// the label to use for ordering
use bevy::transform::TransformSystem;

app.add_systems(PostUpdate,
    debug_globaltransform
        // we want to read the GlobalTransform after
        // it has been updated by Bevy for this frame
        .after(TransformSystem::TransformPropagate)
);
// ANCHOR_END: globaltransform-app
}

// ANCHOR: transformhelper
fn camera_look_follow(
    q_target: Query<Entity, With<MySpecialMarker>>,
    mut transform_params: ParamSet<(
        TransformHelper,
        Query<&mut Transform, With<MyGameCamera>>,
    )>,
) {
    // get the Entity ID we want to target
    let e_target = q_target.single();
    // compute its actual current GlobalTransform
    // (could be Err if entity doesn't have transforms)
    let Ok(global) = transform_params.p0().compute_global_transform(e_target) else {
        return;
    };
    // get camera transform and make it look at the global translation
    transform_params.p1().single_mut().look_at(global.translation(), Vec3::Y);
}
// ANCHOR_END: transformhelper
