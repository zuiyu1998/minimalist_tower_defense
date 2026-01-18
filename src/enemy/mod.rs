use std::fmt::Debug;

use crate::common::{GameLayer, Stas, spawn_hurt};
use avian2d::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Debug)]
pub struct EnemySpawnerContainer(HashMap<String, Box<dyn EnemySpawner>>);

impl Default for EnemySpawnerContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl EnemySpawnerContainer {
    pub fn new() -> Self {
        let mut container = Self::empty();

        container.register("square", SquareEnemySpawner);

        container
    }

    pub fn empty() -> Self {
        Self(Default::default())
    }

    pub fn register<T: EnemySpawner>(&mut self, name: &str, spawner: T) {
        self.0.insert(name.to_string(), Box::new(spawner));
    }

    pub fn spawn_enemy(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        position: Vec3,
        enemy: &str,
    ) {
        if let Some(spawner) = self.0.get(enemy) {
            spawner.spawn_enemy(commands, asset_server, position);
        } else {
            tracing::error!("{} enemy spawner not match.", enemy);
        }
    }
}

pub trait EnemySpawner: 'static + Debug + Send + Sync {
    fn spawn_enemy(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        position: Vec3,
    );
}

#[derive(Debug)]
pub struct SquareEnemySpawner;

impl EnemySpawner for SquareEnemySpawner {
    fn spawn_enemy(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        position: Vec3,
    ) {
        let image = asset_server.load("images/enemy/square.png");

        let collider = Collider::rectangle(80.0, 80.0);

        let parent = commands.id();

        let mut commands = commands.commands();

        let mut entity_commands = commands.spawn((
            Enemy,
            Square,
            Sprite {
                image,
                custom_size: Some(Vec2::splat(80.0)),
                ..default()
            },
            RigidBody::Kinematic,
            collider.clone(),
            LinearVelocity(Vec2::new(-10.0, 10.0)),
            GameLayer::enemy_layers(),
            Transform {
                translation: position,
                ..default()
            },
            Stas::default(),
            Name::new("Square"),
        ));

        let enemy = entity_commands.id();

        spawn_hurt(
            &mut entity_commands,
            collider,
            GameLayer::enemy_hurtbox_layers(),
        );

        commands.entity(parent).add_child(enemy);
    }
}

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Debug, Component)]
pub struct Square;

pub(super) fn plugin(_app: &mut App) {}
