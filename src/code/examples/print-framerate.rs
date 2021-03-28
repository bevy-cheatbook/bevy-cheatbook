use bevy::prelude::*;

// ANCHOR: main
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.system())
        .run();
}
// ANCHOR_END: main
