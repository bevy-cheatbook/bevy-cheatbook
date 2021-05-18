use bevy::{asset::HandleId, prelude::*};

#[derive(Default)]
// ANCHOR: example
struct AssetsLoading(Vec<HandleId>);

fn setup(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    // we can have different asset types
    let font: Handle<Font> = server.load("my_font.ttf");
    let menu_bg: Handle<Texture> = server.load("menu.png");
    let scene: Handle<Scene> = server.load("level01.gltf#Scene0");

    // add them all to our collection for tracking
    loading.0.push(font.id);
    loading.0.push(menu_bg.id);
    loading.0.push(scene.id);
}

fn check_assets_ready(
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    use bevy::asset::LoadState;

    let mut ready = true;

    match server.get_group_load_state(loading.0.clone()) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {}
        _ => {
            ready = false;
        }
    }

    if !ready {
        return;
    }

    // all assets are now ready
}
// ANCHOR_END: example

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<AssetsLoading>()
        .add_startup_system(setup.system())
        .add_system(check_assets_ready.system())
        .run();
}
