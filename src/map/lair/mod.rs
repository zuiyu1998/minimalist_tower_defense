use bevy::prelude::*;

#[derive(Debug, Component)]
#[require(Transform)]
pub struct Lair;

pub fn spawn_lair(commands: &mut Commands) -> Entity {
    commands.spawn((Lair, Name::new("Lair"))).id()
}


pub(super) fn plugin(_app: &mut App) {}
