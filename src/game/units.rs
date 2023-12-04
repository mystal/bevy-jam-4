use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::game::input::PlayerInput;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Component)]
pub struct SwarmParent;

#[derive(Component)]
pub struct BasicShooter {
}

pub fn spawn_swarm(commands: &mut Commands) {
    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: RegularPolygonFeature::Radius(40.0),
        ..default()
    };

    commands.spawn((
        Name::new("SwarmParent"),
        SwarmParent,
        PlayerInput::default(),
        SpatialBundle::default(),
    )).with_children(|b| {
        b.spawn((
            Name::new("BasicShooter"),
            BasicShooter {},
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(Color::ORANGE * 5.0),
        ));
    });
}
