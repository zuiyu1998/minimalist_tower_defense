mod item_data_factory;
mod tile;

pub use item_data_factory::*;
pub use tile::*;

use std::fmt::Debug;

use bevy::prelude::*;

use crate::{
    MainCamera,
    screens::Screen,
    unit::{UnitData, UnitFactoryContainer},
};

#[derive(Debug, Clone, Default)]
pub struct MapItemData {
    name: String,
    unit_name: String,
    x: i32,
    y: i32,
}

impl MapItemData {
    pub fn get_unit_data(&self) -> UnitData {
        UnitData {
            item_name: self.unit_name.clone(),
        }
    }
}

#[derive(Debug, Resource)]
pub struct MapData {
    items: Vec<MapItemData>,
    pub item_size: i32,
    pub item_space_size: i32,
}

#[derive(Debug, Resource, Default)]
pub struct MapState {
    pub selelcted_map_item_data: Option<MapItemData>,
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
            name: "square".to_string(),
            x: 0,
            y: 0,
            ..default()
        });

        MapData {
            items,
            item_size: 128,
            item_space_size: 2,
        }
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
    map_data: Res<MapData>,
    container: Res<MapItemFactoryContainer>,
    map: Single<Entity, With<Map>>,
    map_positon: Single<&MapPosition>,
    map_state: Res<MapState>,
    unit_factory_container: Res<UnitFactoryContainer>,
) {
    if map_state.selelcted_map_item_data.is_some()
        && mouse_button_input.just_pressed(MouseButton::Left)
    {
        let mut map_item_data = map_state.selelcted_map_item_data.clone().unwrap();
        map_item_data.x = map_positon.x;
        map_item_data.y = map_positon.y;

        let position = get_item_position(
            map_item_data.x,
            map_item_data.y,
            map_data.item_size,
            map_data.item_space_size,
        )
        .extend(0.0);

        let mut commands = commands.entity(*map);

        container.spawn_map_item(
            &mut commands,
            &asset_server,
            &map_item_data,
            position,
            &unit_factory_container,
        );
    }
}

fn update_map_position(
    mut map_position: Single<(&mut Transform, &mut MapPosition)>,
    mut cursor_moved_reader: MessageReader<CursorMoved>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    map_data: Res<MapData>,
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
        global_position.x + (map_data.item_size / 2) as f32,
        global_position.y + (map_data.item_size / 2) as f32,
    );

    let position_i = get_item_position_i(
        global_position.x,
        global_position.y,
        map_data.item_size,
        map_data.item_space_size,
    );

    let position = get_item_position(
        position_i.x,
        position_i.y,
        map_data.item_size,
        map_data.item_space_size,
    );

    map_position.0.translation.x = position.x;
    map_position.0.translation.y = position.y;

    map_position.1.x = position_i.x;
    map_position.1.y = position_i.y;
}

#[derive(Debug, Component)]
pub struct Map;

fn get_item_position_i(x: f32, y: f32, item_size: i32, item_space_size: i32) -> IVec2 {
    let x = (x / (item_size + item_space_size) as f32).floor() as i32;
    let y = (y / (item_size + item_space_size) as f32).floor() as i32;

    IVec2 { x, y }
}

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
    unit_factory_container: &UnitFactoryContainer,
) {
    let mut commands = command.spawn((
        Map,
        Name::new("Map"),
        Transform::default(),
        DespawnOnExit(Screen::Gameplay),
        Visibility::Visible,
    ));

    for item in map_data.items.iter() {
        let item = item.clone();
        let position =
            get_item_position(item.x, item.y, map_data.item_size, map_data.item_space_size)
                .extend(0.0);

        container.spawn_map_item(
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
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
        Name::new("MapPosition"),
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
    ));
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MapData>();
    app.init_resource::<MapItemFactoryContainer>();
    app.init_resource::<MapState>();

    app.add_systems(
        Update,
        (update_map_position, on_spawn_unit).run_if(in_state(Screen::Gameplay)),
    );
}
