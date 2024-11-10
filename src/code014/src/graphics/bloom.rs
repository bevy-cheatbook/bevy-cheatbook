use bevy::prelude::*;

fn setup(mut commands: Commands) {
// ANCHOR: bloom-config
use bevy::core_pipeline::bloom::BloomSettings;

commands.spawn((
    Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    },
    BloomSettings::NATURAL,
));
// ANCHOR_END: bloom-config
}

fn main() {
    App::new()
        .add_systems(Startup, setup);
}
