use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub use bevy_rapier2d::{
    prelude::{ActiveEvents, ActiveCollisionTypes, CollisionEvent},
    geometry::Group,
};

use crate::game::input::PlayerInput;

pub mod groups {
    use bevy_rapier2d::geometry::Group;

    pub const WORLD: Group = Group::GROUP_1;
    pub const HIT: Group = Group::GROUP_2;
    pub const HURT: Group = Group::GROUP_3;
    pub const PLAYER: Group = Group::GROUP_4;
    pub const ENEMY: Group = Group::GROUP_5;

    pub const ALL: Group = Group::ALL;
    pub const NONE: Group = Group::NONE;
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_movement);
    }
}

#[derive(Clone, Copy, Default, Component)]
pub struct Velocity {
    pub inner: Vec2,
}

impl Velocity {
    pub fn new(vel: Vec2) -> Self {
        Self {
            inner: vel,
        }
    }
}

#[derive(Component)]
pub struct PlayerMovement {
    speed: f32,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            speed: 600.0,
        }
    }
}

fn update_movement(
    time: Res<Time>,
    mut q: Query<(&PlayerInput, &PlayerMovement, &mut Transform)>,
) {
    for (input, movement, mut transform) in q.iter_mut() {
        transform.translation += input.movement.extend(0.0) * time.delta_seconds() * movement.speed;
    }
}

#[derive(Bundle)]
pub struct ColliderBundle {
    shape: Collider,
    layers: CollisionGroups,
    sensor: Sensor,
}

impl ColliderBundle {
    pub fn circle(radius: f32, memberships: Group, filters: Group) -> Self {
        Self {
            shape: Collider::ball(radius),
            layers: CollisionGroups::new(memberships, filters),
            sensor: Sensor,
        }
    }

    pub fn rect(size: Vec2, memberships: Group, filters: Group) -> Self {
        let half_extents = size / 2.0;
        Self {
            shape: Collider::cuboid(half_extents.x, half_extents.y),
            layers: CollisionGroups::new(memberships, filters),
            sensor: Sensor,
        }
    }
}

