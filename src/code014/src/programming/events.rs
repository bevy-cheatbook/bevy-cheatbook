use bevy::prelude::*;

#[derive(Component)]
struct PlayerXp(u32);

// ANCHOR: events
#[derive(Event)]
struct LevelUpEvent(Entity);

fn player_level_up(
    mut ev_levelup: EventWriter<LevelUpEvent>,
    query: Query<(Entity, &PlayerXp)>,
) {
    for (entity, xp) in query.iter() {
        if xp.0 > 1000 {
            ev_levelup.send(LevelUpEvent(entity));
        }
    }
}

fn debug_levelups(
    mut ev_levelup: EventReader<LevelUpEvent>,
) {
    for ev in ev_levelup.read() {
        eprintln!("Entity {:?} leveled up!", ev.0);
    }
}
// ANCHOR_END: events

fn main() {
    let mut app = App::new();
// ANCHOR: events-appbuilder
app.add_event::<LevelUpEvent>();
// ANCHOR_END: events-appbuilder
    // ANCHOR: events-update-system
    app.add_systems(Update, (player_level_up, debug_levelups));
    // ANCHOR_END: events-update-system
}
