use bevy::prelude::*;

// ANCHOR: dnd-file
fn file_drop(
    mut evr_dnd: EventReader<FileDragAndDrop>,
) {
    for ev in evr_dnd.read() {
        if let FileDragAndDrop::DroppedFile { id, path_buf } = ev {
            println!("Dropped file with path: {:?}, in window id: {:?}", path_buf, id);
        }
    }
}
// ANCHOR_END: dnd-file

fn main() {
    let mut app = App::new();
    app.add_systems(Update, file_drop);
}
