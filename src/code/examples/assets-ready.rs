use bevy::prelude::*;

#[derive(Default)]
// ANCHOR: example
struct AssetsLoading(Vec<HandleUntyped>);

fn setup(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
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
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            // all assets are now ready

            // this might be a good place to transition into your in-game state

            // remove the resource to drop the tracking handles
            commands.remove_resource::<AssetsLoading>();
            // (note: if you don't have any other handles to the assets
            // elsewhere, they will get unloaded after this)
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
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
