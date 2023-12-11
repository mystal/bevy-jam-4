use bevy::prelude::*;

use crate::game::{
    ai,
    enemies::EnemyBundle,
};

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WavesManager>()
            .add_systems(Update, waves_manager);
    }
}

#[derive(Default, Resource)]
pub struct WavesManager {
    spawned_enemies: Vec<Entity>,
}

#[derive(Component)]
pub struct WaveEntity;

fn waves_manager(
    mut commands: Commands,
    mut waves_manager: ResMut<WavesManager>,
    mut removed: RemovedComponents<WaveEntity>
) {
    for entity in removed.read() {
        if let Some(i) = waves_manager.spawned_enemies.iter().position(|&e| e == entity) {
            waves_manager.spawned_enemies.swap_remove(i);
        }
    }

    if waves_manager.spawned_enemies.is_empty() {
        let x = -500.0;
        let spacing = 100.0;
        for i in 0..10 {
            let entity = commands.spawn((
                EnemyBundle::new(Vec2::new(x + spacing * i as f32, 500.0)),
                ai::SimpleShooterAi::new(3.0, 0.0..=2.0),
                WaveEntity,
            )).id();
            waves_manager.spawned_enemies.push(entity);
        }
    }
}
