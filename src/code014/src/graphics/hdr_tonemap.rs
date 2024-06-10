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
        deband_dither: DebandDither::Disabled,
        ..default()
    },
));
// ANCHOR_END: deband-dither
// ANCHOR: color-grading
use bevy::render::view::ColorGrading;

let mut camera = Camera3dBundle::default();

// Global color parameters (applying to the whole image):

// Camera Exposure offset (stops)
camera.color_grading.global.exposure = -0.5;
// White balance: colder/bluer (negative), warmer/redder (positive)
camera.color_grading.global.temperature = -0.2;
// Color tint: greener (negative), magenta (positive)
camera.color_grading.global.tint = 0.125;
// Color saturation: gray (0.0), desaturated (0.0..1.0), boosted (>1.0)
camera.color_grading.global.post_saturation = 0.75;
// Color hue adjustment, rotates the color wheel. Value in *radians*.
camera.color_grading.global.hue = 5.0f32.to_radians(); // 5 degrees

// We can also adjust some parameters separately
// for shadows/midtones/highlights.

// You can customize the thresholds / range.
// The transition around the threshold is gradual, not abrupt.
// Default is 0.2 .. 0.7.
camera.color_grading.global.midtones_range = 0.25 .. 0.75;

// Contrast: reduce (0.0..1.0), enhance (>1.0)
camera.color_grading.shadows.contrast = 0.9;
camera.color_grading.midtones.contrast = 1.1;

// Color saturation: gray (0.0), desaturated (0.0..1.0), boosted (>1.0)
camera.color_grading.shadows.saturation = 0.0;
camera.color_grading.highlights.saturation = 1.2;

// Adjustments using the Standard ASC CDL color correction formula:
// `output = (input * gain + lift) ^ gamma`; Default:
// `output = (input *  1.0 +  0.0) ^ 1.0`
camera.color_grading.shadows.gain = 1.1;
camera.color_grading.midtones.lift = 0.1;
camera.color_grading.highlights.gamma = 0.9;

commands.spawn(camera);
// ANCHOR_END: color-grading
}

fn main() {
    App::new()
        .add_systems(Startup, setup);
}
