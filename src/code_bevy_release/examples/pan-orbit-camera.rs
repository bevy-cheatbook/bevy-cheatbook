use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel,MouseMotion};

// ANCHOR: example
/// Component for panning and orbiting
#[derive(Default)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
}

/// Pan the camera with LHold or scrollwheel, orbit with rclick.
fn pan_orbit_camera(
    time: Res<Time>,
    windows: Res<Windows>,
    mousebtn: Res<Input<MouseButton>>,
    ev_motion: Res<Events<MouseMotion>>,
    ev_scroll: Res<Events<MouseWheel>>,
    mut reader_motion: Local<EventReader<MouseMotion>>,
    mut reader_scroll: Local<EventReader<MouseWheel>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    let mut translation = Vec2::zero();
    let mut rotation_move = Vec2::default();
    let mut scroll = 0.0;
    let dt = time.delta_seconds();

    if mousebtn.pressed(MouseButton::Right) {
        for ev in reader_motion.iter(&ev_motion) {
            rotation_move += ev.delta;
        }
    } else if mousebtn.pressed(MouseButton::Left) {
        // Pan only if we're not rotating at the moment
        for ev in reader_motion.iter(&ev_motion) {
            translation += ev.delta;
        }
    }

    for ev in reader_scroll.iter(&ev_scroll) {
        scroll += ev.y;
    }

    // Either pan+scroll or arcball. We don't do both at once.
    for (mut camera, mut trans) in query.iter_mut() {
        if rotation_move.length_squared() > 0.0 {
            let window = windows.get_primary().unwrap();
            let window_w = window.width() as f32;
            let window_h = window.height() as f32;

            // Link virtual sphere rotation relative to window to make it feel nicer
            let delta_x = rotation_move.x / window_w * std::f32::consts::PI * 2.0;
            let delta_y = rotation_move.y / window_h * std::f32::consts::PI;

            let delta_yaw = Quat::from_rotation_y(delta_x);
            let delta_pitch = Quat::from_rotation_x(delta_y);

            trans.translation =
                delta_yaw * delta_pitch * (trans.translation - camera.focus) + camera.focus;

            let look = Mat4::face_toward(
                trans.translation,
                camera.focus,
                Vec3::new(0.0, 1.0, 0.0)
            );
            trans.rotation = look.to_scale_rotation_translation().1;
        } else {
            // The plane is x/y while z is "up". Multiplying by dt gives constant rate
            let mut translation = Vec3::new(-translation.x * dt, translation.y * dt, 0.0);
            camera.focus += translation;
            translation.z = -scroll;
            trans.translation += translation;
        }
    }
}

/// Spawn a camera like this.
fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle::default())
        .with(PanOrbitCamera::default());
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
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        }).with(PanOrbitCamera::default());
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
