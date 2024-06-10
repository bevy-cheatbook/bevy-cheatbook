use bevy::prelude::*;

#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}
#[derive(Component)]
struct PlayerXp(u32);
#[derive(Component)]
struct PlayerName(String);
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Friendly;
#[derive(Component)]
struct LowHpMarker;
#[derive(Component)]
struct CurrentModifier;
#[derive(Component)]
struct PlayerPendingAction;
#[derive(Component)]
struct StatusEffect;

// ANCHOR: bundle
#[derive(Bundle)]
struct PlayerBundle {
    xp: PlayerXp,
    name: PlayerName,
    health: Health,
    marker: Player,

    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    sprite: SpriteBundle,
}
// ANCHOR_END: bundle

// ANCHOR: cleanup-bundle
/// Contains all components to remove when
/// resetting the player between rooms/levels.
#[derive(Bundle)]
struct PlayerResetCleanupBundle {
    status_effect: StatusEffect,
    pending_action: PlayerPendingAction,
    modifier: CurrentModifier,
    low_hp_marker: LowHpMarker,
}
// ANCHOR_END: cleanup-bundle

// ANCHOR: bundle-default
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            xp: PlayerXp(0),
            name: PlayerName("Player".into()),
            health: Health {
                hp: 100.0,
                extra: 0.0,
            },
            marker: Player,
            sprite: Default::default(),
        }
    }
}
// ANCHOR_END: bundle-default

fn setup(mut commands: Commands) {
let e_player = Entity::PLACEHOLDER;
// ANCHOR: bundle-spawn
commands.spawn(PlayerBundle {
    xp: PlayerXp(0),
    name: PlayerName("Player 1".into()),
    health: Health {
        hp: 100.0,
        extra: 0.0,
    },
    marker: Player,
    sprite: SpriteBundle {
        // TODO
        ..Default::default()
    },
});
// ANCHOR_END: bundle-spawn
// ANCHOR: bundle-spawn-default
commands.spawn(PlayerBundle {
    name: PlayerName("Player 1".into()),
    ..Default::default()
});
// ANCHOR_END: bundle-spawn-default
// ANCHOR: bundle-spawn-loose
commands.spawn((
    SpriteBundle {
        // ...
        ..default()
    },
    Health {
        hp: 50.0,
        extra: 0.0,
    },
    Enemy,
    // ...
));
// ANCHOR_END: bundle-spawn-loose
// ANCHOR: cleanup-bundle-remove
commands.entity(e_player)
    .remove::<PlayerResetCleanupBundle>();
// ANCHOR_END: cleanup-bundle-remove
}

fn main() {
    let app = App::new()
        .add_systems(Startup, setup);
}
