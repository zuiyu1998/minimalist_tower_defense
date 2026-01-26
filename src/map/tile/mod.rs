use bevy::prelude::*;

use crate::consts::MAP_ITEM_CONTENT_SIZE;

use super::MapItemData;

#[derive(Debug, Component)]
pub struct Hill;

pub fn spawn_hill_map_item(
    commands: &mut EntityCommands,
    asset_server: &AssetServer,
    _map_item_data: &MapItemData,
    position: Vec3,
) {
    let image = asset_server.load("images/map/Hill.png");

    commands.with_child((
        Hill,
        Sprite {
            image,
            custom_size: Some(Vec2::splat(MAP_ITEM_CONTENT_SIZE)),
            ..default()
        },
        Transform {
            translation: position,
            ..default()
        },
    ));
}
