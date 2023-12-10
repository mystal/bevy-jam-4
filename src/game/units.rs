use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    game::{
        combat::HurtBoxBundle,
        factions::Faction,
        health::Health,
        input::PlayerInput,
        projectiles::ProjectileBundle,
    },
    physics::{groups, PlayerMovement, Velocity},
};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SwarmParent>()
            .add_systems(Update, (
                shooter_flock_movement,
                shooter_fire,
            ).chain());
    }
}

#[derive(Component, Reflect)]
pub struct SwarmParent {
    pub separation: f32,
    pub alignment: f32,
    pub cohesion: f32,
    pub separation_dist: f32,
    pub cohesion_dist: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub shooter_cooldown: f32,
    pub last_fired_time: f32,
}

impl SwarmParent {
    pub fn new() -> Self {
        Self {
            separation: 1.2,
            alignment: 1.0,
            cohesion: 1.0,
            separation_dist: 50.0,
            cohesion_dist: 30.0,
            max_speed: 200.0,
            max_force: 1.0,
            shooter_cooldown: 2.0,
            last_fired_time: -1.0,
        }
    }
}

#[derive(Component)]
pub struct BasicShooter {
    last_fired: f32,
    cooldown: f32,
}

#[derive(Bundle)]
pub struct BasicShooterBundle {
    name: Name,
    shooter: BasicShooter,
    faction: Faction,
    velocity: Velocity,
    health: Health,
    hurt_box: HurtBoxBundle,
    shape: ShapeBundle,
    fill: Fill,
}

impl BasicShooterBundle {
    pub fn new(pos: Vec2) -> Self {
        let shape = shapes::RegularPolygon {
            sides: 3,
            feature: RegularPolygonFeature::Radius(20.0),
            ..default()
        };
        let transform = Transform::from_translation(pos.extend(0.0));
        Self {
            name: Name::new("BasicShooter"),
            shooter: BasicShooter {
                last_fired: -1.0,
                cooldown: 1.0,
            },
            faction: Faction::Player,
            velocity: Velocity::default(),
            health: Health::new(1.0),
            hurt_box: HurtBoxBundle::circle(16.0, groups::PLAYER),
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(transform),
                ..default()
            },
            fill: Fill::color(Color::ORANGE * 4.0),
        }
    }
}

// TODO: Set up a simple boids simulation for the main cluster of ships.

pub fn spawn_swarm(commands: &mut Commands) {
    commands.spawn((
        Name::new("SwarmParent"),
        SwarmParent::new(),
        PlayerMovement::default(),
        PlayerInput::default(),
        SpatialBundle::default(),
        Faction::Player,
    )).with_children(|b| {
        for _ in 0..10 {
            let radius = 150.0;
            let x = (fastrand::f32() * 2.0) - 1.0;
            let y = (fastrand::f32() * 2.0) - 1.0;
            let pos = Vec2::new(x, y) * radius;
            b.spawn(BasicShooterBundle::new(pos));
        }
    });
}

fn shooter_flock_movement(
    parent_q: Query<(&Children, &Transform, &SwarmParent)>,
    mut flock_q: Query<(&mut Transform, &mut Velocity), (With<BasicShooter>, Without<SwarmParent>)>,
) {
    // TODO: Get list of children in parent. Do iter_many over those Entities.
    let Ok((children, transform, swarm)) = parent_q.get_single() else {
        return;
    };
    // let swarm_pos = transform.translation.truncate();
    let swarm_pos = Vec2::ZERO;

    for child in children {
        let Ok((&transform, &velocity)) = flock_q.get(*child) else {
            continue;
        };
        let pos = transform.translation.truncate();

        let sepration = {
            // Try to steer away from nearby boids.
            let mut steer = Vec2::ZERO;
            let mut count = 0;

            // Check if we're too close to all other boids.
            for (other_transform, _) in flock_q.iter() {
                let other_pos = other_transform.translation.truncate();
                let dist = pos.distance(other_pos);

                // If we're too close, modify our steering vector.
                if dist > 0.0 && dist < swarm.separation_dist {
                    let diff = (pos - other_pos).normalize() / dist;
                    steer += diff;
                    count += 1;
                }
            }

            // Average out the steering.
            if count > 0 {
                steer /= count as f32;
            }

            if steer != Vec2::ZERO {
                steer = steer.clamp_length(swarm.max_speed, swarm.max_speed);
                steer -= velocity.inner;
                steer = steer.clamp_length_max(swarm.max_force);
            }

            steer
        };
        let alignment = {
            let desired = (swarm_pos - pos).clamp_length(swarm.max_speed, swarm.max_speed);
            (desired - velocity.inner).clamp_length_max(swarm.max_force)
        };
        let cohesion = {
            // Try to move to the center of nearby boids.
            let mut sum = Vec2::ZERO;
            let mut count = 0;

            for (other_transform, _) in flock_q.iter() {
                let other_pos = other_transform.translation.truncate();
                let dist = pos.distance(other_pos);
                if dist > 0.0 && dist < swarm.cohesion_dist {
                    sum += other_pos;
                    count += 1;
                }
            }

            if count > 0 {
                let avg_pos = sum / count as f32;
                let desired = (avg_pos - pos).clamp_length(swarm.max_speed, swarm.max_speed);
                (desired - velocity.inner).clamp_length_max(swarm.max_force)
            } else {
                Vec2::ZERO
            }
        };

        // Update our physics.
        if let Ok((mut transform, mut velocity)) = flock_q.get_mut(*child) {
            let accel = sepration * swarm.separation + alignment * swarm.alignment + cohesion * swarm.cohesion;
            velocity.inner += accel;
            velocity.inner = velocity.inner.clamp_length_max(swarm.max_speed);
            transform.translation += velocity.inner.extend(0.0);
        };
    }
}

fn shooter_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut parent_q: Query<(&Children, &PlayerInput, &mut SwarmParent, &Faction)>,
    shooter_q: Query<(&GlobalTransform, &BasicShooter)>,
) {
    for (children, input, mut parent, faction) in parent_q.iter_mut() {
        if !input.shoot || children.is_empty() {
            continue;
        }

        let now = time.elapsed_seconds();
        let cooldown = parent.shooter_cooldown / children.len() as f32;
        if parent.last_fired_time > 0.0 && (now - parent.last_fired_time) < cooldown {
            continue;
        }

        // Pick a child at random to shoot from.
        // TODO: Make sure we pick a child that's a shooter.
        let entity = *fastrand::choice(children.iter()).unwrap();
        let Ok((transform, _shooter)) = shooter_q.get(entity) else {
            continue;
        };
        let pos = transform.translation().truncate() + Vec2::Y * 20.0;
        let vel = Vec2::Y * 1000.0;
        commands.spawn(ProjectileBundle::new(pos, vel, 1.0, *faction));

        parent.last_fired_time = now;
    }
}
