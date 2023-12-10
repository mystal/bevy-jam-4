use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::game::{
    factions::Faction,
    projectiles::ProjectileBundle,
};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, simple_shooter_ai);
    }
}

#[derive(Component)]
pub struct SimpleShooterAi {
    cooldown: f32,
    variance: RangeInclusive<f32>,
    cooldown_expires: f32,
}

impl SimpleShooterAi {
    pub fn new(cooldown: f32, variance: RangeInclusive<f32>) -> Self {
        Self {
            cooldown,
            variance,
            cooldown_expires: -1.0,
        }
    }
}

fn simple_shooter_ai(
    mut commands: Commands,
    time: Res<Time>,
    mut ai_q: Query<(&mut SimpleShooterAi, &GlobalTransform, &Faction)>,
) {
    let now = time.elapsed_seconds();
    for (mut ai, transform, faction) in ai_q.iter_mut() {
        if ai.cooldown_expires > now {
            continue;
        }

        if ai.cooldown_expires > 0.0 {
            // Spawn a projectile.
            let pos = transform.translation().truncate() + Vec2::Y * -20.0;
            let vel = Vec2::Y * -1000.0;
            commands.spawn(ProjectileBundle::new(pos, vel, 1.0, *faction));
        }

        // Set a new cooldown_expires.
        let variance_range = ai.variance.end() - ai.variance.start();
        let variance = ai.variance.start() + (fastrand::f32() * variance_range);
        ai.cooldown_expires = now + ai.cooldown + variance;
    }
}
