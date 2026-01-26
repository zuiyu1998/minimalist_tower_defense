mod environment;
mod item_data_factory;
mod lair;
mod tile;

pub use environment::*;
pub use item_data_factory::*;
pub use tile::*;

use std::fmt::Debug;

use bevy::prelude::*;

use crate::{
    MainCamera,
    consts::{
        MAP_ITEM_SELECTED_SIZE, MAP_LAYER, MAP_TIEM_SIZE, MAP_TILE_LAYER, MAP_TILE_SELECTED_LAYER,
    },
    map::lair::spawn_lair,
    screens::Screen,
    unit::{UnitData, UnitFactoryContainer},
};

#[derive(Debug, Clone, Default)]
pub struct MapItemData {
    name: String,
    unit_item_name: String,
    unit_image: String,
    x: i32,
    y: i32,
}

impl MapItemData {
    pub fn get_unit_data(&self) -> UnitData {
        UnitData {
            item_name: self.unit_item_name.clone(),
            image: self.unit_image.clone(),
        }
    }

    pub fn from_unit_data(unit_data: &UnitData) -> Self {
        Self {
            name: "unit".to_string(),
            unit_item_name: unit_data.item_name.to_string(),
            unit_image: unit_data.image.to_string(),
            ..default()
        }
    }
}

#[derive(Debug, Resource)]
pub struct MapData {
    items: Vec<MapItemData>,
}

#[derive(Debug, Resource, Default)]
pub struct MapState {
    pub selelcted_map_item_data: Option<MapItemData>,
    pub enable: bool,
}

impl Default for MapData {
    fn default() -> Self {
        let mut items = vec![];

        for i in -2..2 {
            items.push(MapItemData {
                name: "hill".to_string(),
                x: i,
                y: 2,
                ..default()
            });
        }

        for i in -2..2 {
            items.push(MapItemData {
                name: "hill".to_string(),
                x: i,
                y: -2,
                ..default()
            });
        }

        items.push(MapItemData {
            name: "unit".to_string(),
            unit_item_name: "bonfire".to_string(),
            unit_image: "TemporaryArrowTower".to_string(),
            x: 0,
            y: 0,
            ..default()
        });

        MapData { items }
    }
}

#[derive(Debug, Component, Default)]
pub struct MapPosition {
    x: i32,
    y: i32,
}

fn on_spawn_unit(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Single<(Entity, &Map)>,
    map_positon: Single<&MapPosition>,
    mut map_state: ResMut<MapState>,
    unit_factory_container: Res<UnitFactoryContainer>,
) {
    if map_state.enable
        && map_state.selelcted_map_item_data.is_some()
        && mouse_button_input.just_pressed(MouseButton::Left)
    {
        let mut map_item_data = map_state.selelcted_map_item_data.clone().unwrap();
        map_item_data.x = map_positon.x;
        map_item_data.y = map_positon.y;

        let position = get_item_position(map_item_data.x, map_item_data.y).extend(0.0);

        let (map_entity, map) = map.into_inner();

        let mut commands = commands.entity(map_entity);

        map.item_factory_container.spawn_map_item(
            &mut commands,
            &asset_server,
            &map_item_data,
            position,
            &unit_factory_container,
        );

        map_state.enable = false;
        map_state.selelcted_map_item_data = None;
    }
}

fn update_map_position(
    mut map_position: Single<(&mut Transform, &mut MapPosition)>,
    mut cursor_moved_reader: MessageReader<CursorMoved>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let mut event = None;

    for cursor_moved in cursor_moved_reader.read() {
        event = Some(cursor_moved.clone());
    }

    let Some(event) = event else {
        return;
    };

    let Ok(global_position) = camera.0.viewport_to_world_2d(camera.1, event.position) else {
        return;
    };

    let global_position = Vec2::new(
        global_position.x + (MAP_TIEM_SIZE / 2.0) as f32,
        global_position.y + (MAP_TIEM_SIZE / 2.0) as f32,
    );

    let position_i = get_item_position_i(global_position.x, global_position.y);

    let position = get_item_position(position_i.x, position_i.y);

    map_position.0.translation.x = position.x;
    map_position.0.translation.y = position.y;

    map_position.1.x = position_i.x;
    map_position.1.y = position_i.y;
}

#[derive(Debug, Component)]
pub struct Map {
    item_factory_container: MapItemFactoryContainer,
    x: i32,
    y: i32,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            item_factory_container: Default::default(),
            x: 25,
            y: 25,
        }
    }
}

impl Map {
    pub fn get_map_size(&self) -> Vec2 {
        let x = self.x as f32 * MAP_TIEM_SIZE;
        let y = self.y as f32 * MAP_TIEM_SIZE;

        Vec2 { x, y }
    }
}

fn get_item_position_i(x: f32, y: f32) -> IVec2 {
    let x = ((x - 1.0) / MAP_TIEM_SIZE).floor() as i32;
    let y = (y / MAP_TIEM_SIZE).floor() as i32;

    IVec2 { x, y }
}

fn get_item_position(x: i32, y: i32) -> Vec2 {
    let x = 1.0 + x as f32 * MAP_TIEM_SIZE;
    let y = -1.0 + y as f32 * MAP_TIEM_SIZE;

    Vec2 {
        x: x as f32,
        y: y as f32,
    }
}

pub fn spawn_map(
    command: &mut Commands,
    asset_server: &AssetServer,
    map_data: &MapData,
    unit_factory_container: &UnitFactoryContainer,
) {
    let map = Map::default();

    let image = asset_server.load("images/map/bg.png");

    let mut commands = command.spawn((
        Name::new("Map"),
        DespawnOnExit(Screen::Gameplay),
        Visibility::Visible,
        MapEnvironment::default(),
        Sprite {
            image: image,
            custom_size: Some(map.get_map_size()),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: MAP_LAYER,
            },
            ..default()
        },
    ));

    for item in map_data.items.iter() {
        let item = item.clone();
        let position = get_item_position(item.x, item.y).extend(MAP_TILE_LAYER);

        map.item_factory_container.spawn_map_item(
            &mut commands,
            asset_server,
            &item,
            position,
            unit_factory_container,
        );
    }

    let image = asset_server.load("images/map/ButtonSelectLine3.png");

    commands.with_child((
        MapPosition::default(),
        Sprite {
            image,
            custom_size: Some(Vec2::splat(MAP_ITEM_SELECTED_SIZE)),
            ..default()
        },
        Name::new("MapPosition"),
        Transform {
            translation: Vec3::new(0.0, 0.0, MAP_TILE_SELECTED_LAYER),
            ..default()
        },
    ));

    let lair = {
        let mut commands = commands.commands();
        spawn_lair(&mut commands)
    };

    commands.add_child(lair);

    commands.insert(map);
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MapData>();
    app.init_resource::<MapState>();

    app.add_plugins(lair::plugin);

    app.add_systems(
        Update,
        (update_map_position, on_spawn_unit).run_if(in_state(Screen::Gameplay)),
    );
}
