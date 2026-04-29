mod arrow_tower;
mod bonfire;

pub use arrow_tower::*;
pub use bonfire::*;

use std::{fmt::Debug, time::Duration};

use crate::{
    asset_tracking::LoadResource,
    common::{EnemyTargets, GameLayer, Stas, spawn_hurt},
    skill::Skill,
};
use avian2d::prelude::*;
use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    ecs::system::SystemParam,
    platform::collections::HashMap,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(SystemParam)]
pub struct UnitSystemParams<'w> {
    pub unit_factory_container: Res<'w, UnitFactoryContainer>,
    unit_data_assets: Res<'w, UnitDataAssets>,
    unit_data_set: Res<'w, Assets<UnitData>>,
}

impl UnitSystemParams<'_> {
    pub fn get_unit_data(&self, item_name: &str) -> Option<UnitData> {
        self.unit_data_assets
            .assets
            .get(item_name)
            .and_then(|handle| self.unit_data_set.get(handle).cloned())
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct UnitDataAssets {
    assets: HashMap<String, Handle<UnitData>>,
}

impl FromWorld for UnitDataAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let mut assets = HashMap::new();

        assets.insert(
            "bonfire".into(),
            asset_server.load("unit/bonfire.unit_data.yaml"),
        );

        assets.insert(
            "arrow_tower".into(),
            asset_server.load("unit/arrow_tower.unit_data.yaml"),
        );

        Self { assets }
    }
}

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
pub struct IdleState;

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
pub struct EnableState;

//更新技能冷却
fn on_cooldown_timer_update(mut cooldown_timer_q: Query<(&mut CooldownTimer,)>, time: Res<Time>) {
    for (mut cooldown_timer,) in cooldown_timer_q.iter_mut() {
        cooldown_timer.timer.tick(time.delta());
    }
}

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
        container.register("bonfire", BonfireFactory);

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

#[derive(Default, TypePath)]
pub struct UnitDataLoader;

#[derive(Debug, Error)]
pub enum UnitDataLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse Yaml: {0}")]
    Yaml(#[from] yaml_serde::Error),
}

impl AssetLoader for UnitDataLoader {
    type Asset = UnitData;

    type Settings = ();

    type Error = UnitDataLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).await?;
        let asset: UnitData = yaml_serde::from_str(&buffer)?;
        Ok(asset)
    }
    fn extensions(&self) -> &[&str] {
        &[".unit_data.yaml"]
    }
}

#[derive(Debug, Clone, Asset, Reflect, Deserialize, Serialize)]
pub struct UnitData {
    pub item_name: String,
    pub image: String,
    //冷却倒计时，单位为秒
    pub cooldown_timer: u64,
}

impl UnitData {
    pub fn get_unit_image(&self, asset_server: &AssetServer) -> Handle<Image> {
        asset_server.load(&format!("images/unit/{}.png", self.image))
    }
}

pub trait UnitFactory: 'static + Send + Sync + Debug {
    fn spawn(&self, data: &UnitData, entity_commands: &mut EntityCommands);
}

#[derive(Debug, Component)]
pub struct FirstCreate;

#[derive(Debug, Component)]
pub struct CooldownTimer {
    pub timer: Timer,
}

impl CooldownTimer {
    pub fn new(secs: u64) -> Self {
        CooldownTimer {
            timer: Timer::new(Duration::from_secs(secs), TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Component, Clone, Default)]
pub struct Unit {
    cooldown_timer: u64,
}

impl Unit {
    pub fn from_data(data: &UnitData) -> Self {
        Unit {
            cooldown_timer: data.cooldown_timer,
        }
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

        let mut commands = commands.commands();
        let collider = Collider::rectangle(100.0, 100.0);

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
            collider.clone(),
            unit_layers,
            EnemyTargets::default(),
            CooldownTimer::new(self.cooldown_timer),
            Skill {},
            Stas::default(),
            FirstCreate,
        ));

        spawn_hurt(
            &mut entity_commands,
            collider,
            GameLayer::unit_hurtbox_layers(),
        );

        factory.spawn(data, &mut entity_commands);
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<UnitFactoryContainer>();
    app.init_asset_loader::<UnitDataLoader>();
    app.init_asset::<UnitData>();
    app.add_systems(Update, (on_cooldown_timer_update,));

    app.load_resource::<UnitDataAssets>();

    arrow_tower::plugin(app);
    bonfire::plugin(app);
}
