#![allow(unused_variables)]

use bevy::prelude::*;

struct MySpecialEvent;
struct MyRegularEvent;

// ANCHOR: main
use bevy::ecs::event::Events;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // add the `Events<T>` resource manually
        // these events will not have automatic cleanup
        .init_resource::<Events<MySpecialEvent>>()

        // this is a regular event type with automatic cleanup
        .add_event::<MyRegularEvent>()

        // add the cleanup systems
        .add_system(my_event_manager::<MySpecialEvent>)
        .run();
}

/// Custom cleanup strategy for events
///
/// Generic to allow using for any custom event type
fn my_event_manager<T: 'static + Send + Sync>(
    mut events: ResMut<Events<T>>,
) {
    // TODO: implement your custom logic
    // for deciding when to clear the events

    // clear all events like this:
    events.clear();

    // or with double-buffering
    // (this is what Bevy's default strategy does)
    events.update();

    // or drain them, if you want to iterate,
    // to access the values:
    for event in events.drain() {
        // TODO: do something with each event
    }
}
// ANCHOR_END: main
