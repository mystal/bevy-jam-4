use bevy::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;

pub mod combat;
pub mod enemies;
pub mod input;
pub mod projectiles;
pub mod units;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                combat::CombatPlugin,
                enemies::EnemiesPlugin,
                input::InputPlugin,
                projectiles::ProjectilesPlugin,
                units::UnitsPlugin,
            ))
            .add_systems(Startup, start_game);
    }
}

fn start_game(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scale: 2.0,
                ..default()
            },
            ..default()
        },
        BloomSettings::default(),
    ));

    // Spawn swarm
    units::spawn_swarm(&mut commands);
}
