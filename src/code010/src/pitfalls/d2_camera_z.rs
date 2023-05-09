use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
// ANCHOR: spawn
// ✗ INCORRECT: we are creating a new transform with the given rotation
// it does not have the correct Z,
// because it overrides the Transform from the bundle default
commands.spawn(Camera2dBundle {
    transform: Transform::from_rotation(Quat::from_rotation_z(30.0f32.to_radians())),
    ..Default::default()
});

// ✓ OKAY: we can set the XYZ position ourselves.
// Z=999.9 is what Bevy's default uses.
commands.spawn(Camera2dBundle {
    transform: Transform::from_xyz(0.0, 0.0, 999.9)
        .with_rotation(Quat::from_rotation_z(30.0f32.to_radians())),
    ..Default::default()
});

// ✓ OKAY: we can create the bundle in a `mut` variable,
// and then modify only what we care about
// This way, Bevy's defaults remain intact
let mut my_camera = Camera2dBundle::default();
my_camera.transform.rotation = Quat::from_rotation_z(30.0f32.to_radians());
commands.spawn(my_camera);
// ANCHOR_END: spawn
}

fn _main() {
    App::new().add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera);
}
