use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct AttackDistance;

pub fn spawn_attack_distance(
    commmads: &mut EntityCommands,
    circle: f32,
    collision_layers: CollisionLayers,
) {
    commmads.with_child((
        Collider::circle(circle),
        Sensor,
        AttackDistance,
        collision_layers,
        CollisionEventsEnabled,
    ));
}
