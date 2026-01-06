pub mod arrow_tower;

use std::time::Duration;

use crate::{common::{GameLayer, spawn_attack_distance}, skill::Skill};
use avian2d::prelude::*;
use bevy::prelude::*;

use arrow_tower::ArrowTower;

pub fn spawn_unit(commands: &mut Commands, asset_server: &AssetServer) {
    let image = asset_server.load("images/unit/TemporaryArrowTower.png");

    let position = Vec3::new(0.0, 0.0, 0.0);

    let unit_layers = GameLayer::unit_layers();

    let mut commands = commands.spawn((
        Unit,
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
        ArrowTower,
        EnemyTargets::default(),
        CooldownTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)),
        Skill {},
    ));

    let unit_attack_distance_layers = GameLayer::unit_attack_distance_layers();

    spawn_attack_distance(&mut commands, 500.0, unit_attack_distance_layers);
}

#[derive(Debug, Component, Default)]
pub struct EnemyTargets(Vec<Entity>);

#[derive(Debug, Component)]
pub struct CooldownTimer(Timer);

#[derive(Debug, Component)]
pub struct Unit;

pub(super) fn plugin(app: &mut App) {
    arrow_tower::plugin(app);
}
