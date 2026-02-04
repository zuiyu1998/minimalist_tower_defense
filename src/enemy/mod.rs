use std::fmt::Debug;

use crate::{
    common::{GameLayer, LightSource, Stas, spawn_hurt},
    navigator::NavigatorPath,
};
use avian2d::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*};
use vleue_navigator::{NavMesh, prelude::ManagedNavMesh};

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
            LinearVelocity(Vec2::new(0.0, 0.0)),
            GameLayer::enemy_layers(),
            Transform {
                translation: position,
                ..default()
            },
            Stas::default(),
            Name::new("Square"),
            SleepingDisabled,
            NavigatorPath::default(),
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

//添加导航
fn find_navigator_path(
    mut query: Query<(Ref<GlobalTransform>, &mut NavigatorPath)>,
    navmeshes: Res<Assets<NavMesh>>,
    navmesh: Single<&ManagedNavMesh>,
    light_source_q: Query<Ref<GlobalTransform>, With<LightSource>>,
) {
    if light_source_q.is_empty() {
        return;
    }

    let Some(navmesh) = navmeshes.get(*navmesh) else {
        return;
    };

    let light_sources: Vec<GlobalTransform> =
        light_source_q.iter().map(|item| item.clone()).collect();
    let light_source = light_sources.first().unwrap();

    for (transform, mut navigator_path) in query.iter_mut() {
        let Some(path) =
            navmesh.transformed_path(transform.translation(), light_source.translation())
        else {
            continue;
        };

        if let Some((first, remaining)) = path.path.split_first() {
            let mut remaining = remaining.to_vec();
            remaining.reverse();

            navigator_path.current = *first;
            navigator_path.next = remaining.to_vec();
        }
    }
}

pub fn move_enemy(
    mut navigator: Query<(
        &GlobalTransform,
        &NavigatorPath,
        Entity,
        &mut LinearVelocity,
        &Enemy,
    )>,
) {
    for (transform, path, _entity, mut linvel, _enemy) in navigator.iter_mut() {
        let move_direction = path.current - transform.translation();
        linvel.0 = move_direction.truncate().normalize() * 50.0;

        if transform.translation().distance(path.current) < 50.0 && path.next.is_empty() {
            linvel.0 = Vec2::ZERO;
            continue;
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, (find_navigator_path, move_enemy));
}
