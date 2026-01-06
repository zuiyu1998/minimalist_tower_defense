use std::fmt::Debug;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{map::spawn_hill_map_item, unit::spawn_unit};

use super::MapItemData;

#[derive(Debug, Resource)]
pub struct MapItemFactoryContainer(HashMap<String, Box<dyn MapItemFactory>>);

impl MapItemFactoryContainer {
    pub fn new() -> Self {
        let mut container = MapItemFactoryContainer::empty();
        container.register(HillMapItemFactory);
        container.register(ArrowTowerMapItemFactory);

        container
    }

    pub fn register<T: MapItemFactory>(&mut self, value: T) {
        self.0
            .insert(value.map_item_name().to_string(), Box::new(value));
    }

    pub fn get_map_item_factory(&self, name: &str) -> Option<&dyn MapItemFactory> {
        self.0.get(name).map(|v| &**v)
    }

    pub fn spawn_map_item(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        item_data: &MapItemData,
    ) {
        if let Some(factory) = self.get_map_item_factory(&item_data.name) {
            factory.spawn_map_item(commands, asset_server, &item_data);
        }
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

    fn spawn_map_item(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        item_data: &MapItemData,
    );
}

#[derive(Debug)]
pub struct HillMapItemFactory;

impl MapItemFactory for HillMapItemFactory {
    fn map_item_name(&self) -> &'static str {
        "hill"
    }

    fn spawn_map_item(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        item_data: &MapItemData,
    ) {
        spawn_hill_map_item(commands, asset_server, item_data);
    }
}

#[derive(Debug)]
pub struct ArrowTowerMapItemFactory;

impl MapItemFactory for ArrowTowerMapItemFactory {
    fn map_item_name(&self) -> &'static str {
        "arrow_tower"
    }

    fn spawn_map_item(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        item_data: &MapItemData,
    ) {
        let position = Vec3::new(item_data.position.x, item_data.position.y, 0.0);
        spawn_unit(commands, asset_server, position, Name::new("ArrowTower"));
    }
}
