use bevy::prelude::*;

use crate::player::Player;

pub(super) fn plugin(_app: &mut App) {}

pub fn spawn_level(mut commands: Commands) {
    commands.spawn(Player);
}
