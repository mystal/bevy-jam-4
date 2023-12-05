use bevy::prelude::*;

#[derive(Clone, Copy, Default, Component)]
pub struct Velocity {
    pub inner: Vec2,
}
