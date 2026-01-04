use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Hitbox;

pub fn spawn_hit(commands: &mut EntityCommands, collider: Collider, layers: CollisionLayers) {
    commands.with_child((Hitbox, collider, layers, CollisionEventsEnabled, Sensor));
}

#[derive(Debug, Component)]
pub struct Hurtbox;

pub fn spawn_hurt(commands: &mut EntityCommands, collider: Collider, layers: CollisionLayers) {
    commands.with_child((Hurtbox, collider, layers));
}
