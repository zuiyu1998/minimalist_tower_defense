use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Hit;

pub fn spawn_hit(commands: &mut EntityCommands, collider: Collider, layers: CollisionLayers) {
    commands.with_child((Hit, collider, layers, CollisionEventsEnabled));
}
