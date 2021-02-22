use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel,MouseMotion};
use bevy::render::camera::PerspectiveProjection;

// ANCHOR: example
/// Tags an entity as capable of panning and orbiting.
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::zero(),
            radius: 5.0,
            upside_down: false,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    windows: Res<Windows>,
    mut reader_motion: Local<EventReader<MouseMotion>>,
    mut reader_scroll: Local<EventReader<MouseWheel>>,
    ev_motion: Res<Events<MouseMotion>>,
    ev_mouse: Res<Input<MouseButton>>,
    ev_scroll: Res<Events<MouseWheel>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::zero();
    let mut rotation_move = Vec2::zero();
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if ev_mouse.pressed(orbit_button) {
        for ev in reader_motion.iter(&ev_motion) {
            rotation_move += ev.delta;
        }
    } else if ev_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in reader_motion.iter(&ev_motion) {
            pan += ev.delta;
        }
    }
    for ev in reader_scroll.iter(&ev_scroll) {
        scroll += ev.y;
    }
    if ev_mouse.just_released(orbit_button) || ev_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::unit_y();
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            // translate by local axes
            let right = transform.rotation * Vec3::unit_x() * -pan.x;
            let up = transform.rotation * Vec3::unit_y() * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

/// Spawn a camera like this
fn spawn_camera(commands: &mut Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(translation)
            .looking_at(Vec3::zero(), Vec3::unit_y()),
        ..Default::default()
    })
        .with(PanOrbitCamera {
            radius,
            ..Default::default()
        });
}
// ANCHOR_END: example

fn spawn_scene(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn a cube and a light
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

    spawn_camera(commands);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_scene.system())
        .add_system(pan_orbit_camera.system())
        .run();

    // just to catch compilation errors
    let _ = App::build()
        .add_startup_system(spawn_camera.system());
}
