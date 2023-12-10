use bevy::prelude::*;

use crate::physics::{self, groups, ActiveCollisionTypes, ActiveEvents, CollisionEvent, Group};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, check_hits);
    }
}

#[derive(Component)]
pub struct HitSpec {
    pub damage: f32,
}

impl HitSpec {
}

#[derive(Bundle)]
pub struct HitBoxBundle {
    hit_spec: HitSpec,
    collider: physics::ColliderBundle,
    active_events: ActiveEvents,
    collision_types: ActiveCollisionTypes,
}

impl HitBoxBundle {
    // TODO: Make with_offset, with_damage, with_knockback, and with_layers methods.
    pub fn circle(radius: f32, damage: f32, extra_memberships: Group) -> Self {
        let memberships = groups::HIT | extra_memberships;
        let filters = groups::HURT;
        Self {
            hit_spec: HitSpec {
                damage,
            },
            collider: physics::ColliderBundle::circle(radius, memberships, filters),
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC,
        }
    }

    pub fn rect(size: Vec2, damage: f32, extra_memberships: Group) -> Self {
        let memberships = groups::HIT | extra_memberships;
        let filters = groups::HURT;
        Self {
            hit_spec: HitSpec {
                damage,
            },
            collider: physics::ColliderBundle::rect(size, memberships, filters),
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC,
        }
    }
}

#[derive(Bundle)]
pub struct HurtBoxBundle {
    collider: physics::ColliderBundle,
    active_events: ActiveEvents,
    collision_types: ActiveCollisionTypes,
}

impl HurtBoxBundle {
    pub fn circle(radius: f32, extra_memberships: Group) -> Self {
        let memberships = groups::HURT | extra_memberships;
        let filters = groups::HIT;
        Self {
            collider: physics::ColliderBundle::circle(radius, memberships, filters),
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC,
        }
    }

    pub fn rect(size: Vec2, extra_memberships: Group) -> Self {
        let memberships = groups::HURT | extra_memberships;
        let filters = groups::HIT;
        Self {
            collider: physics::ColliderBundle::rect(size, memberships, filters),
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC,
        }
    }
}

pub fn check_hits(
    mut collisions: EventReader<CollisionEvent>,
    // mut hits: EventWriter<HitEvent>,
    // mut player_hits: EventWriter<PlayerHitEvent>,
    parent_q: Query<&Parent>,
    // rigid_body_q: Query<&RigidBody>,
    hit_box_q: Query<&HitSpec>,
    // hurt_box_q: Query<(), With<HurtBox>>,
    // player_q: Query<(Entity, &PlayerHealth), With<Player>>,
    // enemy_q: Query<(), With<Enemy>>,
    name_q: Query<&Name>,
    // groups_q: Query<&CollisionGroups>,
) {
    // let (player_entity, health) = player_q.single();

    // Listen for collision events involving a hit box and a hurt box and send a hit event.
    for collision in collisions.read() {
        info!("Collision event: {:?}", collision);
    }
}
