use bevy::prelude::*;

use crate::{enemy::spawn_enemy, player::Player, unit::spawn_unit};

pub(super) fn plugin(_app: &mut App) {}

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Player);

    spawn_enemy(&mut commands, &asset_server);
    spawn_unit(&mut commands, &asset_server);
}
