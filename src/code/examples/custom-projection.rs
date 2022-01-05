use bevy::prelude::*;

// ANCHOR: example
use bevy::render::camera::{Camera, CameraProjection, DepthCalculation, CameraPlugin};
use bevy::render::view::VisibleEntities;

#[derive(Component)]
struct SimpleOrthoProjection {
    far: f32,
    aspect: f32,
}

impl CameraProjection for SimpleOrthoProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            -self.aspect, self.aspect, -1.0, 1.0, 0.0, self.far
        )
    }

    // what to do on window resize
    fn update(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        // for 2D (camera doesn't rotate)
        //DepthCalculation::ZDifference

        // otherwise
        DepthCalculation::Distance
    }

    fn far(&self) -> f32 {
        self.far
    }
}

impl Default for SimpleOrthoProjection {
    fn default() -> Self {
        Self { far: 1000.0, aspect: 1.0 }
    }
}

fn setup(mut commands: Commands) {
    // same components as bevy's Camera2dBundle,
    // but with our custom projection

    let projection = SimpleOrthoProjection::default();

    // Need to set the camera name to one of the bevy-internal magic constants,
    // depending on which camera we are implementing (2D, 3D, or UI).
    // Bevy uses this name to find the camera and configure the rendering.
    // Since this example is a 2d camera:

    let cam_name = CameraPlugin::CAMERA_2D;

    let mut camera = Camera::default();
    camera.name = Some(cam_name.to_string());

    commands.spawn_bundle((
        // position the camera like bevy would do by default for 2D:
        Transform::from_translation(Vec3::new(0.0, 0.0, projection.far - 0.1)),
        GlobalTransform::default(),
        VisibleEntities::default(),
        camera,
        projection,
    ));
}

fn main() {
    // need to add a bevy-internal camera system to update
    // the projection on window resizing

    use bevy::render::camera::camera_system;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            camera_system::<SimpleOrthoProjection>,
        )
        .run();
}
// ANCHOR_END: example
