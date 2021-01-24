# Pan + Orbit Camera

Provide an intuitive camera that pans with left click or scrollwheel, and orbits with right click.

```rust,no_run,noplayground
/// Tags an entity as capable of panning and orbiting.
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::zero(),
        }
    }
}

/// Hold readers for events
#[derive(Default)]
struct InputState {
    pub reader_motion: EventReader<MouseMotion>,
    pub reader_scroll: EventReader<MouseWheel>,
}

/// Pan the camera with LHold or scrollwheel, orbit with rclick.
fn pan_orbit_camera(
    time: Res<Time>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    ev_motion: Res<Events<MouseMotion>>,
    mousebtn: Res<Input<MouseButton>>,
    ev_scroll: Res<Events<MouseWheel>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    let mut translation = Vec2::zero();
    let mut rotation_move = Vec2::default();
    let mut scroll = 0.0;
    let dt = time.delta_seconds();

    if mousebtn.pressed(MouseButton::Right) {
        for ev in state.reader_motion.iter(&ev_motion) {
            rotation_move += ev.delta;
        }
    } else if mousebtn.pressed(MouseButton::Left) {
        // Pan only if we're not rotating at the moment
        for ev in state.reader_motion.iter(&ev_motion) {
            translation += ev.delta;
        }
    }

    for ev in state.reader_scroll.iter(&ev_scroll) {
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

            let look = Mat4::face_toward(trans.translation, camera.focus, Vec3::new(0.0, 1.0, 0.0));
            trans.rotation = look.to_scale_rotation_translation().1;
        } else {
            // The plane is x/y while z is "up". Multiplying by dt allows for a constant pan rate
            let mut translation = Vec3::new(-translation.x * dt, translation.y * dt, 0.0);
            camera.focus += translation;
            translation.z = -scroll;
            trans.translation += translation;
        }
    }
}

/// Spawn a camera like this.
fn spawn_camera(commands: &mut Commands) {
    commands.spawn((PanOrbitCamera::default(),))
        .with_bundle(Camera3dBundle::default())
        .insert_resource(InputState::default());
}
```
