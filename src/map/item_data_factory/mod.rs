use std::fmt::Debug;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    enemy::EnemySpawnerContainer,
    map::spawn_hill_map_item,
    unit::{UnitFactoryContainer, spawn_unit},
};

use super::MapItemData;

#[derive(Debug, Resource)]
pub struct MapItemFactoryContainer(HashMap<String, Box<dyn MapItemFactory>>);

impl MapItemFactoryContainer {
    pub fn new() -> Self {
        let mut container = MapItemFactoryContainer::empty();
        container.register(HillMapItemFactory);
        container.register(UnitMapItemFactory);
        container.register(EnemyMapItemFactory::default());

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
        position: Vec3,
        unit_factory_container: &UnitFactoryContainer,
    ) {
        if let Some(factory) = self.get_map_item_factory(&item_data.name) {
            factory.spawn_map_item(
                commands,
                asset_server,
                &item_data,
                position,
                unit_factory_container,
            );
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
        position: Vec3,
        unit_factory_container: &UnitFactoryContainer,
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
        position: Vec3,
        _unit_factory_container: &UnitFactoryContainer,
    ) {
        spawn_hill_map_item(commands, asset_server, item_data, position);
    }
}

#[derive(Debug)]
pub struct UnitMapItemFactory;

impl MapItemFactory for UnitMapItemFactory {
    fn map_item_name(&self) -> &'static str {
        "unit"
    }

    fn spawn_map_item(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        item_data: &MapItemData,
        position: Vec3,
        unit_factory_container: &UnitFactoryContainer,
    ) {
        let unit_data = item_data.get_unit_data();
        spawn_unit(
            commands,
            asset_server,
            position,
            &unit_data,
            unit_factory_container,
        );
    }
}

#[derive(Debug, Default)]
pub struct EnemyMapItemFactory {
    data: EnemySpawnerContainer,
}

impl MapItemFactory for EnemyMapItemFactory {
    fn map_item_name(&self) -> &'static str {
        "enemy"
    }

    fn spawn_map_item(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        item_data: &MapItemData,
        position: Vec3,
        _unit_factory_container: &UnitFactoryContainer,
    ) {
       self.data.spawn_enemy(commands, asset_server, position, &item_data.enemy_item_name);
    }
}
