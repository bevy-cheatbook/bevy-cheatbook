use bevy::prelude::*;

// ANCHOR: example
use bevy::render::primitives::Frustum;
use bevy::render::camera::{Camera, Camera2d, CameraProjection, DepthCalculation};
use bevy::render::view::VisibleEntities;

#[derive(Component)]
struct SimpleOrthoProjection {
    near: f32,
    far: f32,
    aspect: f32,
}

impl CameraProjection for SimpleOrthoProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            -self.aspect, self.aspect, -1.0, 1.0, self.near, self.far
        )
    }

    // what to do on window resize
    fn update(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        // for 2D (camera doesn't rotate)
        DepthCalculation::ZDifference

        // otherwise
        //DepthCalculation::Distance
    }

    fn far(&self) -> f32 {
        self.far
    }
}

impl Default for SimpleOrthoProjection {
    fn default() -> Self {
        Self { near: 0.0, far: 1000.0, aspect: 1.0 }
    }
}

fn setup(mut commands: Commands) {
    // We need all the components that Bevy's built-in camera bundles would add

    let projection = SimpleOrthoProjection::default();
    let camera = Camera {
        near: projection.near,
        far: projection.far,
        ..default()
    };
    // position the camera like bevy would do by default for 2D:
    let transform = Transform::from_xyz(0.0, 0.0, projection.far - 0.1);
    // frustum construction code copied from Bevy
    let view_projection =
        projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = Frustum::from_view_projection(
        &view_projection,
        &transform.translation,
        &transform.back(),
        projection.far,
    );

    commands.spawn_bundle((
        camera,
        projection,
        frustum,
        VisibleEntities::default(),
        transform,
        GlobalTransform::default(),
        Camera2d,
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
