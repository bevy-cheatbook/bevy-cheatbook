use bevy::prelude::*;

// ANCHOR: types
#[derive(Component)]
struct MyMovementState {
    position: Vec3,
    velocity: Vec3,
}

#[derive(Component)]
struct OldMovementState {
    position: Vec3,
}
// ANCHOR_END: types

// ANCHOR: controller
fn my_movement(
    time: Res<Time>,
    mut q_movement: Query<(&mut MyMovementState, &mut OldMovementState)>,
) {
    for (mut state, mut old_state) in &mut q_movement {
        // Reborrow `state` to mutably access both of its fields
        // See Cheatbook page on "Split Borrows"
        let state = &mut *state;

        // Store the old position.
        old_state.position = state.position;

        // Compute the new position.
        // (`delta_seconds` always returns the fixed timestep
        // duration, if this system is added to `FixedUpdate`)
        state.position += state.velocity * time.delta_seconds();
    }
}
// ANCHOR_END: controller

// ANCHOR: interpolation
fn transform_movement_interpolate(
    fixed_time: Res<Time<Fixed>>,
    mut q_movement: Query<(
        &mut Transform, &MyMovementState, &OldMovementState
    )>,
) {
    for (mut xf, state, old_state) in &mut q_movement {
        let a = fixed_time.overstep_fraction();
        xf.translation = old_state.position.lerp(state.position, a);
    }
}
// ANCHOR_END: interpolation

// ANCHOR: extrapolation-velocity
fn transform_movement_extrapolate_velocity(
    fixed_time: Res<Time<Fixed>>,
    mut q_movement: Query<(
        &mut Transform, &MyMovementState,
    )>,
) {
    for (mut xf, state) in &mut q_movement {
        let a = fixed_time.overstep_fraction();
        let future_position = state.position
            + state.velocity * fixed_time.delta_seconds();
        xf.translation = state.position.lerp(future_position, a);
    }
}
// ANCHOR_END: extrapolation-velocity

// ANCHOR: extrapolation-from-old
fn transform_movement_extrapolate_from_old(
    fixed_time: Res<Time<Fixed>>,
    mut q_movement: Query<(
        &mut Transform, &MyMovementState, &OldMovementState
    )>,
) {
    for (mut xf, state, old_state) in &mut q_movement {
        let a = fixed_time.overstep_fraction();
        let delta = state.position - old_state.position;
        let future_position = state.position + delta;
        xf.translation = state.position.lerp(future_position, a);
    }
}
// ANCHOR_END: extrapolation-from-old

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
// ANCHOR: controller-app
app.add_systems(FixedUpdate, my_movement);
// ANCHOR_END: controller-app
// ANCHOR: interpolation
app.add_systems(Update, transform_movement_interpolate);
// ANCHOR_END: interpolation
// ANCHOR: extrapolation-velocity
app.add_systems(Update, transform_movement_extrapolate_velocity);
// ANCHOR_END: extrapolation-velocity
// ANCHOR: extrapolation-from-old
app.add_systems(Update, transform_movement_extrapolate_from_old);
// ANCHOR_END: extrapolation-from-old
}
