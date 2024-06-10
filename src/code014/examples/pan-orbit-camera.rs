use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use winit::keyboard::KeyCode;

// ANCHOR: types
// Bundle to spawn our custom camera easily
#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera3dBundle,
    pub state: PanOrbitState,
    pub settings: PanOrbitSettings,
}

// The internal state of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitState {
    center: Vec3,
    radius: f32,
    upside_down: bool,
    pitch: f32,
    yaw: f32,
}

/// The configuration of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitSettings {
    pub pan_sensitivity: f32,
    pub orbit_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub pan_key: Option<KeyCode>,
    pub orbit_key: Option<KeyCode>,
    pub zoom_key: Option<KeyCode>,
    pub scroll_action: Option<PanOrbitAction>,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanOrbitAction {
    Pan,
    Orbit,
    Zoom,
}
// ANCHOR_END: types

// ANCHOR: defaults
impl Default for PanOrbitState {
    fn default() -> Self {
        PanOrbitState {
            center: Vec3::ZERO,
            radius: 10.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        PanOrbitSettings {
            pan_sensitivity: 1.0,
            orbit_sensitivity: 1.0,
            zoom_sensitivity: 1.0,
            pan_key: Some(KeyCode::ControlLeft),
            orbit_key: Some(KeyCode::AltLeft),
            zoom_key: Some(KeyCode::ShiftLeft),
            scroll_action: Some(PanOrbitAction::Zoom),
            scroll_line_sensitivity: 8.0,
            scroll_pixel_sensitivity: 1.0,
        }
    }
}
// ANCHOR_END: defaults

// ANCHOR: setup
fn spawn_camera(mut commands: Commands) {
    commands.spawn(PanOrbitCameraBundle::default());
}
// ANCHOR_END: setup

// ANCHOR: impl
fn pan_orbit_camera(
    kbd: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q_camera: Query<(
        &PanOrbitSettings,
        &mut PanOrbitState,
        &mut Transform,
    )>,
) {
    // First, accumulate the total amount of
    // mouse motion and scroll, from all pending events:
    let total_motion: Vec2 = evr_motion.read()
        .map(|ev| ev.delta).sum();

    let mut total_scroll_lines = Vec2::ZERO;
    let mut total_scroll_pixels = Vec2::ZERO;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                total_scroll_lines.x += ev.x;
                total_scroll_lines.y += ev.y;
            }
            MouseScrollUnit::Pixel => {
                total_scroll_pixels.x += ev.x;
                total_scroll_pixels.y += ev.y;
            }
        }
    }
    let any_scroll =
        total_scroll_lines != Vec2::ZERO || total_scroll_pixels != Vec2::ZERO;

    // if nothing happened, we don't need to go any further
    if !any_scroll && total_motion == Vec2::ZERO {
        return;
    }

    for (settings, mut state, mut transform) in &mut q_camera {
        // Check how much of each thing we need to apply.
        // Accumulate values from motion and scroll,
        // based on our configuration settings.

        let mut total_pan = Vec2::ZERO;
        if settings.pan_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_pan += total_motion * settings.pan_sensitivity;
        }
        if any_scroll && settings.scroll_action == Some(PanOrbitAction::Pan) {
            total_pan += total_scroll_lines
                * settings.scroll_line_sensitivity * settings.pan_sensitivity;
            total_pan += total_scroll_pixels
                * settings.scroll_pixel_sensitivity * settings.pan_sensitivity;
        }

        let mut total_orbit = Vec2::ZERO;
        if settings.orbit_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_orbit += total_motion * settings.orbit_sensitivity;
        }
        if any_scroll && settings.scroll_action == Some(PanOrbitAction::Orbit) {
            total_orbit += total_scroll_lines
                * settings.scroll_line_sensitivity * settings.orbit_sensitivity;
            total_orbit += total_scroll_pixels
                * settings.scroll_pixel_sensitivity * settings.orbit_sensitivity;
        }

        let mut total_zoom = Vec2::ZERO;
        if settings.zoom_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_zoom += total_motion * settings.zoom_sensitivity;
        }
        if any_scroll && settings.scroll_action == Some(PanOrbitAction::Zoom) {
            total_zoom += total_scroll_lines
                * settings.scroll_line_sensitivity * settings.zoom_sensitivity;
            total_zoom += total_scroll_pixels
                * settings.scroll_pixel_sensitivity * settings.zoom_sensitivity;
        }

        // Now we can actually do the things!

        let mut any = false;

        // To PAN, we can get the UP and RIGHT direction
        // vectors from the camera's transform, and use
        // them to move the center point.
        if total_pan != Vec2::ZERO {
            any = true;
            state.center += transform.right() * total_pan.x;
            state.center += transform.up() * total_pan.y;
        }

        // To ORBIT, we change our pitch and yaw values
        if total_orbit != Vec2::ZERO {
            any = true;
            state.yaw += total_pan.x;
            state.pitch += total_pan.y;
        }

        // To ZOOM, we need to multiply our radius.
        if total_zoom != Vec2::ZERO {
            any = true;
            // in order for zoom to feel intuitive,
            // everything needs to be exponential
            // (done via multiplication)
            // not linear
            // (done via addition)

            // so we compute the exponential of our
            // accumulated value and multiply by that
            state.radius *= total_zoom.y.exp();
        }

        // Finally, compute the new camera transform.
        // (if we changed anything, or if the pan-orbit
        // controller was just added and thus we are running
        // for the first time and need to initialize)
        if any || state.is_added() {
            // YXZ Euler Rotation performs yaw/pitch/roll.
            transform.rotation =
                Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);
            // To position the camera, get the backward direction vector
            // and place the camera at the desired radius from the center.
            transform.translation = state.center + transform.back() * state.radius;
        }
    }
}
// ANCHOR_END: impl

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn a cube and a light
    commands.spawn(PbrBundle {
        mesh: meshes.add(bevy::math::primitives::Cuboid::new(1.0, 2.0, 3.0).mesh()),
        material: materials.add(StandardMaterial::from(Color::srgb(0.8, 0.7, 0.6))),
        transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, spawn_scene);
// ANCHOR: setup-app
app.add_systems(Startup, spawn_camera);
// ANCHOR_END: setup-app
// ANCHOR: impl-app-rc
app.add_systems(Update,
    pan_orbit_camera
        .run_if(any_with_component::<PanOrbitState>),
);
// ANCHOR_END: impl-app-rc
    app.run();
}
