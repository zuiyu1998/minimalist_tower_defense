pub mod arrow_tower;

use std::{fmt::Debug, time::Duration};

use crate::{
    common::{GameLayer, spawn_attack_distance},
    skill::Skill,
    unit::arrow_tower::ArrowTowerFactory,
};
use avian2d::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*};

pub fn spawn_unit(
    commands: &mut EntityCommands,
    asset_server: &AssetServer,
    position: Vec3,
    data: &UnitData,
    container: &UnitFactoryContainer,
) {
    if let Some(factory) = container.0.get(&data.item_name) {
        let unit = Unit::from_data(data);
        unit.spawn_unit(commands, asset_server, position, data, factory.as_ref());
    } else {
        tracing::error!("{} factory not match.", data.item_name);
    }
}

#[derive(Debug, Resource)]
pub struct UnitFactoryContainer(HashMap<String, Box<dyn UnitFactory>>);

impl Default for UnitFactoryContainer {
    fn default() -> Self {
        let mut container = UnitFactoryContainer::empty();
        container.register("arrow_tower", ArrowTowerFactory);

        container
    }
}

impl UnitFactoryContainer {
    pub fn register<T: UnitFactory>(&mut self, name: &str, value: T) {
        self.0.insert(name.to_string(), Box::new(value));
    }

    pub fn empty() -> Self {
        UnitFactoryContainer(Default::default())
    }
}

#[derive(Debug, Clone)]
pub struct UnitData {
    pub item_name: String,
    pub image: String,
}

impl UnitData {
    pub fn get_unit_image(&self, asset_server: &AssetServer) -> Handle<Image> {
        asset_server.load(&format!("images/unit/{}.png", self.image))
    }
}

pub trait UnitFactory: 'static + Send + Sync + Debug {
    fn spawn(&self, data: &UnitData, commands: &mut EntityCommands);
}

#[derive(Debug, Component, Default)]
pub struct EnemyTargets(Vec<Entity>);

#[derive(Debug, Component)]
pub struct CooldownTimer(Timer);

#[derive(Debug, Component, Clone, Default)]
pub struct Unit {}

impl Unit {
    pub fn from_data(_data: &UnitData) -> Self {
        Unit {}
    }

    pub fn spawn_unit(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        position: Vec3,
        data: &UnitData,
        factory: &dyn UnitFactory,
    ) {
        let image = data.get_unit_image(asset_server);

        let unit_layers = GameLayer::unit_layers();

        let parent = commands.id();

        let mut commands = commands.commands();

        let mut entity_commands = commands.spawn((
            self.clone(),
            Sprite {
                image,
                custom_size: Some(Vec2::splat(128.0)),
                ..default()
            },
            Transform {
                translation: position,
                ..default()
            },
            RigidBody::Static,
            Collider::rectangle(100.0, 100.0),
            unit_layers,
            EnemyTargets::default(),
            CooldownTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)),
            Skill {},
        ));

        factory.spawn(data, &mut entity_commands);

        let unit = entity_commands.id();

        let unit_attack_distance_layers = GameLayer::unit_attack_distance_layers();

        spawn_attack_distance(&mut entity_commands, 500.0, unit_attack_distance_layers);

        commands.entity(parent).add_child(unit);
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<UnitFactoryContainer>();

    arrow_tower::plugin(app);
}
