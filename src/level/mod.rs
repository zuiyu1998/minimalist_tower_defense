use bevy::prelude::*;

use crate::{
    map::{MapData, MapItemFactoryContainer, spawn_map},
    player::Player,
};

pub(super) fn plugin(_app: &mut App) {}

pub fn spawn_level(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    map_data: Res<MapData>,
    container: Res<MapItemFactoryContainer>,
) {
    commands.spawn(Player);

    spawn_map(&mut commands, &map_data, &container);
}
