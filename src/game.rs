use bevy::prelude::*;

pub mod ai;
pub mod camera;
pub mod combat;
pub mod enemies;
pub mod factions;
pub mod health;
pub mod input;
pub mod projectiles;
pub mod units;
pub mod waves;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ai::AiPlugin,
                camera::CameraPlugin,
                combat::CombatPlugin,
                enemies::EnemiesPlugin,
                health::HealthPlugin,
                input::InputPlugin,
                projectiles::ProjectilesPlugin,
                units::UnitsPlugin,
                waves::WavesPlugin,
            ))
            .add_systems(Startup, start_game);
    }
}

fn start_game(
    mut commands: Commands,
) {
    camera::spawn_camera(&mut commands, 2.0);

    // Spawn swarm
    units::spawn_swarm(&mut commands);
}
