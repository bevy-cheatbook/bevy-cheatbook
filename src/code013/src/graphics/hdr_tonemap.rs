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
        // no tonemapping
        tonemapping: Tonemapping::None,
        ..default()
    },
));
commands.spawn((
    Camera3dBundle {
        // this is the default:
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    },
));
commands.spawn((
    Camera3dBundle {
        // another common choice:
        tonemapping: Tonemapping::ReinhardLuminance,
        ..default()
    },
));
// ANCHOR_END: tonemap-config
// ANCHOR: deband-dither
use bevy::core_pipeline::tonemapping::DebandDither;

commands.spawn((
    Camera3dBundle {
        dither: DebandDither::Disabled,
        ..default()
    },
));
// ANCHOR_END: deband-dither
// ANCHOR: color-grading
use bevy::render::view::ColorGrading;

commands.spawn((
    Camera3dBundle {
        color_grading: ColorGrading {
            exposure: 0.0,
            gamma: 1.0,
            pre_saturation: 1.0,
            post_saturation: 1.0,
        },
        ..default()
    },
));
// ANCHOR_END: color-grading
}

fn main() {
    App::new()
        .add_systems(Startup, setup);
}
