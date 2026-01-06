mod item_data_factory;
mod tile;

pub use item_data_factory::*;
pub use tile::*;

use std::fmt::Debug;

use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct MapItemData {
    name: String,
    x: i32,
    y: i32,
    position: Vec2,
}

#[derive(Debug, Resource)]
pub struct MapData {
    items: Vec<MapItemData>,
    item_size: i32,
    item_space_size: i32,
}

impl Default for MapData {
    fn default() -> Self {
        let mut items = vec![];

        for i in -2..2 {
            items.push(MapItemData {
                name: "hill".to_string(),
                x: i,
                y: 2,
                position: Vec2::ZERO,
            });
        }

        for i in -2..2 {
            items.push(MapItemData {
                name: "hill".to_string(),
                x: i,
                y: -2,
                position: Vec2::ZERO,
            });
        }

        MapData {
            items,
            item_size: 128,
            item_space_size: 2,
        }
    }
}

#[derive(Debug, Component)]
pub struct Map;

fn get_item_position(x: i32, y: i32, item_size: i32, item_space_size: i32) -> Vec2 {
    let x = x * (item_size + item_space_size);
    let y = y * (item_size + item_space_size);

    Vec2 {
        x: x as f32,
        y: y as f32,
    }
}

pub fn spawn_map(
    command: &mut Commands,
    asset_server: &AssetServer,
    map_data: &MapData,
    container: &MapItemFactoryContainer,
) {
    let mut commands = command.spawn((Map, Name::new("Map"), Transform::default()));

    for item in map_data.items.iter() {
        if let Some(factory) = container.get_map_item_factory(&item.name) {
            let mut item = item.clone();

            item.position =
                get_item_position(item.x, item.y, map_data.item_size, map_data.item_space_size);

            factory.spawn_map_item(&mut commands, asset_server, &item);
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MapData>();
    app.init_resource::<MapItemFactoryContainer>();
}
