use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    game::{
        combat::HitBoxBundle,
        factions::Faction,
    },
    physics::{groups, Velocity},
};

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, projectile_movement)
            .add_systems(PostUpdate, update_lifetimes);
    }
}

#[derive(Component)]
pub struct Projectile {
    lifetime: Timer,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    name: Name,
    projectile: Projectile,
    faction: Faction,
    velocity: Velocity,
    hit_box: HitBoxBundle,
    shape: ShapeBundle,
    fill: Fill,
}

impl ProjectileBundle {
    pub fn new(pos: Vec2, vel: Vec2, damage: f32, faction: Faction) -> Self {
        let radius = 8.0;
        let shape = shapes::Circle {
            radius,
            ..default()
        };
        let transform = Transform::from_translation(pos.extend(0.0));
        Self {
            name: Name::new("Projectile"),
            projectile: Projectile {
                lifetime: Timer::from_seconds(5.0, TimerMode::Once)
            },
            faction,
            velocity: Velocity::new(vel),
            hit_box: HitBoxBundle::circle(radius, damage, groups::NONE),
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(transform),
                ..default()
            },
            fill: Fill::color(Color::CYAN * 4.0),
        }
    }
}

pub fn projectile_movement(
    time: Res<Time>,
    mut projectile_q: Query<(&mut Transform, &Velocity), With<Projectile>>,
) {
    let dt = time.delta_seconds();
    for (mut transform, velocity) in projectile_q.iter_mut() {
        transform.translation += velocity.inner.extend(0.0) * dt;
    }
}

pub fn update_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_q: Query<(Entity, &mut Projectile)>,
) {
    let dt = time.delta();
    for (entity, mut projectile) in projectile_q.iter_mut() {
        if projectile.lifetime.tick(dt).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
