use bevy::prelude::*;

use crate::{enemy::spawn_enemy, player::Player};

pub(super) fn plugin(_app: &mut App) {}

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Player);

    spawn_enemy(&mut commands, &asset_server);
}
