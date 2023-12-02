use bevy::prelude::*;

// ANCHOR: custom-canvas
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // provide the ID selector string here
            canvas: Some("#mygame-canvas".into()),
            // ... any other window properties ...
            ..default()
        }),
        ..default()
    }));
    // ...
    app.run();
}
// ANCHOR_END: custom-canvas
