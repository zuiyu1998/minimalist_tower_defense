mod arrow_tower;
mod bonfire;

pub use arrow_tower::*;
pub use bonfire::*;

use std::{fmt::Debug, time::Duration};

use crate::{common::GameLayer, skill::Skill};
use avian2d::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*};

//更新技能冷却
fn on_cooldown_timer_update(
    mut commands: Commands,
    mut cooldown_timer_q: Query<(&mut CooldownTimer, &mut Unit, Entity)>,
    time: Res<Time>,
) {
    for (mut cooldown_timer, mut unit, entity) in cooldown_timer_q.iter_mut() {
        cooldown_timer.0.tick(time.delta());

        if cooldown_timer.0.just_finished() {
            unit.cooling_down = false;

            commands.entity(entity).remove::<CooldownTimer>();
        }
    }
}

fn add_cooldown_timer(
    mut commands: Commands,
    mut cooldown_timer_q: Query<(&mut Unit, Entity), Without<CooldownTimer>>,
) {
    for (mut unit, entity) in cooldown_timer_q.iter_mut() {
        if !unit.cooling_down {
            unit.cooling_down = true;

            commands
                .entity(entity)
                .insert(CooldownTimer::new(unit.cooldown_timer));
        }
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
    fn spawn(&self, data: &UnitData, entity_commands: &mut EntityCommands);
}

#[derive(Debug, Component, Default)]
pub struct EnemyTargets(Vec<Entity>);

#[derive(Debug, Component)]
pub struct CooldownTimer(Timer);

impl CooldownTimer {
    pub fn new(secs: u64) -> Self {
        CooldownTimer(Timer::new(Duration::from_secs(secs), TimerMode::Once))
    }
}

#[derive(Debug, Component, Clone, Default)]
pub struct Unit {
    cooling_down: bool,
    cooldown_timer: u64,
}

impl Unit {
    pub fn from_data(_data: &UnitData) -> Self {
        Unit {
            cooling_down: false,
            cooldown_timer: 1,
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
            CooldownTimer::new(self.cooldown_timer),
            Skill {},
        ));

        factory.spawn(data, &mut entity_commands);
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<UnitFactoryContainer>();
    app.add_systems(Update, (on_cooldown_timer_update, add_cooldown_timer));

    arrow_tower::plugin(app);
    bonfire::plugin(app);
}
