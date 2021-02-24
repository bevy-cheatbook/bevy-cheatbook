use bevy::prelude::*;

// ANCHOR: example
struct AssetsLoading(Vec<HandleUntyped>);

fn setup(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    // we can have different asset types
    let font: Handle<Font> = server.load("my_font.ttf");
    let menu_bg: Handle<Texture> = server.load("menu.png");
    let scene: Handle<Scene> = server.load("level01.gltf#Scene0");

    // add them all to our collection for tracking
    loading.0.push(font.clone_untyped());
    loading.0.push(menu_bg.clone_untyped());
    loading.0.push(scene.clone_untyped());
}

fn check_assets_ready(
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    use bevy::asset::LoadState;

    let mut ready = true;

    for handle in loading.0.iter() {
        match server.get_load_state(handle) {
            LoadState::Failed => {
                // one of our assets had an error
            }
            LoadState::Loaded => {}
            _ => {
                ready = false;
            }
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
        .add_startup_system(setup.system())
        .add_system(check_assets_ready.system())
        .run();
}
