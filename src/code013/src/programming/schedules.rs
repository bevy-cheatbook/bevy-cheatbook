use bevy::{app::MainScheduleOrder, ecs::schedule::{ExecutorKind, LogLevel, ScheduleBuildSettings, ScheduleLabel}, prelude::*};

fn camera_movement() {}
fn setup_camera() {}
fn my_weird_custom_stuff() {}
fn system_a() {}
fn system_b() {}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyGameplaySet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyUiSet;

// ANCHOR: custom-schedule
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct PrepareUpdate;
// ANCHOR_END: custom-schedule

fn main() {
let mut app = App::new();
// ANCHOR: add-system
// add something to the Update schedule (runs every frame)
app.add_systems(Update, camera_movement);

// add something to the Startup schedule (runs once at app startup)
app.add_systems(Startup, setup_camera);
// ANCHOR_END: add-system

// ANCHOR: single-threaded
// Make FixedUpdate run single-threaded
app.edit_schedule(FixedUpdate, |schedule| {
    schedule.set_executor_kind(ExecutorKind::SingleThreaded);

    // or alternatively: Simple will apply Commands after every system
    schedule.set_executor_kind(ExecutorKind::Simple);
});
// ANCHOR_END: single-threaded

// ANCHOR: ambiguity-detector
// Enable ambiguity warnings for the Update schedule
app.edit_schedule(Update, |schedule| {
    schedule.set_build_settings(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        ..default()
    });
});
// ANCHOR_END: ambiguity-detector

// ANCHOR: disable-auto-apply-deferred
app.edit_schedule(Update, |schedule| {
    schedule.set_build_settings(ScheduleBuildSettings {
        auto_insert_apply_deferred: false,
        ..default()
    });
});
// ANCHOR_END: disable-auto-apply-deferred

// ANCHOR: apply-deferred
app.add_systems(
    Update,
    apply_deferred
        .after(MyGameplaySet)
        .before(MyUiSet)
);
app.add_systems(Update, (
    (
        system_a,
        apply_deferred,
        system_b,
    ).chain(),
));
// ANCHOR_END: apply-deferred

// ANCHOR: custom-schedule-app
// Ensure the schedule has been created
// (this is technically optional; Bevy will auto-init
// the schedule the first time it is used)
app.init_schedule(PrepareUpdate);

// Add it to the MainScheduleOrder so it runs every frame
// as part of the Main schedule. We want our PrepareUpdate
// schedule to run after StateTransition.
app.world.resource_mut::<MainScheduleOrder>()
    .insert_after(StateTransition, PrepareUpdate);

// Now we can add some systems to our new schedule!
app.add_systems(PrepareUpdate, (
    my_weird_custom_stuff,
));
// ANCHOR_END: custom-schedule-app
}
