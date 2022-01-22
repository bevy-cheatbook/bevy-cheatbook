#![allow(clippy::too_many_arguments)]

use std::ops::{Div, Mul, Neg};

use bevy::{
   input::mouse::{MouseMotion, MouseWheel},
   prelude::*,
};

// ANCHOR: example
/// Trait for customizing camera behavior.
pub trait MovableCameraParams {
   const DEFAULT_SPEED: f32 = 1.0;
   const ACCELERATION: f32 = 1.0;
   const SLOW_SPEED: f32 = 0.1;
   const SCROLL_SNAP: f32 = 1.0;
   const FORWARD: KeyCode = KeyCode::W;
   const BACKWARD: KeyCode = KeyCode::S;
   const LEFT: KeyCode = KeyCode::A;
   const RIGHT: KeyCode = KeyCode::D;
   const UPWARD: KeyCode = KeyCode::E;
   const DOWNWARD: KeyCode = KeyCode::Q;
   const CHANGE_SPEED: KeyCode = KeyCode::LShift;
   const FOCUS: KeyCode = KeyCode::F;
}

/// Dummy struct to implement the trait on.
struct CameraParams;

impl MovableCameraParams for CameraParams {}

/// Tags an entity as being capable of moving, rotating, and orbiting.
#[derive(Component)]
struct MovableCamera {
   pub speed: f32,
   pub angular_speed: f32,
   pub slow: bool,
   pub cursor_pos: Vec2,
   pub focused: bool,
}

impl Default for MovableCamera {
   fn default() -> Self {
      Self {
         speed: CameraParams::DEFAULT_SPEED,
         angular_speed: CameraParams::DEFAULT_SPEED,
         slow: false,
         cursor_pos: Vec2::default(),
         focused: false,
      }
   }
}

/// Takes a quaternion as input and clamps it between -tau/4 and tau/4.
fn limit_pitch(tq: Quat) -> Quat {
   // Produce new quaternion with zeroed x and z and normalized y and w
   // from the input quaternion.
   // This results in a quaternion that represents the yaw component
   // of the original quaternion.
   // Not tested for quaternions that have roll other than 0.
   let qy = Quat::from_xyzw(0.0, tq.y, 0.0, tq.w).normalize();
   // Remove yaw from input quaternion, leaving pitch
   let qp = qy.inverse().mul(tq);
   // Convert quaternion to Euler angle
   let pitch = Vec3::X.dot(qp.xyz()).asin().mul(2.0);
   // Clamp angle to range
   let quarter_tau = std::f32::consts::TAU / 4.0;
   let clamped_pitch = pitch.clamp(quarter_tau.neg(), quarter_tau);
   // Convert angle back to quaternion
   let qp = Quat::from_rotation_x(clamped_pitch);
   // Multiply yaw quaternion by pitch quaternion to get constrained quaternion
   qy.mul(qp)
}

/// Rotates a camera quat by a linear amount.
fn rotate_cam_quat(window_size: Vec2, motion: Vec2, speed: f32, mut tq: Quat) -> Quat {
   let delta_x = motion
      .x
      .div(window_size.x)
      .mul(std::f32::consts::TAU)
      .mul(speed);
   let delta_y = motion
      .y
      .div(window_size.y)
      .mul(std::f32::consts::TAU.div(2.0))
      .mul(speed);
   let delta_yaw = Quat::from_rotation_y(delta_x.neg());
   let delta_pitch = Quat::from_rotation_x(delta_y.neg());
   // note the order of the following multiplications
   tq = delta_yaw.mul(tq); // yaw around GLOBAL y axis
   tq = tq.mul(delta_pitch); // pitch around LOCAL x axis
   limit_pitch(tq)
}

fn get_primary_window_size(windows: &ResMut<Windows>) -> Vec2 {
   let window = windows.get_primary().unwrap();
   Vec2::new(window.width() as f32, window.height() as f32)
}

fn net_movement(keys: &Res<Input<KeyCode>>, negative: KeyCode, positive: KeyCode) -> f32 {
   match (keys.pressed(negative), keys.pressed(positive)) {
      (true, false) => -1.0,
      (false, true) => 1.0,
      _ => 0.0,
   }
}

/// Prevents the cursor from moving.
fn lock_cursor(
   mut windows: ResMut<Windows>,
   buttons: Res<Input<MouseButton>>,
   mut cam: Query<&mut MovableCamera>,
) {
   let mut cam = cam.single_mut();
   if buttons.just_pressed(MouseButton::Right) {
      if let Some(window) = windows.get_primary_mut() {
         window.set_cursor_lock_mode(true);
         if let Some(pos) = window.cursor_position() {
            cam.cursor_pos = pos;
         }
      }
   }

   if buttons.just_released(MouseButton::Right) {
      if let Some(window) = windows.get_primary_mut() {
         window.set_cursor_lock_mode(false);
      }
   }

   if buttons.pressed(MouseButton::Right) {
      if let Some(window) = windows.get_primary_mut() {
         window.set_cursor_position(cam.cursor_pos);
      }
   }
}

/// Adjusts the camera speed based on user input.
fn adjust_cam_speed<P>(
   time: Res<Time>,
   keys: Res<Input<KeyCode>>,
   mut cam: Query<&mut MovableCamera>,
) where
   P: MovableCameraParams,
{
   let mut cam = cam.single_mut();
   if keys.just_pressed(P::CHANGE_SPEED) {
      cam.slow = !cam.slow;
      if !cam.slow {
         cam.speed = P::DEFAULT_SPEED;
      }
   }

   if cam.slow {
      cam.speed = P::SLOW_SPEED;
      cam.angular_speed = P::SLOW_SPEED;
   } else if keys.any_pressed([
      P::LEFT,
      P::RIGHT,
      P::UPWARD,
      P::BACKWARD,
      P::DOWNWARD,
      P::FORWARD,
   ]) {
      cam.speed += CameraParams::ACCELERATION.mul(time.delta_seconds());
   } else {
      cam.speed = CameraParams::DEFAULT_SPEED;
      cam.angular_speed = CameraParams::DEFAULT_SPEED;
   }
}

/// Move the camera with QWEASD, zoom with wheel, focus at
/// camera pos with F, and rotate/orbit with right mouse button.
fn movable_camera<P>(
   windows: ResMut<Windows>,
   time: Res<Time>,
   keys: Res<Input<KeyCode>>,
   buttons: Res<Input<MouseButton>>,
   mut motion: EventReader<MouseMotion>,
   mut scroll_evr: EventReader<MouseWheel>,
   mut q_child: Query<(
      &Parent,
      &mut Transform,
      &mut MovableCamera,
      &PerspectiveProjection,
   )>,
   mut q_parent: Query<(&mut Transform, &GlobalTransform), Without<PerspectiveProjection>>,
) where
   P: MovableCameraParams,
{
   for (parent, mut transform_child, mut cam, ..) in q_child.iter_mut() {
      // Focused Camera
      if cam.focused {
         if keys.any_pressed([
            CameraParams::LEFT,
            CameraParams::RIGHT,
            CameraParams::UPWARD,
            CameraParams::BACKWARD,
            CameraParams::DOWNWARD,
            CameraParams::FORWARD,
         ]) {
            if let Ok((mut transform_parent, ..)) = q_parent.get_mut(parent.0) {
               let zoom = transform_child.translation.z;
               // Set child transform to parent transform
               *transform_child = *transform_parent;
               // Offset child by its zoom
               transform_child.translation += zoom.mul(transform_parent.back());
               // Set parent transform to origin
               *transform_parent = Transform::default();
            }
            cam.focused = false;
         }
      } else if keys.just_pressed(P::FOCUS) {
         if let Ok((mut transform_parent, ..)) = q_parent.get_mut(parent.0) {
            // Hand off position and orientation information to parent
            *transform_parent = *transform_child;
         }
         *transform_child = Transform::default();
         cam.focused = true;
      }

      let mut rotation_move = Vec2::ZERO;
      let mut scroll = 0.0;

      if buttons.pressed(MouseButton::Right) {
         for ev in motion.iter() {
            rotation_move += ev.delta;
         }
      }

      for event in scroll_events.iter() {
         scroll += ev.y;
      }

      if cam.focused {
         // Orbit the camera
         if rotation_move.length_squared() > 0.0 {
            if let Ok((mut transform_parent, ..)) = q_parent.get_mut(parent.0) {
               let window_size = get_primary_window_size(&windows);
               transform_parent.rotation = rotate_cam_quat(
                  window_size,
                  rotation_move,
                  cam.angular_speed,
                  transform_parent.rotation,
               );
            }
         }

         // Zoom the camera. Parent has orientation information so just
         // mutate child's z
         if scroll.abs() > 0.0 {
            transform_child.translation -= Vec3::new(0.0, 0.0, 1.0)
               .mul(CameraParams::SCROLL_SNAP)
               .mul(scroll)
               .mul(cam.speed);
            // Clamp the child's translation so it can't go past focus (the parent)
            transform_child.translation = transform_child.translation.max(Vec3::new(0.0, 0.0, 0.0));
         }
      // Free Camera
      } else {
         // Rotate the camera
         if rotation_move.length_squared() > 0.0 {
            let window_size = get_primary_window_size(&windows);
            transform_child.rotation = rotate_cam_quat(
               window_size,
               rotation_move,
               cam.angular_speed,
               transform_child.rotation,
            );
         }

         // Zoom the camera relative to camera orientation
         if scroll.abs() > 0.0 {
            let transform_clone = *transform_child;
            transform_child.translation += transform_clone
               .forward()
               .mul(CameraParams::SCROLL_SNAP)
               .mul(scroll)
               .mul(cam.speed);
         }

         let mut translate_move = Vec3::new(
            net_movement(&keys, P::RIGHT, P::LEFT),
            net_movement(&keys, P::DOWNWARD, P::UPWARD),
            net_movement(&keys, P::BACKWARD, P::FORWARD),
         )
         .normalize_or_zero();

         // Translate the camera
         if translate_move.length_squared() > 0.0 {
            translate_move = translate_move.mul(time.delta_seconds()).mul(cam.speed);
            // Clone the child's transform so we can use its immutable methods
            let transform_clone = *transform_child;
            // Translate camera along each of its local axes
            transform_child.translation += transform_clone.left().mul(translate_move.x);
            transform_child.translation += transform_clone.up().mul(translate_move.y);
            transform_child.translation += transform_clone.forward().mul(translate_move.z);
         }
      }
   }
}

/// Spawn a camera like this. Note the extra bundle.
fn spawn_camera(mut commands: Commands) {
   commands
      .spawn_bundle((
         Transform::from_xyz(0.0, 0.0, 0.0),
         GlobalTransform::default(),
      ))
      .with_children(|parent| {
         parent
            .spawn_bundle(PerspectiveCameraBundle {
               transform: Transform::from_xyz(0.0, 3.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
               ..Default::default()
            })
            .insert(MovableCamera::default());
      });
}
// ANCHOR_END: example

/// set up a simple 3D scene
fn setup(
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<StandardMaterial>>,
) {
   // plane
   commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      ..Default::default()
   });
   // cubes
   commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..Default::default()
   });
   commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
      transform: Transform::from_xyz(0.5, 2.0, 0.5),
      ..Default::default()
   });
   commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
      transform: Transform::from_xyz(-0.5, 2.0, -0.5),
      ..Default::default()
   });
   // light
   commands.spawn_bundle(PointLightBundle {
      point_light: PointLight {
         intensity: 1500.0,
         shadows_enabled: true,
         ..Default::default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..Default::default()
   });
}

fn main() {
   App::new()
      .insert_resource(Msaa { samples: 4 })
      .add_plugins(DefaultPlugins)
      .add_startup_system(setup)
      .add_startup_system(spawn_camera)
      .add_system(lock_cursor)
      .add_system(adjust_cam_speed::<CameraParams>)
      .add_system(movable_camera::<CameraParams>)
      .run();
}
