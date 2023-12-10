use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

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
        let transform = Transform::from_translation(pos.extend(0.0));
        Self {
            name: Name::new("BasicShooter"),
            enemy: Enemy {
            },
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(transform),
                ..default()
            },
            fill: Fill::color(Color::ORANGE * 4.0),
        }
    }
}
