use bevy::prelude::*;

use crate::{
    map::{MapData, MapItemFactoryContainer, spawn_map},
    player::Player,
};


pub(super) fn plugin(_app: &mut App) {}

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_data: Res<MapData>,
    container: Res<MapItemFactoryContainer>,
) {
    commands.spawn(Player);

    spawn_map(&mut commands, &asset_server, &map_data, &container);
}
