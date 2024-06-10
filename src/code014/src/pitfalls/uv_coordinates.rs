use bevy::prelude::*;

fn setup(mut commands: Commands) {
// ANCHOR: sprite-flipping
commands.spawn(SpriteBundle {
    sprite: Sprite {
        flip_y: true,
        flip_x: false,
        ..Default::default()
    },
    ..Default::default()
});
// ANCHOR_END: sprite-flipping
}

fn _main() {
    App::new().add_plugins(DefaultPlugins)
        .add_systems(Startup, setup);
}

