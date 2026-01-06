use bevy::prelude::*;

use super::MapItemData;

#[derive(Debug, Component)]
pub struct Hill;

pub fn spawn_hill_map_item(
    commands: &mut EntityCommands,
    asset_server: &AssetServer,
    map_item_data: &MapItemData,
) {
    let image = asset_server.load("images/map/Hill.png");

    commands.with_child((
        Hill,
        Sprite {
            image,
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(map_item_data.position.x, map_item_data.position.y, 0.0),
            ..default()
        },
    ));
}
