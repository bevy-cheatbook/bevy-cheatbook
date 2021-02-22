use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel,MouseMotion};

// ANCHOR: example
/// Tags an entity as capable of panning and orbiting.
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub zoom: f32,
    pub rotation: Quat,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::zero(),
            zoom: 5.0,
            rotation: Quat::identity(),
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
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
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

    for (mut pan_orbit, mut transform) in query.iter_mut() {
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
            pan_orbit.rotation = yaw * pan_orbit.rotation; // rotate around global y axis
            pan_orbit.rotation = pan_orbit.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution,
            let window = get_primary_window_size(&windows);
            let ref_height = 0.829;
            let reference = Vec2::new(ref_height * window.x / window.y, ref_height);
            let norm = reference / window;
            pan *= norm;
            // translate by local axes
            let right = transform.rotation * Vec3::unit_x() * -pan.x;
            let up = transform.rotation * Vec3::unit_y() * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.zoom;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.zoom -= scroll * pan_orbit.zoom * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.zoom = f32::max(pan_orbit.zoom, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let parent = Mat4::from_rotation_translation(pan_orbit.rotation, pan_orbit.focus);
            let child = Mat4::from_rotation_translation(Quat::identity(), Vec3::new(0.0, 0.0, pan_orbit.zoom));
            let (_, rot, pos) = parent.mul_mat4(&child).to_scale_rotation_translation();
            transform.translation = pos;
            transform.rotation = rot;
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
    use bevy::render::camera::PerspectiveProjection;

    let camera = PanOrbitCamera::default();
    let zoom = camera.zoom;

    commands.spawn((camera, ))
        .with_bundle(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, zoom))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            perspective_projection: PerspectiveProjection {
                near: 0.01, // no clue why the near clipping plane is set to 1.0 by default
                ..Default::default()
            },
            ..Default::default()
        });
}
// ANCHOR_END: example

fn spawn_scene(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use bevy::render::camera::PerspectiveProjection;

    // spawn a cube and a light
    let transform = Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
        .looking_at(Vec3::default(), Vec3::unit_y());
    let rotation = transform.rotation;
    let zoom = transform.translation.length();
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
        })
        .spawn(Camera3dBundle {
            transform,
            perspective_projection: PerspectiveProjection {
                near: 0.01, // no clue why the near clipping plane is set to 1.0 by default
                ..Default::default()
            },
            ..Default::default()
        }).with(PanOrbitCamera {
            rotation,
            zoom,
            ..Default::default()
        });
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
