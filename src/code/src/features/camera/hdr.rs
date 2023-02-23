use bevy::prelude::*;

fn setup(mut commands: Commands) {
// ANCHOR: hdr-config
commands.spawn((
    Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    },
));
// ANCHOR_END: hdr-config
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
    BloomSettings {
        intensity: 0.25, // the default is 0.3
        ..default()
    },
));
// ANCHOR_END: bloom-config
// ANCHOR: tonemap-config
use bevy::core_pipeline::tonemapping::Tonemapping;

commands.spawn((
    Camera3dBundle {
        tonemapping: Tonemapping::Disabled,
        ..default()
    },
));
commands.spawn((
    Camera3dBundle {
        // this is the default:
        tonemapping: Tonemapping::Enabled {
            deband_dither: true, // dithering
        },
        ..default()
    },
));
// ANCHOR_END: tonemap-config
}

fn main() {
    App::new()
        .add_startup_system(setup);
}
