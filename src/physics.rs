use bevy::prelude::*;

use crate::game::input::PlayerInput;

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
