use bevy::prelude::*;

// ANCHOR: main
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;

fn set_window_icon(
    windows: Res<WinitWindows>,
) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("my_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(set_window_icon)
        .run();
}
// ANCHOR_END: main
