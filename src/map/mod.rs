use std::fmt::Debug;

use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Debug)]
pub struct MapItemData {
    name: String,
}

#[derive(Debug, Resource)]
pub struct MapItemFactoryContainer(HashMap<String, Box<dyn MapItemFactory>>);

impl MapItemFactoryContainer {
    pub fn new() -> Self {
        let container = MapItemFactoryContainer::empty();
        container
    }

    pub fn register<T: MapItemFactory>(&mut self, value: T) {
        self.0
            .insert(value.map_item_name().to_string(), Box::new(value));
    }

    pub fn get_map_item_factory(&self, name: &str) -> Option<&dyn MapItemFactory> {
        self.0.get(name).map(|v| &**v)
    }

    pub fn empty() -> Self {
        MapItemFactoryContainer(Default::default())
    }
}

impl Default for MapItemFactoryContainer {
    fn default() -> Self {
        MapItemFactoryContainer::new()
    }
}

pub trait MapItemFactory: 'static + Send + Sync + Debug {
    fn map_item_name(&self) -> &'static str;

    fn spawn_map_item(&self, commands: &mut EntityCommands, item_data: &MapItemData);
}

#[derive(Debug, Resource)]
pub struct MapData {
    items: Vec<MapItemData>,
}

impl Default for MapData {
    fn default() -> Self {
        MapData { items: vec![] }
    }
}

#[derive(Debug, Component)]
pub struct Map;

pub fn spawn_map(command: &mut Commands, map_data: &MapData, container: &MapItemFactoryContainer) {
    let mut commands = command.spawn((Map, Name::new("Map")));

    for item in map_data.items.iter() {
        if let Some(factory) = container.get_map_item_factory(&item.name) {
            factory.spawn_map_item(&mut commands, &item);
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MapData>();
    app.init_resource::<MapItemFactoryContainer>();
}
