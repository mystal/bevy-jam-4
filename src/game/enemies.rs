use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    game::{
        combat::HurtBoxBundle,
        health::Health,
    },
    physics::groups,
};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Component)]
pub struct Enemy {
}

#[derive(Bundle)]
pub struct EnemyBundle {
    name: Name,
    enemy: Enemy,
    health: Health,
    hurt_box: HurtBoxBundle,
    shape: ShapeBundle,
    fill: Fill,
}

impl EnemyBundle {
    pub fn new(pos: Vec2) -> Self {
        let shape = shapes::RegularPolygon {
            sides: 4,
            feature: RegularPolygonFeature::Radius(20.0),
            ..default()
        };
        let size = Vec2::splat(40.0);
        let transform = Transform::from_translation(pos.extend(0.0));
        Self {
            name: Name::new("Enemy"),
            enemy: Enemy {
            },
            health: Health::new(1.0),
            hurt_box: HurtBoxBundle::rect(size, groups::ENEMY),
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(transform),
                ..default()
            },
            fill: Fill::color(Color::ORANGE * 4.0),
        }
    }
}
