use bevy::prelude::*;

#[derive(Resource)]
struct MyNetProto;

impl MyNetProto {
    fn send_updates(&mut self) -> std::io::Result<()> {
        Ok(())
    }
    fn receive_updates(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// ANCHOR: system-io
fn net_receive(mut netcode: ResMut<MyNetProto>) -> std::io::Result<()> {
    netcode.send_updates(/* ... */)?;
    netcode.receive_updates(/* ... */)?;

    Ok(())
}

fn handle_io_errors(
    In(result): In<std::io::Result<()>>,
    // we can also have regular system parameters
    mut commands: Commands,
) {
    if let Err(e) = result {
        eprintln!("I/O error occurred: {}", e);
        // Maybe spawn some error UI or something?
        commands.spawn((/* ... */));
    }
}
// ANCHOR_END: system-io

fn main() {
let mut app = App::new();
// ANCHOR: system-pipe
app.add_systems(FixedUpdate, net_receive.pipe(handle_io_errors));
// ANCHOR_END: system-pipe
}
