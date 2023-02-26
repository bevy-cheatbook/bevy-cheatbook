use bevy::prelude::*;

// ANCHOR: simple
#[derive(Component)]
struct MyGameCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        MyGameCamera,
    ));
}
// ANCHOR_END: simple

#[derive(Component)]
struct MyOverlayCamera;

#[derive(Component)]
struct MyExtraCamera;

#[derive(Component)]
struct MyMinimapCamera;

fn setup_overlay(mut commands: Commands) {
// ANCHOR: overlay
use bevy::core_pipeline::clear_color::ClearColorConfig;

commands.spawn((
    Camera2dBundle {
        camera_2d: Camera2d {
            // no "background color", we need to see the main camera's output
            clear_color: ClearColorConfig::None,
            ..default()
        },
        camera: Camera {
            // renders after / on top of the main camera
            priority: 1,
            ..default()
        },
        ..default()
    },
    MyOverlayCamera,
));
// ANCHOR_END: overlay
}

fn setup_no_ui(mut commands: Commands) {
// ANCHOR: no-ui
commands.spawn((
    Camera3dBundle::default(),
    // UI config is a separate component
    UiCameraConfig {
        show_ui: false,
    },
    MyExtraCamera,
));
// ANCHOR_END: no-ui
}

fn setup_renderlayers(mut commands: Commands) {
// ANCHOR: renderlayers
use bevy::render::view::visibility::RenderLayers;
// This camera renders everything in layers 0, 1
commands.spawn((
    Camera2dBundle::default(),
    RenderLayers::from_layers(&[0, 1])
));
// This camera renders everything in layers 1, 2
commands.spawn((
    Camera2dBundle::default(),
    RenderLayers::from_layers(&[1, 2])
));
// This sprite will only be seen by the first camera
commands.spawn((
    SpriteBundle::default(),
    RenderLayers::layer(0),
));
// This sprite will be seen by both cameras
commands.spawn((
    SpriteBundle::default(),
    RenderLayers::layer(1),
));
// This sprite will only be seen by the second camera
commands.spawn((
    SpriteBundle::default(),
    RenderLayers::layer(2),
));
// This sprite will also be seen by both cameras
commands.spawn((
    SpriteBundle::default(),
    RenderLayers::from_layers(&[0, 2]),
));
// ANCHOR_END: renderlayers
}

// ANCHOR: is_active
fn toggle_overlay(
    mut q: Query<&mut Camera, With<MyOverlayCamera>>,
) {
    let mut camera = q.single_mut();
    camera.is_active = !camera.is_active;
}
// ANCHOR_END: is_active

// ANCHOR: render-target
use bevy::render::camera::RenderTarget;

fn debug_render_targets(
    q: Query<&Camera>,
) {
    for camera in &q {
        match &camera.target {
            RenderTarget::Window(wid) => {
                eprintln!("Camera renders to window with id: {:?}", wid);
            }
            RenderTarget::Image(handle) => {
                eprintln!("Camera renders to image asset with id: {:?}", handle);
            }
        }
    }
}
// ANCHOR_END: render-target

// ANCHOR: set-viewport
use bevy::render::camera::Viewport;

fn setup_minimap(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after / on top of other cameras
                priority: 2,
                // set the viewport to a 256x256 square in the top left corner
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(256, 256),
                    ..default()
                }),
                ..default()
            },
            ..default()
        },
        MyMinimapCamera,
    ));
}
// ANCHOR_END: set-viewport
// ANCHOR: get-viewport
fn debug_viewports(
    q: Query<&Camera, With<MyExtraCamera>>,
) {
    let camera = q.single();

    // the size of the area being rendered to
    let view_dimensions = camera.logical_viewport_size().unwrap();

    // the top-left and bottom-right coordinates
    let (corner1, corner2) = camera.logical_viewport_rect().unwrap();
}
// ANCHOR_END: get-viewport

fn main() {
    App::new()
        .add_system(toggle_overlay)
        .add_system(setup_overlay)
        .add_system(debug_render_targets)
        .add_system(debug_viewports)
        .add_system(setup)
        .add_system(setup_renderlayers)
        .add_system(setup_minimap)
        .add_system(setup_no_ui);
}
