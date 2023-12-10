use bevy::prelude::*;

use crate::{
    game::{
        enemies::Enemy,
        factions::Faction,
        health::Health,
    },
    physics::{self, groups, ActiveCollisionTypes, ActiveEvents, CollisionEvent, Group},
};

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
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    // mut hits: EventWriter<HitEvent>,
    // mut player_hits: EventWriter<PlayerHitEvent>,
    parent_q: Query<&Parent>,
    // rigid_body_q: Query<&RigidBody>,
    hit_box_q: Query<(&HitSpec, &Faction)>,
    // hurt_box_q: Query<(), With<HurtBox>>,
    // player_q: Query<(Entity, &PlayerHealth), With<Player>>,
    mut health_q: Query<(&mut Health, &Faction)>,
    name_q: Query<&Name>,
    // groups_q: Query<&CollisionGroups>,
) {
    // let (player_entity, health) = player_q.single();

    // Listen for collision events involving a hit box and a hurt box and send a hit event.
    for collision in collisions.read() {
        // info!("Collision event: {:?}", collision);
        if let &CollisionEvent::Started(e1, e2, _flags) = collision {
            if let (Ok((hit_spec, faction1)), Ok((mut health, faction2))) = (hit_box_q.get(e1), health_q.get_mut(e2)) {
                if faction1 == faction2 {
                    continue;
                }

                let lost = health.lose_health(hit_spec.damage);
                let name = name_q.get(e2)
                    .map(|name| name.as_str())
                    .unwrap_or("[unnamed]");
                debug!("Entity {} lost {} health", name, lost);
                if health.current() == 0.0 {
                    commands.entity(e2).despawn();
                    debug!("Entity {} died!", name);
                }
            } else if let (Ok((hit_spec, faction2)), Ok((mut health, faction1))) = (hit_box_q.get(e2), health_q.get_mut(e1)) {
                if faction1 == faction2 {
                    continue;
                }

                let lost = health.lose_health(hit_spec.damage);
                let name = name_q.get(e1)
                    .map(|name| name.as_str())
                    .unwrap_or("[unnamed]");
                debug!("Entity {} lost {} health", name, lost);
                if health.current() == 0.0 {
                    commands.entity(e1).despawn();
                    debug!("Entity {} died!", name);
                }
            }
        }
    }
}
