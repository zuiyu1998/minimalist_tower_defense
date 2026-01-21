use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct AttackDistance;

pub fn spawn_attack_distance(
    commmads: &mut Commands,
    circle: f32,
    collision_layers: CollisionLayers,
) -> Entity {
    commmads
        .spawn((
            Collider::circle(circle),
            Sensor,
            AttackDistance,
            collision_layers,
            CollisionEventsEnabled,
        ))
        .id()
}
