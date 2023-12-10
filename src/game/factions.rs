use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Component)]
pub enum Faction {
    Player,
    Enemy,
}
